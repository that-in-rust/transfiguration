# D03: Tree-sitter Semantic Chunking Research

## Overview

This document contains comprehensive research on tree-sitter based semantic chunking for code summarization, addressing the fundamental limitation of line-based chunking and providing a foundation for intelligent code parsing.

---

## 1. Tree-sitter Fundamentals

### 1.1 Core Data Structures

Tree-sitter provides a **concrete syntax tree** (CST) rather than an abstract syntax tree (AST), preserving all source code details including whitespace and comments.

**Main Types in Rust:**
```rust
pub struct Parser {
    // Main parsing engine
}

pub struct Tree {
    // Complete syntax tree for a file
}

pub struct Node {
    // Individual nodes in the syntax tree
    pub fn kind(&self) -> String;           // Node type ("function_item", "struct_item")
    pub fn is_named(&self) -> bool;         // Whether node represents a named construct
    pub fn start_byte(&self) -> usize;      // Byte offset in source
    pub fn end_byte(&self) -> usize;        // End byte offset
    pub fn start_position(&self) -> Point;  // Row/column position
    pub fn end_position(&self) -> Point;    // End row/column
    pub fn utf8_text(&self, source: &[u8]) -> Result<&str, ()>; // Extract source text
    pub fn parent(&self) -> Option<Node>;   // Parent node
    pub fn child(&self, index: usize) -> Option<Node>; // Child by index
    pub fn next_sibling(&self) -> Option<Node>; // Next sibling
    pub fn child_by_field_name(&self, field_name: &str) -> Option<Node>; // Field access
}

pub struct Query {
    // Pattern matching engine for extracting specific constructs
}

pub struct TreeCursor {
    // Iterator for efficient tree traversal
}
```

### 1.2 Key Advantages Over Line-based Chunking

**Line-based Problems:**
- Arbitrary boundaries break functions across chunks
- No semantic understanding of code structure
- Poor handling of different programming languages
- Loss of context and relationships

**Tree-sitter Benefits:**
- **Semantic Boundaries**: Respects function, class, and module boundaries
- **Language Awareness**: Different parsing rules per language
- **Context Preservation**: Maintains relationships between code elements
- **Incremental Parsing**: Efficient re-parsing of changed files
- **Error Recovery**: Graceful handling of syntax errors

---

## 2. Semantic Chunking Granularity

### 2.1 Rust Language Semantic Constructs

Based on tree-sitter Rust grammar, key semantic levels include:

**Top-level Declarations:**
- `function_item` - Function definitions
- `struct_item` - Struct definitions
- `impl_item` - Implementation blocks
- `trait_item` - Trait definitions
- `mod_item` - Module declarations
- `enum_item` - Enum definitions
- `use_declaration` - Import statements
- `const_item` - Constant definitions
- `static_item` - Static variable definitions

**Implementation-level:**
- `function_signature_item` - Method signatures
- `field_declaration` - Struct fields
- `type_parameters` - Generic parameters
- `where_clause` - Where constraints
- `parameters` - Function parameters
- `block` - Function bodies
- `trait_item` - Trait method definitions

**Expression-level:**
- `call_expression` - Function/method calls
- `field_expression` - Field access (obj.field)
- `method_expression` - Method calls (obj.method())
- `macro_invocation` - Macro calls
- `let_declaration` - Variable declarations

### 2.2 Multi-language Support

**Python:**
- `function_definition` - Function definitions
- `class_definition` - Class definitions
- `async_function_definition` - Async functions
- `decorated_definition` - Decorated functions/classes

**JavaScript/TypeScript:**
- `function_declaration` - Function declarations
- `class_declaration` - Class declarations
- `method_definition` - Method definitions
- `arrow_function` - Arrow functions
- `interface_declaration` - TypeScript interfaces

**Go:**
- `function_declaration` - Function definitions
- `method_declaration` - Method definitions
- `struct_type` - Struct definitions
- `interface_type` - Interface definitions

---

## 3. Storage Format Analysis

### 3.1 Serialization Format Comparison

| Format | Size | Speed | Human-readable | Debugging | Use Case |
|--------|------|-------|----------------|-----------|----------|
| **JSON** | Large | Slow | Yes | Easy | Development, debugging |
| **MessagePack** | Medium | Fast | No | Medium | Production storage |
| **Bincode** | Small | Fastest | No | Hard | In-memory caching |
| **SQLite** | Medium | Medium | Yes | Easy | Large projects, queries |
| **CBOR** | Medium | Fast | Yes | Easy | Web-friendly format |

### 3.2 Recommended Storage Strategy

**Development Environment:**
- **JSON**: Easy debugging and inspection
- Human-readable for manual verification
- Simple integration with existing tools

**Production Environment:**
- **MessagePack**: Best balance of size and performance
- Binary format for efficient storage and retrieval
- Widely supported across languages

**Large-scale Projects:**
- **SQLite**: Complex querying capabilities
- Full-text search for code search
- ACID compliance for data integrity
- Efficient indexing for large datasets

### 3.3 Storage Schema Design

**JSON/MessagePack Structure:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticChunk {
    pub id: String,                    // Unique identifier
    pub file_path: PathBuf,           // Source file path
    pub node_type: String,            // Tree-sitter node type
    pub name: Option<String>,         // Extracted name if available
    pub content: String,              // Full source text
    pub start_byte: usize,            // Position metadata
    pub end_byte: usize,
    pub start_line: u32,
    pub end_line: u32,
    pub start_column: u32,
    pub end_column: u32,
    pub children: Vec<String>,        // IDs of child chunks
    pub parent_id: Option<String>,    // ID of parent chunk
    pub metadata: ChunkMetadata,      // Additional context
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkMetadata {
    pub visibility: Option<String>,   // pub, pub(crate), etc.
    pub is_unsafe: bool,             // Contains unsafe code
    pub generics: Vec<String>,       // Generic type parameters
    pub dependencies: Vec<String>,   // Referenced types/functions
    pub complexity_score: f32,       // Cyclomatic complexity
    pub nesting_depth: u32,          // Maximum nesting depth
    pub doc_comments: Vec<String>,    // Associated documentation
    pub attributes: Vec<String>,     // Rust attributes (#[derive], etc.)
}
```

**SQLite Schema:**
```sql
CREATE TABLE semantic_chunks (
    id TEXT PRIMARY KEY,
    file_path TEXT NOT NULL,
    node_type TEXT NOT NULL,
    name TEXT,
    content TEXT NOT NULL,
    start_byte INTEGER NOT NULL,
    end_byte INTEGER NOT NULL,
    start_line INTEGER NOT NULL,
    end_line INTEGER NOT NULL,
    start_column INTEGER NOT NULL,
    end_column INTEGER NOT NULL,
    parent_id TEXT,
    metadata_json TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (parent_id) REFERENCES semantic_chunks(id)
);

CREATE INDEX idx_chunks_file_path ON semantic_chunks(file_path);
CREATE INDEX idx_chunks_node_type ON semantic_chunks(node_type);
CREATE INDEX idx_chunks_name ON semantic_chunks(name);
CREATE INDEX idx_chunks_parent ON semantic_chunks(parent_id);

-- Full-text search for code content
CREATE VIRTUAL TABLE chunks_fts USING fts5(content, name, file_path);

-- For dependency tracking
CREATE TABLE chunk_dependencies (
    chunk_id TEXT,
    dependency_type TEXT,
    dependency_name TEXT,
    dependency_file TEXT,
    FOREIGN KEY (chunk_id) REFERENCES semantic_chunks(id)
);
```

---

## 4. Query-based Extraction Patterns

### 4.1 Tree-sitter Query Language

Tree-sitter queries use a pattern-matching syntax to extract specific constructs from the syntax tree.

**Query Syntax Basics:**
```
(node_type
    child_field: (child_node_type @capture_name)
) @parent_capture
```

### 4.2 Language-specific Query Patterns

**Rust Function Queries:**
```scheme
; Function definitions with names and bodies
(function_item
  name: (identifier) @function.name
  parameters: (parameters
    (parameter
      pattern: (type_identifier) @param.type
    )*
  )
  body: (block) @function.body
) @function.definition

; Method definitions within impl blocks
(impl_item
  (function_item
    name: (identifier) @method.name
    body: (block) @method.body
  ) @method.definition
) @impl.block

; Struct definitions with fields
(struct_item
  name: (type_identifier) @struct.name
  body: (field_declaration_list
    (field_declaration
      name: (field_identifier) @field.name
      type: (type_identifier) @field.type
    )*
  ) @struct.fields
) @struct.definition

; Trait definitions
(trait_item
  name: (type_identifier) @trait.name
  body: (declaration_list
    (function_item
      name: (identifier) @trait_method.name
    )*
  ) @trait.body
) @trait.definition
```

**Python Function Queries:**
```scheme
; Function definitions
(function_definition
  name: (identifier) @function.name
  parameters: (parameters
    (identifier) @param.name
  )*
  body: (block) @function.body
) @function.definition

; Class definitions with methods
(class_definition
  name: (identifier) @class.name
  body: (block
    (function_definition
      name: (identifier) @method.name
      body: (block) @method.body
    )*
  ) @class.body
) @class.definition
```

### 4.3 Query Implementation in Rust

```rust
use tree_sitter::{Language, Query, QueryCursor};

pub struct SemanticExtractor {
    language: Language,
    function_query: Query,
    struct_query: Query,
    impl_query: Query,
}

impl SemanticExtractor {
    pub fn new(language: Language) -> Result<Self, QueryError> {
        Ok(Self {
            language,
            function_query: Query::new(language, FUNCTION_QUERIES)?,
            struct_query: Query::new(language, STRUCT_QUERIES)?,
            impl_query: Query::new(language, IMPL_QUERIES)?,
        })
    }

    pub fn extract_functions(&self, tree: &Tree, source: &[u8]) -> Vec<SemanticChunk> {
        let mut cursor = QueryCursor::new();
        let matches = cursor.matches(&self.function_query, tree.root_node(), source);

        matches.map(|m| {
            // Extract function name, body, and metadata
            self.build_function_chunk(m, source)
        }).collect()
    }
}
```

---

## 5. File Processing Workflow

### 5.1 Directory Discovery and Filtering

**File Discovery Strategy:**
```rust
use ignore::{WalkBuilder, WalkState};

pub struct FileDiscovery {
    include_patterns: Vec<String>,
    exclude_patterns: Vec<String>,
    max_file_size: usize,
}

impl FileDiscovery {
    pub fn discover_files(&self, root: &Path) -> Vec<PathBuf> {
        WalkBuilder::new(root)
            .types(
                TypesBuilder::new()
                    .add_defaults()
                    .select("rust")
                    .select("python")
                    .select("javascript")
                    .build()
            )
            .max_filesize(self.max_file_size as u64)
            .build()
            .filter_map(|e| e.ok())
            .filter(|e| {
                !e.file_type().unwrap().is_dir() &&
                self.should_include(e.path())
            })
            .map(|e| e.into_path())
            .collect()
    }
}
```

### 5.2 Parallel Processing Architecture

**Tokio-based Parallel Parsing:**
```rust
use tokio::sync::Semaphore;
use futures::stream::{self, StreamExt};

pub struct ParallelChunker {
    max_concurrent: usize,
    extractor: SemanticExtractor,
    storage: ChunkStorage,
}

impl ParallelChunker {
    pub async fn process_directory(&mut self, dir: &Path) -> Result<Vec<SemanticChunk>, Error> {
        let files = self.discover_files(dir);
        let semaphore = Arc::new(Semaphore::new(self.max_concurrent));

        let chunks = stream::iter(files)
            .map(|file| {
                let semaphore = Arc::clone(&semaphore);
                async move {
                    let _permit = semaphore.acquire().await?;
                    self.process_file(&file).await
                }
            })
            .buffer_unordered(self.max_concurrent)
            .collect::<Vec<_>>()
            .await;

        // Handle results and flatten
        chunks.into_iter().try_collect()
    }

    async fn process_file(&self, file: &Path) -> Result<Vec<SemanticChunk>, Error> {
        let source = tokio::fs::read(file).await?;
        let tree = self.extractor.parse(&source)?;
        let chunks = self.extractor.extract_all(&tree, &source)?;

        // Store chunks
        for chunk in &chunks {
            self.storage.store_chunk(chunk).await?;
        }

        Ok(chunks)
    }
}
```

### 5.3 Incremental Processing

**Change Detection and Caching:**
```rust
pub struct IncrementalProcessor {
    file_hash_cache: HashMap<PathBuf, String>,
    chunk_cache: LruCache<String, SemanticChunk>,
}

impl IncrementalProcessor {
    pub async fn process_changes(&mut self, changed_files: Vec<PathBuf>) -> Result<Vec<SemanticChunk>, Error> {
        let mut new_chunks = Vec::new();

        for file in changed_files {
            let current_hash = self.calculate_file_hash(&file).await?;
            let cached_hash = self.file_hash_cache.get(&file);

            if cached_hash != Some(&current_hash) {
                // File changed, re-process
                let chunks = self.process_file(&file).await?;
                new_chunks.extend(chunks);
                self.file_hash_cache.insert(file, current_hash);
            }
        }

        Ok(new_chunks)
    }
}
```

---

## 6. Metadata and Context Strategy

### 6.1 Essential Metadata Categories

**Positional Information:**
- File path and line/column ranges
- Byte offsets for precise location
- Hierarchical position within code structure

**Semantic Information:**
- Node type (function, struct, impl, etc.)
- Visibility modifiers (pub, pub(crate), private)
- Generic type parameters and constraints
- Safety annotations (unsafe blocks)

**Dependency Tracking:**
- Type references and imports
- Function calls and method invocations
- Trait implementations and requirements
- Module dependencies and use statements

**Complexity Metrics:**
- Cyclomatic complexity
- Nesting depth
- Lines of code (excluding comments/blank lines)
- Parameter count and complexity

### 6.2 Cross-file Context Analysis

**Import/Use Resolution:**
```rust
pub struct DependencyAnalyzer {
    symbol_table: HashMap<String, Vec<FileLocation>>,
    import_graph: DiGraph<String, ()>,
}

impl DependencyAnalyzer {
    pub fn analyze_imports(&mut self, chunks: &[SemanticChunk]) -> DependencyGraph {
        for chunk in chunks {
            if chunk.node_type == "use_declaration" {
                self.process_use_declaration(chunk);
            } else if chunk.node_type == "function_item" {
                self.analyze_function_dependencies(chunk);
            }
        }

        self.build_dependency_graph()
    }
}
```

### 6.3 Documentation Integration

**Doc Comment Extraction:**
```rust
pub struct DocumentationExtractor;

impl DocumentationExtractor {
    pub fn extract_doc_comments(&self, node: &Node, source: &[u8]) -> Vec<String> {
        let mut comments = Vec::new();

        // Look for preceding comments
        let mut current = node.prev_sibling();
        while let Some(sibling) = current {
            if sibling.kind() == "line_comment" || sibling.kind() == "block_comment" {
                comments.push(sibling.utf8_text(source).unwrap().to_string());
            } else {
                break;
            }
            current = sibling.prev_sibling();
        }

        comments.reverse(); // Preserve original order
        comments
    }
}
```

---

## 7. Implementation Architecture

### 7.1 Core Components

**Parser Interface:**
```rust
#[async_trait]
pub trait LanguageParser {
    async fn parse_file(&self, file: &Path) -> Result<Tree, ParseError>;
    fn extract_chunks(&self, tree: &Tree, source: &[u8]) -> Result<Vec<SemanticChunk>, ExtractionError>;
    fn get_language_name(&self) -> &'static str;
}

pub struct TreeSitterParser {
    language: Language,
    parser: Parser,
    queries: HashMap<String, Query>,
}

impl LanguageParser for TreeSitterParser {
    async fn parse_file(&self, file: &Path) -> Result<Tree, ParseError> {
        let source = tokio::fs::read(file).await?;
        let mut parser = self.parser.clone();
        parser.parse(&source, None).ok_or(ParseError::InvalidSyntax)
    }

    fn extract_chunks(&self, tree: &Tree, source: &[u8]) -> Result<Vec<SemanticChunk>, ExtractionError> {
        let mut chunks = Vec::new();

        for (query_name, query) in &self.queries {
            let matches = self.extract_with_query(query, tree, source)?;
            chunks.extend(matches);
        }

        Ok(chunks)
    }
}
```

**Storage Abstraction:**
```rust
#[async_trait]
pub trait ChunkStorage {
    async fn store_chunk(&mut self, chunk: &SemanticChunk) -> Result<(), StorageError>;
    async fn get_chunk(&self, id: &str) -> Result<Option<SemanticChunk>, StorageError>;
    async fn get_chunks_by_file(&self, file: &Path) -> Result<Vec<SemanticChunk>, StorageError>;
    async fn search_chunks(&self, query: &str) -> Result<Vec<SemanticChunk>, StorageError>;
    async fn delete_chunks_by_file(&mut self, file: &Path) -> Result<(), StorageError>;
}

pub struct MessagePackStorage {
    base_path: PathBuf,
}

pub struct SqliteStorage {
    pool: SqlitePool,
}
```

### 7.2 Error Handling Strategy

**Error Types:**
```rust
#[derive(Debug, thiserror::Error)]
pub enum ChunkingError {
    #[error("Parse error in file {0}: {1}")]
    ParseError(PathBuf, #[source] ParseError),

    #[error("Query extraction failed: {0}")]
    ExtractionError(#[from] ExtractionError),

    #[error("Storage error: {0}")]
    StorageError(#[from] StorageError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum ExtractionError {
    #[error("Invalid query pattern: {0}")]
    InvalidQuery(String),

    #[error("Node text extraction failed")]
    TextExtractionFailed,

    #[error("Chunk size exceeds limits: {0} tokens")]
    ChunkTooLarge(usize),
}
```

---

## 8. Performance Considerations

### 8.1 Memory Management

**Efficient Memory Usage:**
- **Streaming Processing**: Process files without loading entire codebase into memory
- **Lazy Loading**: Load chunks on-demand for large repositories
- **Memory Pooling**: Reuse allocations for parsing operations
- **Reference Counting**: Share source code slices across chunks

**Rust Implementation:**
```rust
pub struct MemoryEfficientChunker {
    source_cache: LruCache<PathBuf, Arc<Vec<u8>>>,
    chunk_pool: ObjectPool<Vec<SemanticChunk>>,
}

impl MemoryEfficientChunker {
    pub async fn process_large_repo(&mut self, repo: &Path) -> Result<(), Error> {
        // Process files in batches to limit memory usage
        let files = self.discover_files(repo);
        let batch_size = 100;

        for chunk in files.chunks(batch_size) {
            self.process_file_batch(chunk).await?;
            // Clear cache between batches
            self.source_cache.clear();
        }

        Ok(())
    }
}
```

### 8.2 Parallel Processing Optimization

**Work Distribution:**
- **File-level Parallelism**: Parse different files concurrently
- **Chunk-level Parallelism**: Process chunks from large files concurrently
- **I/O Bound Operations**: Use Tokio for non-blocking file operations
- **CPU Bound Operations**: Use Rayon for parsing operations

**Thread Pool Configuration:**
```rust
pub struct OptimizedChunker {
    io_pool: Arc<Runtime>,
    cpu_pool: Arc<ThreadPool>,
    max_concurrent_files: usize,
    max_concurrent_chunks: usize,
}

impl OptimizedChunker {
    pub fn new() -> Self {
        let cpu_count = num_cpus::get();

        Self {
            io_pool: Arc::new(Runtime::new().unwrap()),
            cpu_pool: Arc::new(ThreadPoolBuilder::new()
                .num_threads(cpu_count)
                .build()
                .unwrap()),
            max_concurrent_files: cpu_count * 2,
            max_concurrent_chunks: cpu_count * 4,
        }
    }
}
```

---

## 9. Testing Strategy

### 9.1 Unit Testing

**Parser Testing:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_function_extraction() {
        let source = r#"
            fn example_function(param1: i32, param2: String) -> Result<i32> {
                Ok(param1 + param2.len() as i32)
            }
        "#;

        let parser = TreeSitterParser::new(tree_sitter_rust::language());
        let tree = parser.parse(None, source).unwrap();
        let chunks = parser.extract_chunks(&tree, source.as_bytes()).unwrap();

        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].node_type, "function_item");
        assert_eq!(chunks[0].name, Some("example_function".to_string()));
        assert_eq!(chunks[0].start_line, 2);
        assert_eq!(chunks[0].end_line, 4);
    }
}
```

### 9.2 Integration Testing

**Real-world Codebase Testing:**
```rust
#[tokio::test]
async fn test_real_codebase_processing() {
    let test_repo = include_str!("../test_data/sample_rust_project");
    let mut chunker = SemanticChunker::new();

    let chunks = chunker.process_directory(Path::new("test_data/sample_project")).await.unwrap();

    // Verify chunk properties
    assert!(!chunks.is_empty());

    // Check that no chunks exceed reasonable size limits
    for chunk in &chunks {
        assert!(chunk.content.len() < 10_000); // Reasonable chunk size
        assert!(chunk.end_line > chunk.start_line);
    }

    // Verify semantic boundaries are respected
    let function_chunks: Vec<_> = chunks.iter()
        .filter(|c| c.node_type == "function_item")
        .collect();

    for chunk in function_chunks {
        assert!(chunk.content.contains("fn "));
        assert!(chunk.content.contains('{'));
        assert!(chunk.content.contains('}'));
    }
}
```

### 9.3 Performance Benchmarking

**Benchmarking Suite:**
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_chunking_performance(c: &mut Criterion) {
    let source = include_str!("../test_data/large_rust_file.rs");
    let parser = TreeSitterParser::new(tree_sitter_rust::language());

    c.bench_function("chunk_extraction", |b| {
        b.iter(|| {
            let tree = parser.parse(None, black_box(source)).unwrap();
            let chunks = parser.extract_chunks(&tree, black_box(source.as_bytes())).unwrap();
            black_box(chunks)
        })
    });
}

criterion_group!(benches, bench_chunking_performance);
criterion_main!(benches);
```

---

## 10. Future Enhancements

### 10.1 Advanced Semantic Analysis

**Type System Integration:**
- Cross-reference type definitions and usage
- Track trait implementations across modules
- Build comprehensive dependency graphs

**Control Flow Analysis:**
- Extract control flow graphs from functions
- Identify complex branching patterns
- Analyze async/await patterns

### 10.2 Machine Learning Integration

**Intelligent Chunk Sizing:**
- Use ML to determine optimal chunk boundaries
- Learn from user feedback on chunk quality
- Adaptive chunking based on code complexity

**Content-based Classification:**
- Classify chunks by functionality (utility, business logic, tests)
- Identify code smells and anti-patterns
- Suggest refactoring opportunities

### 10.3 Multi-language Repository Support

**Cross-language Analysis:**
- Track dependencies between different languages
- Understand polyglot repository structures
- Unified chunking across language boundaries

**Build System Integration:**
- Integrate with Cargo, npm, pip build systems
- Understand project structure and dependencies
- Incremental processing based on build graphs

---

## 11. Conclusion

Tree-sitter based semantic chunking represents a significant improvement over naive line-based approaches for code summarization. Key advantages include:

1. **Semantic Accuracy**: Respect for language-specific boundaries and constructs
2. **Context Preservation**: Maintenance of relationships between code elements
3. **Multi-language Support**: Consistent parsing across 20+ programming languages
4. **Performance Optimization**: Efficient incremental parsing and parallel processing
5. **Extensibility**: Foundation for advanced code analysis and understanding

The research provides a comprehensive foundation for implementing production-ready semantic chunking that will significantly improve the quality of AI-generated code summaries while maintaining high performance for large codebases.

---

*Document Version: 1.0*
*Created: October 26, 2025*
*Research based on tree-sitter documentation, Rust ecosystem analysis, and production code analysis patterns*