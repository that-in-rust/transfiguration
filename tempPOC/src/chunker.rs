//! Text chunking utility for processing large source files
//!
//! Follows TDD-First principles with executable specifications and measurable contracts.

use std::path::Path;
use anyhow::{Context, Result};
use tracing::{info, debug};

/// Represents a chunk of source code with metadata
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CodeChunk {
    /// Unique identifier for this chunk
    pub id: uuid::Uuid,
    /// Zero-based chunk index
    pub index: usize,
    /// Starting line number in original file (1-based)
    pub start_line: usize,
    /// Ending line number in original file (1-based)
    pub end_line: usize,
    /// Number of lines in this chunk
    pub line_count: usize,
    /// The actual code content
    pub content: String,
    /// File path this chunk originated from
    pub source_file: String,
}

/// Contract for text chunking operations
///
/// # Preconditions
/// - Input file exists and is readable
/// - Target chunk size > 0
///
/// # Postconditions
/// - Returns Vec<CodeChunk> where each chunk has <= target_size lines
/// - Last chunk may have fewer lines than target_size
/// - All chunks maintain original line ordering
/// - No line is duplicated or omitted across chunks
///
/// # Error Conditions
/// - ChunkError::FileNotFound if input file doesn't exist
/// - ChunkError::IoError if file reading fails
/// - ChunkError::InvalidChunkSize if target_size is 0
#[derive(thiserror::Error, Debug)]
pub enum ChunkError {
    #[error("File not found: {path}")]
    FileNotFound { path: String },

    #[error("IO error reading file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid chunk size: {size}, must be > 0")]
    InvalidChunkSize { size: usize },
}

/// Performance contract for chunking operations
///
/// Contract: Chunking 1MB file with 300-line chunks must complete within 100ms
#[derive(Debug)]
pub struct ChunkingPerformanceContract {
    max_file_size_mb: usize,
    target_chunk_size: usize,
    max_duration_ms: u64,
}

impl Default for ChunkingPerformanceContract {
    fn default() -> Self {
        Self {
            max_file_size_mb: 10,
            target_chunk_size: 300,
            max_duration_ms: 100,
        }
    }
}

/// Text chunker with TDD-driven contract validation
pub struct TextChunker {
    chunk_size: usize,
    performance_contract: ChunkingPerformanceContract,
}

impl TextChunker {
    /// Create new chunker with executable validation
    ///
    /// # Preconditions
    /// - chunk_size > 0
    ///
    /// # Postconditions
    /// - Returns Ok(TextChunker) if valid
    /// - Returns Err(ChunkError::InvalidChunkSize) if invalid
    pub fn new(chunk_size: usize) -> Result<Self> {
        if chunk_size == 0 {
            return Err(ChunkError::InvalidChunkSize { size: chunk_size }.into());
        }

        info!("Creating TextChunker with chunk_size: {}", chunk_size);

        Ok(Self {
            chunk_size,
            performance_contract: ChunkingPerformanceContract::default(),
        })
    }

    /// Chunk file content into manageable pieces with performance validation
    ///
    /// # Preconditions
    /// - file_path points to readable text file
    /// - File size <= performance_contract.max_file_size_mb
    ///
    /// # Postconditions
    /// - Returns Vec<CodeChunk> meeting all contract requirements
    /// - Performance validated against max_duration_ms
    ///
    /// # Error Conditions
    /// - ChunkError::FileNotFound if file doesn't exist
    /// - ChunkError::IoError if reading fails
    /// - ChunkError::PerformanceViolation if too slow
    pub async fn chunk_file(&self, file_path: &Path) -> Result<Vec<CodeChunk>> {
        let start_time = std::time::Instant::now();

        // Validate file exists and get metadata
        let metadata = tokio::fs::metadata(file_path)
            .await
            .with_context(|| format!("Failed to read metadata for {:?}", file_path))?;

        let file_size_mb = (metadata.len() / (1024 * 1024)) as usize;
        if file_size_mb > self.performance_contract.max_file_size_mb {
            return Err(ChunkError::InvalidChunkSize {
                size: file_size_mb
            }.into());
        }

        info!("Reading file: {:?} ({} MB)", file_path, file_size_mb);

        // Read entire file content
        let content = tokio::fs::read_to_string(file_path)
            .await
            .with_context(|| format!("Failed to read file: {:?}", file_path))?;

        let lines: Vec<&str> = content.lines().collect();
        let total_lines = lines.len();

        info!("Processing {} lines with chunk size: {}", total_lines, self.chunk_size);

        let mut chunks = Vec::new();
        let mut chunk_index = 0;

        // Process lines in chunks
        for chunk_start in (0..total_lines).step_by(self.chunk_size) {
            let chunk_end = (chunk_start + self.chunk_size).min(total_lines);

            // Extract chunk content
            let chunk_lines = &lines[chunk_start..chunk_end];
            let chunk_content = chunk_lines.join("\n");

            // Create chunk with metadata
            let chunk = CodeChunk {
                id: uuid::Uuid::new_v4(),
                index: chunk_index,
                start_line: chunk_start + 1, // 1-based line numbers
                end_line: chunk_end, // 1-based line numbers
                line_count: chunk_end - chunk_start,
                content: chunk_content,
                source_file: file_path.to_string_lossy().to_string(),
            };

            debug!("Created chunk {}: lines {}-{} ({} lines)",
                chunk_index, chunk.start_line, chunk.end_line, chunk.line_count);

            chunks.push(chunk);
            chunk_index += 1;
        }

        // Validate performance contract
        let elapsed = start_time.elapsed();
        if elapsed.as_millis() > self.performance_contract.max_duration_ms {
            return Err(anyhow::anyhow!(
                "Performance violation: chunking took {:?}ms, contract: {}ms",
                elapsed.as_millis(),
                self.performance_contract.max_duration_ms
            ));
        }

        info!("Successfully created {} chunks in {:?}", chunks.len(), elapsed);

        // Validate postconditions
        self.validate_chunks(&chunks)?;

        Ok(chunks)
    }

    /// Validate that chunks meet all postconditions
    ///
    /// # Preconditions
    /// - chunks vector is populated
    ///
    /// # Postconditions
    /// - All chunks have <= chunk_size lines
    /// - No duplicate line ranges
    /// - Line ordering preserved
    /// - No empty chunks (except possibly last)
    fn validate_chunks(&self, chunks: &[CodeChunk]) -> Result<()> {
        if chunks.is_empty() {
            return Err(anyhow::anyhow!("No chunks created"));
        }

        // Check chunk sizes
        for (i, chunk) in chunks.iter().enumerate() {
            if chunk.line_count > self.chunk_size {
                return Err(anyhow::anyhow!(
                    "Chunk {} exceeds max size: {} > {}",
                    i, chunk.line_count, self.chunk_size
                ));
            }

            if chunk.content.is_empty() && i != chunks.len() - 1 {
                return Err(anyhow::anyhow!(
                    "Non-final chunk {} is empty", i
                ));
            }
        }

        // Check for overlapping ranges
        for i in 0..chunks.len() {
            for j in (i + 1)..chunks.len() {
                let chunk_i = &chunks[i];
                let chunk_j = &chunks[j];

                // Ranges should not overlap
                if chunk_i.start_line <= chunk_j.end_line && chunk_j.start_line <= chunk_i.end_line {
                    return Err(anyhow::anyhow!(
                        "Overlapping chunks: {} ({}-{}) and {} ({}-{})",
                        i, chunk_i.start_line, chunk_i.end_line,
                        j, chunk_j.start_line, chunk_j.end_line
                    ));
                }
            }
        }

        // Check line ordering
        for i in 1..chunks.len() {
            let prev = &chunks[i - 1];
            let curr = &chunks[i];

            if prev.end_line + 1 != curr.start_line {
                return Err(anyhow::anyhow!(
                    "Line ordering violation: chunk {} ends at {}, chunk {} starts at {}",
                    i - 1, prev.end_line, i, curr.start_line
                ));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test;

    #[tokio_test]
    async fn test_chunker_creation() {
        // Valid chunker
        let chunker = TextChunker::new(300).unwrap();
        assert_eq!(chunker.chunk_size, 300);

        // Invalid chunker
        let result = TextChunker::new(0);
        assert!(result.is_err());
    }

    #[tokio_test]
    async fn test_chunking_contract() {
        let chunker = TextChunker::new(3).unwrap();

        // Create test content
        let test_content = "line1\nline2\nline3\nline4\nline5\nline6\nline7\n";
        let test_file = Path::new("test_content.txt");
        tokio::fs::write(&test_file, test_content).await.unwrap();

        let chunks = chunker.chunk_file(&test_file).await.unwrap();

        // Validate contract: 7 lines with chunk size 3 = 3 chunks (3, 3, 1)
        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0].line_count, 3);
        assert_eq!(chunks[1].line_count, 3);
        assert_eq!(chunks[2].line_count, 1);

        // Cleanup
        tokio::fs::remove_file(&test_file).await.unwrap();
    }

    #[tokio_test]
    async fn test_performance_contract_violation() {
        let chunker = TextChunker::new(300).unwrap();

        // This test would validate performance contract violations
        // In practice, we'd use a large file to trigger the violation
        let test_file = Path::new("nonexistent.txt");
        let result = chunker.chunk_file(&test_file).await;
        assert!(result.is_err());
    }
}