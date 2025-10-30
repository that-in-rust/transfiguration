use crate::models::{ValidationResult, ResearchSource};
use crate::errors::{Result, ResearchError};
use chrono::Utc;
use uuid::Uuid;
use std::collections::HashMap;

pub struct SourceVerifier {
    client: reqwest::Client,
    domain_reliability: HashMap<String, f32>,
}

impl SourceVerifier {
    pub fn new() -> Self {
        let mut domain_reliability = HashMap::new();
        
        // Define reliability scores for different domains
        domain_reliability.insert("github.com".to_string(), 0.9);
        domain_reliability.insert("gitlab.com".to_string(), 0.85);
        domain_reliability.insert("bitbucket.org".to_string(), 0.8);
        domain_reliability.insert("docs.rs".to_string(), 0.9);
        domain_reliability.insert("crates.io".to_string(), 0.9);
        domain_reliability.insert("rust-lang.org".to_string(), 0.95);
        domain_reliability.insert("mozilla.org".to_string(), 0.9);
        domain_reliability.insert("microsoft.com".to_string(), 0.85);
        domain_reliability.insert("google.com".to_string(), 0.85);
        domain_reliability.insert("stackoverflow.com".to_string(), 0.75);
        domain_reliability.insert("reddit.com".to_string(), 0.6);
        domain_reliability.insert("medium.com".to_string(), 0.65);
        domain_reliability.insert("dev.to".to_string(), 0.7);

        Self {
            client: reqwest::Client::builder()
                .user_agent("Research Verifier 1.0")
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap(),
            domain_reliability,
        }
    }

    /// Verify a source URL and assess its reliability
    pub async fn verify_source(
        &self,
        entity_type: &str,
        entity_id: Uuid,
        entity_data: &serde_json::Value,
    ) -> Result<ValidationResult> {
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();
        let mut verification_score = 0.0;

        // Extract URL from entity data
        let url = match entity_data.get("url").and_then(|v| v.as_str()) {
            Some(url) => url,
            None => {
                issues.push("No URL found for verification".to_string());
                return Ok(ValidationResult {
                    id: Uuid::new_v4(),
                    entity_type: entity_type.to_string(),
                    entity_id,
                    validation_type: "source_verification".to_string(),
                    status: "failed".to_string(),
                    score: Some(0.0),
                    issues,
                    recommendations: vec!["Add URL for source verification".to_string()],
                    validated_at: Utc::now(),
                    validator: "SourceVerifier::verify_source".to_string(),
                });
            }
        };

        // URL format validation
        let url_validity = self.validate_url_format(url).await?;
        verification_score += url_validity * 0.2;

        if url_validity < 0.5 {
            issues.push(format!("Invalid URL format: {}", url));
            recommendations.push("Correct URL format".to_string());
        }

        // Domain reliability assessment
        let domain_reliability = self.assess_domain_reliability(url).await?;
        verification_score += domain_reliability * 0.3;

        if domain_reliability < 0.6 {
            issues.push("Low domain reliability score".to_string());
            recommendations.push("Verify information with additional sources".to_string());
        }

        // Accessibility check
        let accessibility = self.check_url_accessibility(url).await?;
        verification_score += accessibility * 0.3;

        if accessibility < 0.5 {
            issues.push("URL is not accessible or returns errors".to_string());
            recommendations.push("Update URL or find alternative source".to_string());
        }

        // Content verification
        let content_verification = self.verify_content_relevance(url, entity_data).await?;
        verification_score += content_verification * 0.2;

        if content_verification < 0.5 {
            issues.push("Content may not be relevant to the research topic".to_string());
            recommendations.push("Review content relevance and update if necessary".to_string());
        }

        // Generate additional recommendations
        if verification_score >= 0.8 {
            recommendations.push("High-quality source suitable for research".to_string());
        } else if verification_score >= 0.6 {
            recommendations.push("Moderate quality source - consider additional verification".to_string());
        } else {
            recommendations.push("Low quality source - seek alternative sources".to_string());
        }

        let status = match verification_score {
            s if s >= 0.8 => "passed",
            s if s >= 0.6 => "warning",
            _ => "failed",
        };

        Ok(ValidationResult {
            id: Uuid::new_v4(),
            entity_type: entity_type.to_string(),
            entity_id,
            validation_type: "source_verification".to_string(),
            status: status.to_string(),
            score: Some(verification_score),
            issues,
            recommendations,
            validated_at: Utc::now(),
            validator: "SourceVerifier::verify_source".to_string(),
        })
    }

    /// Verify multiple sources for cross-referencing
    pub async fn verify_multiple_sources(&self, sources: &[ResearchSource]) -> Result<ValidationResult> {
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();
        let mut total_score = 0.0;
        let mut verified_count = 0;

        for source in sources {
            match self.verify_single_source(source).await {
                Ok(score) => {
                    total_score += score;
                    verified_count += 1;
                }
                Err(e) => {
                    issues.push(format!("Failed to verify source {}: {}", source.url, e));
                }
            }
        }

        let average_score = if verified_count > 0 {
            total_score / verified_count as f32
        } else {
            0.0
        };

        // Check source diversity
        let source_diversity = self.assess_source_diversity(sources).await?;
        let final_score = (average_score * 0.7) + (source_diversity * 0.3);

        if source_diversity < 0.6 {
            issues.push("Low source diversity - may indicate bias".to_string());
            recommendations.push("Include sources from different platforms and domains".to_string());
        }

        if sources.len() < 3 {
            issues.push("Insufficient number of sources for cross-verification".to_string());
            recommendations.push("Add more sources to improve verification confidence".to_string());
        }

        let status = match final_score {
            s if s >= 0.8 => "passed",
            s if s >= 0.6 => "warning",
            _ => "failed",
        };

        Ok(ValidationResult {
            id: Uuid::new_v4(),
            entity_type: "source_collection".to_string(),
            entity_id: Uuid::new_v4(),
            validation_type: "multi_source_verification".to_string(),
            status: status.to_string(),
            score: Some(final_score),
            issues,
            recommendations,
            validated_at: Utc::now(),
            validator: "SourceVerifier::verify_multiple_sources".to_string(),
        })
    }

    /// Validate URL format
    async fn validate_url_format(&self, url: &str) -> Result<f32> {
        match url::Url::parse(url) {
            Ok(parsed_url) => {
                // Check for required components
                let mut score = 0.5; // Base score for valid URL

                if parsed_url.scheme() == "https" {
                    score += 0.3; // Bonus for HTTPS
                } else if parsed_url.scheme() == "http" {
                    score += 0.1; // Small bonus for HTTP
                }

                if parsed_url.domain().is_some() {
                    score += 0.2; // Bonus for having domain
                }

                Ok(score.min(1.0))
            }
            Err(_) => Ok(0.0),
        }
    }

    /// Assess domain reliability
    async fn assess_domain_reliability(&self, url: &str) -> Result<f32> {
        if let Ok(parsed_url) = url::Url::parse(url) {
            if let Some(domain) = parsed_url.domain() {
                // Check exact domain match
                if let Some(&reliability) = self.domain_reliability.get(domain) {
                    return Ok(reliability);
                }

                // Check domain patterns
                if domain.ends_with(".edu") {
                    Ok(0.85) // Educational institutions
                } else if domain.ends_with(".gov") {
                    Ok(0.9) // Government sites
                } else if domain.ends_with(".org") {
                    Ok(0.75) // Organizations
                } else if domain.contains("github") || domain.contains("gitlab") {
                    Ok(0.85) // Code repositories
                } else {
                    Ok(0.6) // Default for unknown domains
                }
            } else {
                Ok(0.3) // No domain
            }
        } else {
            Ok(0.1) // Invalid URL
        }
    }

    /// Check if URL is accessible
    async fn check_url_accessibility(&self, url: &str) -> Result<f32> {
        match self.client.head(url).send().await {
            Ok(response) => {
                let status = response.status();
                match status.as_u16() {
                    200..=299 => Ok(1.0), // Success
                    300..=399 => Ok(0.8), // Redirect (still accessible)
                    400..=499 => Ok(0.2), // Client error
                    500..=599 => Ok(0.1), // Server error
                    _ => Ok(0.0),
                }
            }
            Err(_) => {
                // Try GET request as fallback
                match self.client.get(url).send().await {
                    Ok(response) => {
                        let status = response.status();
                        match status.as_u16() {
                            200..=299 => Ok(0.9), // Success but slower
                            300..=399 => Ok(0.7), // Redirect
                            _ => Ok(0.1),
                        }
                    }
                    Err(_) => Ok(0.0), // Completely inaccessible
                }
            }
        }
    }

    /// Verify content relevance
    async fn verify_content_relevance(&self, url: &str, entity_data: &serde_json::Value) -> Result<f32> {
        // Try to fetch content and check for relevant keywords
        match self.client.get(url).send().await {
            Ok(response) => {
                if let Ok(content) = response.text().await {
                    let relevance_score = self.calculate_content_relevance(&content, entity_data).await?;
                    Ok(relevance_score)
                } else {
                    Ok(0.5) // Neutral score if content can't be read
                }
            }
            Err(_) => Ok(0.3), // Low score if content can't be fetched
        }
    }

    /// Calculate content relevance based on keywords
    async fn calculate_content_relevance(&self, content: &str, entity_data: &serde_json::Value) -> Result<f32> {
        let content_lower = content.to_lowercase();
        let mut relevance_score = 0.0;
        let mut keyword_count = 0;

        // Extract keywords from entity data
        let keywords = self.extract_keywords_from_entity(entity_data).await?;

        for keyword in &keywords {
            keyword_count += 1;
            if content_lower.contains(&keyword.to_lowercase()) {
                relevance_score += 1.0;
            }
        }

        if keyword_count > 0 {
            Ok((relevance_score / keyword_count as f32).min(1.0))
        } else {
            Ok(0.5) // Neutral score if no keywords to check
        }
    }

    /// Extract keywords from entity data
    async fn extract_keywords_from_entity(&self, entity_data: &serde_json::Value) -> Result<Vec<String>> {
        let mut keywords = Vec::new();

        // Extract from name
        if let Some(name) = entity_data.get("name").and_then(|v| v.as_str()) {
            keywords.extend(name.split_whitespace().map(|s| s.to_string()));
        }

        // Extract from description
        if let Some(description) = entity_data.get("description").and_then(|v| v.as_str()) {
            // Extract significant words (longer than 3 characters)
            keywords.extend(
                description
                    .split_whitespace()
                    .filter(|word| word.len() > 3)
                    .take(5) // Limit to first 5 significant words
                    .map(|s| s.to_string())
            );
        }

        // Extract from category
        if let Some(category) = entity_data.get("category").and_then(|v| v.as_str()) {
            keywords.push(category.replace('_', " "));
        }

        // Extract from metadata
        if let Some(metadata) = entity_data.get("metadata").and_then(|v| v.as_object()) {
            if let Some(language) = metadata.get("language").and_then(|v| v.as_str()) {
                keywords.push(language.to_string());
            }
        }

        Ok(keywords)
    }

    /// Verify a single source
    async fn verify_single_source(&self, source: &ResearchSource) -> Result<f32> {
        let mut score = 0.0;

        // URL validity
        let url_score = self.validate_url_format(&source.url).await?;
        score += url_score * 0.3;

        // Domain reliability
        let domain_score = self.assess_domain_reliability(&source.url).await?;
        score += domain_score * 0.4;

        // Accessibility
        let access_score = self.check_url_accessibility(&source.url).await?;
        score += access_score * 0.3;

        Ok(score)
    }

    /// Assess diversity of source collection
    async fn assess_source_diversity(&self, sources: &[ResearchSource]) -> Result<f32> {
        if sources.is_empty() {
            return Ok(0.0);
        }

        let mut domains = std::collections::HashSet::new();
        let mut source_types = std::collections::HashSet::new();

        for source in sources {
            // Extract domain
            if let Ok(parsed_url) = url::Url::parse(&source.url) {
                if let Some(domain) = parsed_url.domain() {
                    domains.insert(domain.to_string());
                }
            }

            // Collect source types
            source_types.insert(source.source_type.clone());
        }

        // Calculate diversity scores
        let domain_diversity = (domains.len() as f32 / sources.len() as f32).min(1.0);
        let type_diversity = (source_types.len() as f32 / 5.0).min(1.0); // Normalize to max 5 types

        Ok((domain_diversity + type_diversity) / 2.0)
    }

    /// Check for duplicate or near-duplicate sources
    pub async fn detect_duplicate_sources(&self, sources: &[ResearchSource]) -> Result<Vec<(usize, usize)>> {
        let mut duplicates = Vec::new();

        for (i, source1) in sources.iter().enumerate() {
            for (j, source2) in sources.iter().enumerate().skip(i + 1) {
                if self.are_sources_similar(source1, source2).await? {
                    duplicates.push((i, j));
                }
            }
        }

        Ok(duplicates)
    }

    /// Check if two sources are similar (potential duplicates)
    async fn are_sources_similar(&self, source1: &ResearchSource, source2: &ResearchSource) -> Result<bool> {
        // Exact URL match
        if source1.url == source2.url {
            return Ok(true);
        }

        // Similar URLs (different protocols or trailing slashes)
        let url1_normalized = source1.url.trim_end_matches('/').replace("https://", "").replace("http://", "");
        let url2_normalized = source2.url.trim_end_matches('/').replace("https://", "").replace("http://", "");
        
        if url1_normalized == url2_normalized {
            return Ok(true);
        }

        // Similar titles (if available)
        if let (Some(title1), Some(title2)) = (&source1.title, &source2.title) {
            let similarity = self.calculate_string_similarity(title1, title2);
            if similarity > 0.9 {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Calculate string similarity (simple implementation)
    fn calculate_string_similarity(&self, s1: &str, s2: &str) -> f32 {
        let s1_lower = s1.to_lowercase();
        let s2_lower = s2.to_lowercase();

        if s1_lower == s2_lower {
            return 1.0;
        }

        // Simple word overlap similarity
        let words1: std::collections::HashSet<&str> = s1_lower.split_whitespace().collect();
        let words2: std::collections::HashSet<&str> = s2_lower.split_whitespace().collect();

        let intersection = words1.intersection(&words2).count();
        let union = words1.union(&words2).count();

        if union > 0 {
            intersection as f32 / union as f32
        } else {
            0.0
        }
    }
}