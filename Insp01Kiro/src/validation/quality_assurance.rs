use crate::models::{ValidationResult, ConfidenceLevel};
use crate::errors::{Result, ResearchError};
use chrono::Utc;
use uuid::Uuid;
use std::collections::HashMap;

pub struct QualityAssurance {
    coverage_thresholds: HashMap<String, CoverageThresholds>,
    quality_metrics: QualityMetrics,
}

#[derive(Debug, Clone)]
pub struct CoverageThresholds {
    pub min_projects: i32,
    pub min_diversity_score: f32,
    pub min_quality_score: f32,
    pub required_attributes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct QualityMetrics {
    pub completeness_weight: f32,
    pub accuracy_weight: f32,
    pub reliability_weight: f32,
    pub diversity_weight: f32,
}

impl QualityAssurance {
    pub fn new() -> Self {
        let mut coverage_thresholds = HashMap::new();
        
        // Define coverage thresholds for each research category
        coverage_thresholds.insert("ide_migration".to_string(), CoverageThresholds {
            min_projects: 10,
            min_diversity_score: 0.7,
            min_quality_score: 0.75,
            required_attributes: vec![
                "source_technology".to_string(),
                "target_technology".to_string(),
                "migration_approach".to_string(),
                "performance_metrics".to_string(),
            ],
        });

        coverage_thresholds.insert("vscode_fork".to_string(), CoverageThresholds {
            min_projects: 15,
            min_diversity_score: 0.6,
            min_quality_score: 0.7,
            required_attributes: vec![
                "customization_type".to_string(),
                "extension_compatibility".to_string(),
                "business_model".to_string(),
            ],
        });

        coverage_thresholds.insert("rust_ide".to_string(), CoverageThresholds {
            min_projects: 8,
            min_diversity_score: 0.8,
            min_quality_score: 0.8,
            required_attributes: vec![
                "architecture_approach".to_string(),
                "gui_framework".to_string(),
                "performance_characteristics".to_string(),
            ],
        });

        coverage_thresholds.insert("ai_ide".to_string(), CoverageThresholds {
            min_projects: 12,
            min_diversity_score: 0.65,
            min_quality_score: 0.75,
            required_attributes: vec![
                "ai_integration_type".to_string(),
                "ai_providers".to_string(),
                "user_interaction_model".to_string(),
            ],
        });

        coverage_thresholds.insert("wasm_app".to_string(), CoverageThresholds {
            min_projects: 6,
            min_diversity_score: 0.75,
            min_quality_score: 0.8,
            required_attributes: vec![
                "wasm_framework".to_string(),
                "js_interop_patterns".to_string(),
                "performance_benchmarks".to_string(),
            ],
        });

        let quality_metrics = QualityMetrics {
            completeness_weight: 0.3,
            accuracy_weight: 0.3,
            reliability_weight: 0.25,
            diversity_weight: 0.15,
        };

        Self {
            coverage_thresholds,
            quality_metrics,
        }
    }

    /// Assess overall quality of an entity
    pub async fn assess_quality(
        &self,
        entity_type: &str,
        entity_id: Uuid,
        entity_data: &serde_json::Value,
    ) -> Result<ValidationResult> {
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();
        let mut quality_score = 0.0;

        // Assess completeness quality
        let completeness_score = self.assess_completeness_quality(entity_data).await?;
        quality_score += completeness_score * self.quality_metrics.completeness_weight;

        // Assess accuracy quality
        let accuracy_score = self.assess_accuracy_quality(entity_data).await?;
        quality_score += accuracy_score * self.quality_metrics.accuracy_weight;

        // Assess reliability quality
        let reliability_score = self.assess_reliability_quality(entity_data).await?;
        quality_score += reliability_score * self.quality_metrics.reliability_weight;

        // Assess diversity contribution (for projects)
        if entity_type == "project" {
            let diversity_score = self.assess_diversity_contribution(entity_data).await?;
            quality_score += diversity_score * self.quality_metrics.diversity_weight;
        } else {
            quality_score += 0.8 * self.quality_metrics.diversity_weight; // Neutral score for non-projects
        }

        // Generate quality-based issues and recommendations
        if completeness_score < 0.7 {
            issues.push("Low completeness score - missing critical data fields".to_string());
            recommendations.push("Review and complete missing data fields".to_string());
        }

        if accuracy_score < 0.7 {
            issues.push("Low accuracy score - data quality concerns detected".to_string());
            recommendations.push("Verify and correct data accuracy issues".to_string());
        }

        if reliability_score < 0.6 {
            issues.push("Low reliability score - source credibility concerns".to_string());
            recommendations.push("Verify sources and improve source reliability".to_string());
        }

        // Overall quality assessment
        let status = match quality_score {
            s if s >= 0.8 => "passed",
            s if s >= 0.6 => "warning",
            _ => "failed",
        };

        if quality_score >= 0.8 {
            recommendations.push("High quality data - suitable for analysis and pattern extraction".to_string());
        } else if quality_score >= 0.6 {
            recommendations.push("Moderate quality data - consider additional verification".to_string());
        } else {
            recommendations.push("Low quality data - requires significant improvement before use".to_string());
        }

        Ok(ValidationResult {
            id: Uuid::new_v4(),
            entity_type: entity_type.to_string(),
            entity_id,
            validation_type: "quality_assurance".to_string(),
            status: status.to_string(),
            score: Some(quality_score),
            issues,
            recommendations,
            validated_at: Utc::now(),
            validator: "QualityAssurance::assess_quality".to_string(),
        })
    }

    /// Validate research coverage for a category
    pub async fn validate_research_coverage(
        &self,
        category: &str,
        target_count: i32,
        current_projects: &[serde_json::Value],
    ) -> Result<ValidationResult> {
        let thresholds = self.coverage_thresholds.get(category).ok_or_else(|| {
            ResearchError::Validation {
                field: "category".to_string(),
                message: format!("No coverage thresholds defined for: {}", category),
            }
        })?;

        let mut issues = Vec::new();
        let mut recommendations = Vec::new();
        let mut coverage_score = 0.0;

        // Check project count
        let project_count = current_projects.len() as i32;
        let count_score = (project_count as f32 / thresholds.min_projects as f32).min(1.0);
        coverage_score += count_score * 0.4;

        if project_count < thresholds.min_projects {
            issues.push(format!(
                "Insufficient project count: {} (target: {})",
                project_count, thresholds.min_projects
            ));
            recommendations.push(format!(
                "Discover {} more projects to meet minimum threshold",
                thresholds.min_projects - project_count
            ));
        }

        // Check diversity
        let diversity_score = self.calculate_diversity_score(current_projects, &thresholds.required_attributes).await?;
        coverage_score += diversity_score * 0.3;

        if diversity_score < thresholds.min_diversity_score {
            issues.push(format!(
                "Low diversity score: {:.2} (target: {:.2})",
                diversity_score, thresholds.min_diversity_score
            ));
            recommendations.push("Increase diversity by targeting projects with different characteristics".to_string());
        }

        // Check quality distribution
        let quality_distribution = self.analyze_quality_distribution(current_projects).await?;
        let quality_score = quality_distribution.average_quality;
        coverage_score += quality_score * 0.3;

        if quality_score < thresholds.min_quality_score {
            issues.push(format!(
                "Low average quality: {:.2} (target: {:.2})",
                quality_score, thresholds.min_quality_score
            ));
            recommendations.push("Focus on higher quality projects or improve data collection".to_string());
        }

        // Check attribute coverage
        let attribute_coverage = self.check_attribute_coverage(current_projects, &thresholds.required_attributes).await?;
        coverage_score += attribute_coverage * 0.0; // This is already factored into quality

        // Generate coverage-specific recommendations
        if coverage_score >= 0.8 {
            recommendations.push("Excellent research coverage - ready for analysis".to_string());
        } else if coverage_score >= 0.6 {
            recommendations.push("Good coverage - consider targeted improvements".to_string());
        } else {
            recommendations.push("Insufficient coverage - significant expansion needed".to_string());
        }

        let status = match coverage_score {
            s if s >= 0.8 => "passed",
            s if s >= 0.6 => "warning",
            _ => "failed",
        };

        Ok(ValidationResult {
            id: Uuid::new_v4(),
            entity_type: "research_coverage".to_string(),
            entity_id: Uuid::new_v4(), // Generate a new ID for coverage validation
            validation_type: "coverage".to_string(),
            status: status.to_string(),
            score: Some(coverage_score),
            issues,
            recommendations,
            validated_at: Utc::now(),
            validator: format!("QualityAssurance::coverage::{}", category),
        })
    }

    /// Cross-validate findings across multiple sources
    pub async fn cross_validate_findings(
        &self,
        findings: &[serde_json::Value],
        sources: &[serde_json::Value],
    ) -> Result<ValidationResult> {
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();
        let mut validation_score = 0.0;

        // Check source diversity for findings
        let source_diversity = self.calculate_source_diversity(findings, sources).await?;
        validation_score += source_diversity * 0.4;

        if source_diversity < 0.6 {
            issues.push("Low source diversity - findings may be biased".to_string());
            recommendations.push("Seek additional sources to validate findings".to_string());
        }

        // Check for contradictory findings
        let contradiction_score = self.detect_contradictions(findings).await?;
        validation_score += contradiction_score * 0.3;

        if contradiction_score < 0.7 {
            issues.push("Contradictory findings detected - requires resolution".to_string());
            recommendations.push("Investigate and resolve contradictory findings".to_string());
        }

        // Check evidence strength
        let evidence_strength = self.assess_evidence_strength(findings, sources).await?;
        validation_score += evidence_strength * 0.3;

        if evidence_strength < 0.6 {
            issues.push("Weak evidence base for findings".to_string());
            recommendations.push("Strengthen evidence with additional sources and data".to_string());
        }

        let status = match validation_score {
            s if s >= 0.8 => "passed",
            s if s >= 0.6 => "warning",
            _ => "failed",
        };

        Ok(ValidationResult {
            id: Uuid::new_v4(),
            entity_type: "cross_validation".to_string(),
            entity_id: Uuid::new_v4(),
            validation_type: "cross_validation".to_string(),
            status: status.to_string(),
            score: Some(validation_score),
            issues,
            recommendations,
            validated_at: Utc::now(),
            validator: "QualityAssurance::cross_validate".to_string(),
        })
    }

    // Helper methods for quality assessment
    async fn assess_completeness_quality(&self, data: &serde_json::Value) -> Result<f32> {
        let mut score = 0.0;
        let mut total_checks = 0;

        // Check for presence of key fields
        let key_fields = ["id", "name", "description", "url"];
        for field in &key_fields {
            total_checks += 1;
            if data.get(field).is_some() {
                score += 1.0;
            }
        }

        // Check for non-empty string fields
        let string_fields = ["name", "description"];
        for field in &string_fields {
            if let Some(value) = data.get(field).and_then(|v| v.as_str()) {
                total_checks += 1;
                if !value.trim().is_empty() {
                    score += 1.0;
                }
            }
        }

        // Check for metadata richness
        if let Some(metadata) = data.get("metadata").and_then(|v| v.as_object()) {
            total_checks += 1;
            if metadata.len() > 3 {
                score += 1.0;
            }
        }

        Ok(if total_checks > 0 { score / total_checks as f32 } else { 0.0 })
    }

    async fn assess_accuracy_quality(&self, data: &serde_json::Value) -> Result<f32> {
        let mut score = 1.0; // Start with perfect score
        let mut penalty_count = 0;

        // Check URL validity
        if let Some(url) = data.get("url").and_then(|v| v.as_str()) {
            if url::Url::parse(url).is_err() {
                score -= 0.2;
                penalty_count += 1;
            }
        }

        // Check numeric ranges
        if let Some(stars) = data.get("stars").and_then(|v| v.as_i64()) {
            if stars < 0 || stars > 1_000_000 {
                score -= 0.15;
                penalty_count += 1;
            }
        }

        // Check date consistency
        if let (Some(created), Some(updated)) = (
            data.get("created_at").and_then(|v| v.as_str()),
            data.get("updated_at").and_then(|v| v.as_str()),
        ) {
            if let (Ok(created_dt), Ok(updated_dt)) = (
                chrono::DateTime::parse_from_rfc3339(created),
                chrono::DateTime::parse_from_rfc3339(updated),
            ) {
                if updated_dt < created_dt {
                    score -= 0.25;
                    penalty_count += 1;
                }
            }
        }

        Ok(score.max(0.0))
    }

    async fn assess_reliability_quality(&self, data: &serde_json::Value) -> Result<f32> {
        let mut score = 0.5; // Start with neutral score

        // GitHub sources are generally reliable
        if let Some(url) = data.get("url").and_then(|v| v.as_str()) {
            if url.contains("github.com") {
                score += 0.3;
            }
        }

        // Check for verification status
        if let Some(status) = data.get("verification_status").and_then(|v| v.as_str()) {
            match status {
                "verified" => score += 0.2,
                "pending" => score += 0.0,
                "failed" => score -= 0.3,
                _ => {}
            }
        }

        // Check for reliability score if available
        if let Some(reliability) = data.get("reliability_score").and_then(|v| v.as_f64()) {
            score = score * 0.7 + (reliability as f32) * 0.3;
        }

        Ok(score.min(1.0).max(0.0))
    }

    async fn assess_diversity_contribution(&self, data: &serde_json::Value) -> Result<f32> {
        let mut diversity_factors = 0;
        let mut total_factors = 0;

        // Language diversity
        total_factors += 1;
        if let Some(language) = data.get("metadata")
            .and_then(|m| m.get("language"))
            .and_then(|v| v.as_str())
        {
            // Bonus for less common languages
            match language.to_lowercase().as_str() {
                "rust" | "go" | "zig" => diversity_factors += 1,
                "typescript" | "javascript" => diversity_factors += 0, // Common, no bonus
                _ => diversity_factors += 1, // Other languages get bonus
            }
        }

        // Size diversity (stars)
        total_factors += 1;
        if let Some(stars) = data.get("stars").and_then(|v| v.as_i64()) {
            match stars {
                0..=100 => diversity_factors += 1,      // Small projects
                101..=1000 => diversity_factors += 0,   // Medium projects (common)
                1001..=10000 => diversity_factors += 1, // Large projects
                _ => diversity_factors += 1,            // Very large projects
            }
        }

        // Category-specific diversity
        total_factors += 1;
        if let Some(category) = data.get("category").and_then(|v| v.as_str()) {
            match category {
                "rust_ide" | "wasm_app" => diversity_factors += 1, // Less common categories
                "vscode_fork" | "ai_ide" => diversity_factors += 0, // More common categories
                _ => diversity_factors += 1,
            }
        }

        Ok(if total_factors > 0 {
            diversity_factors as f32 / total_factors as f32
        } else {
            0.5
        })
    }

    async fn calculate_diversity_score(&self, projects: &[serde_json::Value], required_attributes: &[String]) -> Result<f32> {
        if projects.is_empty() {
            return Ok(0.0);
        }

        let mut diversity_scores = Vec::new();

        // Language diversity
        let languages: std::collections::HashSet<String> = projects
            .iter()
            .filter_map(|p| {
                p.get("metadata")
                    .and_then(|m| m.get("language"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_lowercase())
            })
            .collect();
        diversity_scores.push(languages.len() as f32 / 5.0); // Normalize to max 5 languages

        // Size diversity (stars distribution)
        let star_ranges = projects
            .iter()
            .filter_map(|p| p.get("stars").and_then(|v| v.as_i64()))
            .fold([0; 4], |mut acc, stars| {
                match stars {
                    0..=100 => acc[0] += 1,
                    101..=1000 => acc[1] += 1,
                    1001..=10000 => acc[2] += 1,
                    _ => acc[3] += 1,
                }
                acc
            });
        let size_diversity = star_ranges.iter().filter(|&&count| count > 0).count() as f32 / 4.0;
        diversity_scores.push(size_diversity);

        // Attribute diversity
        for attr in required_attributes {
            let unique_values: std::collections::HashSet<String> = projects
                .iter()
                .filter_map(|p| {
                    p.get(attr)
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_lowercase())
                })
                .collect();
            diversity_scores.push((unique_values.len() as f32 / 3.0).min(1.0)); // Normalize to max 3 unique values
        }

        Ok(diversity_scores.iter().sum::<f32>() / diversity_scores.len() as f32)
    }

    async fn analyze_quality_distribution(&self, projects: &[serde_json::Value]) -> Result<QualityDistribution> {
        let mut quality_scores = Vec::new();

        for project in projects {
            let completeness = self.assess_completeness_quality(project).await?;
            let accuracy = self.assess_accuracy_quality(project).await?;
            let reliability = self.assess_reliability_quality(project).await?;
            
            let overall_quality = (completeness + accuracy + reliability) / 3.0;
            quality_scores.push(overall_quality);
        }

        let average_quality = if quality_scores.is_empty() {
            0.0
        } else {
            quality_scores.iter().sum::<f32>() / quality_scores.len() as f32
        };

        let high_quality_count = quality_scores.iter().filter(|&&score| score >= 0.8).count();
        let medium_quality_count = quality_scores.iter().filter(|&&score| score >= 0.6 && score < 0.8).count();
        let low_quality_count = quality_scores.iter().filter(|&&score| score < 0.6).count();

        Ok(QualityDistribution {
            average_quality,
            high_quality_count,
            medium_quality_count,
            low_quality_count,
        })
    }

    async fn check_attribute_coverage(&self, projects: &[serde_json::Value], required_attributes: &[String]) -> Result<f32> {
        if projects.is_empty() || required_attributes.is_empty() {
            return Ok(0.0);
        }

        let mut coverage_scores = Vec::new();

        for attr in required_attributes {
            let projects_with_attr = projects
                .iter()
                .filter(|p| p.get(attr).is_some())
                .count();
            
            let coverage = projects_with_attr as f32 / projects.len() as f32;
            coverage_scores.push(coverage);
        }

        Ok(coverage_scores.iter().sum::<f32>() / coverage_scores.len() as f32)
    }

    async fn calculate_source_diversity(&self, findings: &[serde_json::Value], sources: &[serde_json::Value]) -> Result<f32> {
        if findings.is_empty() || sources.is_empty() {
            return Ok(0.0);
        }

        // Create a map of source types
        let source_types: std::collections::HashMap<String, String> = sources
            .iter()
            .filter_map(|s| {
                let id = s.get("id").and_then(|v| v.as_str())?;
                let source_type = s.get("source_type").and_then(|v| v.as_str())?;
                Some((id.to_string(), source_type.to_string()))
            })
            .collect();

        // Analyze source diversity for findings
        let mut unique_source_types = std::collections::HashSet::new();
        let mut total_source_references = 0;

        for finding in findings {
            if let Some(source_ids) = finding.get("source_ids").and_then(|v| v.as_array()) {
                for source_id in source_ids {
                    if let Some(id_str) = source_id.as_str() {
                        if let Some(source_type) = source_types.get(id_str) {
                            unique_source_types.insert(source_type.clone());
                            total_source_references += 1;
                        }
                    }
                }
            }
        }

        // Calculate diversity score
        let type_diversity = unique_source_types.len() as f32 / 5.0; // Normalize to max 5 source types
        let reference_density = if findings.len() > 0 {
            (total_source_references as f32 / findings.len() as f32) / 3.0 // Normalize to max 3 sources per finding
        } else {
            0.0
        };

        Ok((type_diversity + reference_density) / 2.0)
    }

    async fn detect_contradictions(&self, findings: &[serde_json::Value]) -> Result<f32> {
        // This is a simplified contradiction detection
        // In a real implementation, this would use NLP techniques to detect semantic contradictions
        
        let mut contradiction_score = 1.0;
        let mut findings_by_category: std::collections::HashMap<String, Vec<&serde_json::Value>> = std::collections::HashMap::new();

        // Group findings by category
        for finding in findings {
            if let Some(category) = finding.get("category").and_then(|v| v.as_str()) {
                findings_by_category.entry(category.to_string()).or_insert_with(Vec::new).push(finding);
            }
        }

        // Check for contradictions within categories
        for (_, category_findings) in findings_by_category {
            if category_findings.len() > 1 {
                // Simple keyword-based contradiction detection
                let positive_keywords = ["good", "excellent", "high", "strong", "successful"];
                let negative_keywords = ["bad", "poor", "low", "weak", "failed"];

                let mut positive_count = 0;
                let mut negative_count = 0;

                for finding in &category_findings {
                    if let Some(description) = finding.get("description").and_then(|v| v.as_str()) {
                        let desc_lower = description.to_lowercase();
                        
                        for keyword in &positive_keywords {
                            if desc_lower.contains(keyword) {
                                positive_count += 1;
                                break;
                            }
                        }
                        
                        for keyword in &negative_keywords {
                            if desc_lower.contains(keyword) {
                                negative_count += 1;
                                break;
                            }
                        }
                    }
                }

                // If we have both positive and negative findings in the same category, it might be a contradiction
                if positive_count > 0 && negative_count > 0 {
                    contradiction_score -= 0.2;
                }
            }
        }

        Ok(contradiction_score.max(0.0))
    }

    async fn assess_evidence_strength(&self, findings: &[serde_json::Value], sources: &[serde_json::Value]) -> Result<f32> {
        if findings.is_empty() {
            return Ok(0.0);
        }

        let mut total_strength = 0.0;
        let mut finding_count = 0;

        // Create source reliability map
        let source_reliability: std::collections::HashMap<String, f32> = sources
            .iter()
            .filter_map(|s| {
                let id = s.get("id").and_then(|v| v.as_str())?;
                let reliability = s.get("reliability_score").and_then(|v| v.as_f64())? as f32;
                Some((id.to_string(), reliability))
            })
            .collect();

        for finding in findings {
            let mut finding_strength = 0.0;
            let mut source_count = 0;

            // Check confidence level
            if let Some(confidence) = finding.get("confidence_level").and_then(|v| v.as_str()) {
                finding_strength += match confidence {
                    "high" => 0.4,
                    "medium" => 0.25,
                    "low" => 0.1,
                    _ => 0.0,
                };
            }

            // Check source reliability
            if let Some(source_ids) = finding.get("source_ids").and_then(|v| v.as_array()) {
                let mut source_reliability_sum = 0.0;
                for source_id in source_ids {
                    if let Some(id_str) = source_id.as_str() {
                        if let Some(&reliability) = source_reliability.get(id_str) {
                            source_reliability_sum += reliability;
                            source_count += 1;
                        }
                    }
                }
                
                if source_count > 0 {
                    finding_strength += (source_reliability_sum / source_count as f32) * 0.4;
                }
            }

            // Check evidence richness
            if let Some(evidence) = finding.get("evidence").and_then(|v| v.as_object()) {
                let evidence_richness = (evidence.len() as f32 / 5.0).min(1.0); // Normalize to max 5 evidence items
                finding_strength += evidence_richness * 0.2;
            }

            total_strength += finding_strength;
            finding_count += 1;
        }

        Ok(if finding_count > 0 {
            total_strength / finding_count as f32
        } else {
            0.0
        })
    }
}

#[derive(Debug)]
struct QualityDistribution {
    average_quality: f32,
    high_quality_count: usize,
    medium_quality_count: usize,
    low_quality_count: usize,
}