use crate::models::{ValidationResult, ConfidenceLevel};
use crate::errors::Result;
use chrono::Utc;
use uuid::Uuid;
use std::collections::HashMap;

pub struct BiasDetector {
    bias_patterns: HashMap<String, BiasPattern>,
}

#[derive(Debug, Clone)]
pub struct BiasPattern {
    pub name: String,
    pub description: String,
    pub detection_rules: Vec<DetectionRule>,
    pub severity_threshold: f32,
    pub mitigation_strategies: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DetectionRule {
    pub field: String,
    pub condition: BiasCondition,
    pub weight: f32,
}

#[derive(Debug, Clone)]
pub enum BiasCondition {
    ValueConcentration { threshold: f32 },
    TemporalBias { days_threshold: i64 },
    SourceBias { dominant_source_threshold: f32 },
    PopularityBias { star_threshold: i64 },
    LanguageBias { language: String, percentage_threshold: f32 },
    GeographicBias { region_concentration_threshold: f32 },
}

impl BiasDetector {
    pub fn new() -> Self {
        let mut bias_patterns = HashMap::new();

        // Define bias detection patterns
        bias_patterns.insert("selection_bias".to_string(), Self::selection_bias_pattern());
        bias_patterns.insert("popularity_bias".to_string(), Self::popularity_bias_pattern());
        bias_patterns.insert("recency_bias".to_string(), Self::recency_bias_pattern());
        bias_patterns.insert("technology_bias".to_string(), Self::technology_bias_pattern());
        bias_patterns.insert("source_bias".to_string(), Self::source_bias_pattern());
        bias_patterns.insert("confirmation_bias".to_string(), Self::confirmation_bias_pattern());

        Self { bias_patterns }
    }

    /// Detect bias in research data
    pub async fn detect_bias(
        &self,
        entity_type: &str,
        entity_id: Uuid,
        entity_data: &serde_json::Value,
    ) -> Result<ValidationResult> {
        let mut detected_biases = Vec::new();
        let mut bias_score = 0.0;
        let mut total_weight = 0.0;

        // Run all bias detection patterns
        for (bias_type, pattern) in &self.bias_patterns {
            let bias_strength = self.evaluate_bias_pattern(entity_data, pattern).await?;
            
            if bias_strength > pattern.severity_threshold {
                detected_biases.push(format!(
                    "{}: {} (strength: {:.2})",
                    bias_type, pattern.description, bias_strength
                ));
            }

            bias_score += bias_strength;
            total_weight += 1.0;
        }

        let average_bias_score = if total_weight > 0.0 {
            bias_score / total_weight
        } else {
            0.0
        };

        // Generate recommendations
        let mut recommendations = Vec::new();
        for (bias_type, pattern) in &self.bias_patterns {
            let bias_strength = self.evaluate_bias_pattern(entity_data, pattern).await?;
            if bias_strength > pattern.severity_threshold {
                recommendations.extend(pattern.mitigation_strategies.clone());
            }
        }

        // Remove duplicate recommendations
        recommendations.sort();
        recommendations.dedup();

        let status = if detected_biases.is_empty() {
            "passed"
        } else if average_bias_score < 0.5 {
            "warning"
        } else {
            "failed"
        };

        // Invert score so higher is better (less bias)
        let quality_score = 1.0 - average_bias_score;

        Ok(ValidationResult {
            id: Uuid::new_v4(),
            entity_type: entity_type.to_string(),
            entity_id,
            validation_type: "bias_detection".to_string(),
            status: status.to_string(),
            score: Some(quality_score),
            issues: detected_biases,
            recommendations,
            validated_at: Utc::now(),
            validator: "BiasDetector::detect_bias".to_string(),
        })
    }

    /// Detect bias across a dataset
    pub async fn detect_dataset_bias(&self, dataset: &[serde_json::Value]) -> Result<ValidationResult> {
        let mut detected_biases = Vec::new();
        let mut recommendations = Vec::new();
        let mut bias_scores = Vec::new();

        // Analyze distribution biases
        let distribution_biases = self.analyze_distribution_bias(dataset).await?;
        detected_biases.extend(distribution_biases.0);
        recommendations.extend(distribution_biases.1);
        bias_scores.push(distribution_biases.2);

        // Analyze temporal biases
        let temporal_biases = self.analyze_temporal_bias(dataset).await?;
        detected_biases.extend(temporal_biases.0);
        recommendations.extend(temporal_biases.1);
        bias_scores.push(temporal_biases.2);

        // Analyze source biases
        let source_biases = self.analyze_source_bias(dataset).await?;
        detected_biases.extend(source_biases.0);
        recommendations.extend(source_biases.1);
        bias_scores.push(source_biases.2);

        let average_bias_score = if !bias_scores.is_empty() {
            bias_scores.iter().sum::<f32>() / bias_scores.len() as f32
        } else {
            0.0
        };

        let status = if detected_biases.is_empty() {
            "passed"
        } else if average_bias_score < 0.5 {
            "warning"
        } else {
            "failed"
        };

        let quality_score = 1.0 - average_bias_score;

        Ok(ValidationResult {
            id: Uuid::new_v4(),
            entity_type: "dataset".to_string(),
            entity_id: Uuid::new_v4(),
            validation_type: "dataset_bias_detection".to_string(),
            status: status.to_string(),
            score: Some(quality_score),
            issues: detected_biases,
            recommendations,
            validated_at: Utc::now(),
            validator: "BiasDetector::detect_dataset_bias".to_string(),
        })
    }

    /// Evaluate a specific bias pattern
    async fn evaluate_bias_pattern(&self, data: &serde_json::Value, pattern: &BiasPattern) -> Result<f32> {
        let mut pattern_score = 0.0;
        let mut total_weight = 0.0;

        for rule in &pattern.detection_rules {
            let rule_score = self.evaluate_detection_rule(data, rule).await?;
            pattern_score += rule_score * rule.weight;
            total_weight += rule.weight;
        }

        Ok(if total_weight > 0.0 {
            pattern_score / total_weight
        } else {
            0.0
        })
    }

    /// Evaluate a detection rule
    async fn evaluate_detection_rule(&self, data: &serde_json::Value, rule: &DetectionRule) -> Result<f32> {
        let field_value = data.get(&rule.field);

        match &rule.condition {
            BiasCondition::ValueConcentration { threshold } => {
                // This would need dataset context to evaluate properly
                // For single entity, return neutral score
                Ok(0.0)
            }
            BiasCondition::TemporalBias { days_threshold } => {
                if let Some(date_str) = field_value.and_then(|v| v.as_str()) {
                    if let Ok(date) = chrono::DateTime::parse_from_rfc3339(date_str) {
                        let days_ago = (Utc::now() - date.with_timezone(&Utc)).num_days();
                        if days_ago < *days_threshold {
                            Ok(0.8) // High recency bias
                        } else {
                            Ok(0.0)
                        }
                    } else {
                        Ok(0.0)
                    }
                } else {
                    Ok(0.0)
                }
            }
            BiasCondition::SourceBias { dominant_source_threshold: _ } => {
                // Check if source is from a dominant platform
                if let Some(url) = data.get("url").and_then(|v| v.as_str()) {
                    if url.contains("github.com") {
                        Ok(0.3) // Moderate GitHub bias
                    } else {
                        Ok(0.0)
                    }
                } else {
                    Ok(0.0)
                }
            }
            BiasCondition::PopularityBias { star_threshold } => {
                if let Some(stars) = field_value.and_then(|v| v.as_i64()) {
                    if stars > *star_threshold {
                        Ok(0.6) // Moderate popularity bias
                    } else {
                        Ok(0.0)
                    }
                } else {
                    Ok(0.0)
                }
            }
            BiasCondition::LanguageBias { language, percentage_threshold: _ } => {
                if let Some(metadata) = data.get("metadata").and_then(|v| v.as_object()) {
                    if let Some(proj_lang) = metadata.get("language").and_then(|v| v.as_str()) {
                        if proj_lang.to_lowercase() == language.to_lowercase() {
                            Ok(0.4) // Moderate language bias
                        } else {
                            Ok(0.0)
                        }
                    } else {
                        Ok(0.0)
                    }
                } else {
                    Ok(0.0)
                }
            }
            BiasCondition::GeographicBias { region_concentration_threshold: _ } => {
                // Would need geographic data to evaluate
                Ok(0.0)
            }
        }
    }

    /// Analyze distribution bias across dataset
    async fn analyze_distribution_bias(&self, dataset: &[serde_json::Value]) -> Result<(Vec<String>, Vec<String>, f32)> {
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();
        let mut bias_score = 0.0;

        if dataset.is_empty() {
            return Ok((issues, recommendations, 0.0));
        }

        // Analyze language distribution
        let mut language_counts: HashMap<String, usize> = HashMap::new();
        for item in dataset {
            if let Some(metadata) = item.get("metadata").and_then(|v| v.as_object()) {
                if let Some(language) = metadata.get("language").and_then(|v| v.as_str()) {
                    *language_counts.entry(language.to_lowercase()).or_insert(0) += 1;
                }
            }
        }

        if !language_counts.is_empty() {
            let total_projects = dataset.len();
            let max_language_count = *language_counts.values().max().unwrap_or(&0);
            let language_concentration = max_language_count as f32 / total_projects as f32;

            if language_concentration > 0.7 {
                issues.push(format!("High language concentration: {:.1}% of projects use the same language", language_concentration * 100.0));
                recommendations.push("Include projects from diverse programming languages".to_string());
                bias_score += 0.6;
            }
        }

        // Analyze star distribution
        let stars: Vec<i64> = dataset
            .iter()
            .filter_map(|item| item.get("stars").and_then(|v| v.as_i64()))
            .collect();

        if !stars.is_empty() {
            let high_star_count = stars.iter().filter(|&&s| s > 1000).count();
            let high_star_ratio = high_star_count as f32 / stars.len() as f32;

            if high_star_ratio > 0.8 {
                issues.push("High popularity bias: most projects have >1000 stars".to_string());
                recommendations.push("Include smaller, less popular projects for balanced perspective".to_string());
                bias_score += 0.5;
            }
        }

        Ok((issues, recommendations, bias_score / 2.0)) // Average the bias components
    }

    /// Analyze temporal bias
    async fn analyze_temporal_bias(&self, dataset: &[serde_json::Value]) -> Result<(Vec<String>, Vec<String>, f32)> {
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();
        let mut bias_score = 0.0;

        let dates: Vec<chrono::DateTime<Utc>> = dataset
            .iter()
            .filter_map(|item| {
                item.get("created_at")
                    .and_then(|v| v.as_str())
                    .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                    .map(|dt| dt.with_timezone(&Utc))
            })
            .collect();

        if !dates.is_empty() {
            let now = Utc::now();
            let recent_count = dates
                .iter()
                .filter(|&date| (now - *date).num_days() < 365)
                .count();

            let recent_ratio = recent_count as f32 / dates.len() as f32;

            if recent_ratio > 0.8 {
                issues.push("High recency bias: most projects are less than 1 year old".to_string());
                recommendations.push("Include mature projects with longer track records".to_string());
                bias_score += 0.6;
            }

            // Check for temporal clustering
            let mut year_counts: HashMap<i32, usize> = HashMap::new();
            for date in &dates {
                *year_counts.entry(date.year()).or_insert(0) += 1;
            }

            if let Some(max_year_count) = year_counts.values().max() {
                let year_concentration = *max_year_count as f32 / dates.len() as f32;
                if year_concentration > 0.6 {
                    issues.push("Temporal clustering: projects concentrated in single year".to_string());
                    recommendations.push("Distribute project selection across multiple years".to_string());
                    bias_score += 0.4;
                }
            }
        }

        Ok((issues, recommendations, bias_score))
    }

    /// Analyze source bias
    async fn analyze_source_bias(&self, dataset: &[serde_json::Value]) -> Result<(Vec<String>, Vec<String>, f32)> {
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();
        let mut bias_score = 0.0;

        // Analyze platform distribution
        let mut platform_counts: HashMap<String, usize> = HashMap::new();
        for item in dataset {
            if let Some(url) = item.get("url").and_then(|v| v.as_str()) {
                if let Ok(parsed_url) = url::Url::parse(url) {
                    if let Some(domain) = parsed_url.domain() {
                        *platform_counts.entry(domain.to_string()).or_insert(0) += 1;
                    }
                }
            }
        }

        if !platform_counts.is_empty() {
            let total_projects = dataset.len();
            let github_count = platform_counts.get("github.com").unwrap_or(&0);
            let github_ratio = *github_count as f32 / total_projects as f32;

            if github_ratio > 0.9 {
                issues.push("High platform bias: >90% of projects from GitHub".to_string());
                recommendations.push("Include projects from GitLab, Bitbucket, and other platforms".to_string());
                bias_score += 0.7;
            }
        }

        Ok((issues, recommendations, bias_score))
    }

    // Bias pattern definitions
    fn selection_bias_pattern() -> BiasPattern {
        BiasPattern {
            name: "Selection Bias".to_string(),
            description: "Systematic preference for certain types of projects".to_string(),
            detection_rules: vec![
                DetectionRule {
                    field: "url".to_string(),
                    condition: BiasCondition::SourceBias { dominant_source_threshold: 0.8 },
                    weight: 1.0,
                },
            ],
            severity_threshold: 0.5,
            mitigation_strategies: vec![
                "Diversify project sources beyond GitHub".to_string(),
                "Use systematic sampling methods".to_string(),
                "Include projects from different platforms and communities".to_string(),
            ],
        }
    }

    fn popularity_bias_pattern() -> BiasPattern {
        BiasPattern {
            name: "Popularity Bias".to_string(),
            description: "Over-representation of popular projects".to_string(),
            detection_rules: vec![
                DetectionRule {
                    field: "stars".to_string(),
                    condition: BiasCondition::PopularityBias { star_threshold: 1000 },
                    weight: 1.0,
                },
            ],
            severity_threshold: 0.4,
            mitigation_strategies: vec![
                "Include smaller, less popular projects".to_string(),
                "Balance popularity metrics with technical merit".to_string(),
                "Consider projects across different popularity ranges".to_string(),
            ],
        }
    }

    fn recency_bias_pattern() -> BiasPattern {
        BiasPattern {
            name: "Recency Bias".to_string(),
            description: "Over-representation of recent projects".to_string(),
            detection_rules: vec![
                DetectionRule {
                    field: "created_at".to_string(),
                    condition: BiasCondition::TemporalBias { days_threshold: 365 },
                    weight: 1.0,
                },
            ],
            severity_threshold: 0.6,
            mitigation_strategies: vec![
                "Include mature projects with longer track records".to_string(),
                "Balance new innovations with proven solutions".to_string(),
                "Consider project evolution over time".to_string(),
            ],
        }
    }

    fn technology_bias_pattern() -> BiasPattern {
        BiasPattern {
            name: "Technology Bias".to_string(),
            description: "Over-representation of specific technologies".to_string(),
            detection_rules: vec![
                DetectionRule {
                    field: "metadata".to_string(),
                    condition: BiasCondition::LanguageBias { 
                        language: "rust".to_string(), 
                        percentage_threshold: 0.7 
                    },
                    weight: 1.0,
                },
            ],
            severity_threshold: 0.5,
            mitigation_strategies: vec![
                "Include diverse technology stacks".to_string(),
                "Consider alternative implementation approaches".to_string(),
                "Balance preferred technologies with other valid options".to_string(),
            ],
        }
    }

    fn source_bias_pattern() -> BiasPattern {
        BiasPattern {
            name: "Source Bias".to_string(),
            description: "Over-reliance on specific information sources".to_string(),
            detection_rules: vec![
                DetectionRule {
                    field: "url".to_string(),
                    condition: BiasCondition::SourceBias { dominant_source_threshold: 0.8 },
                    weight: 1.0,
                },
            ],
            severity_threshold: 0.6,
            mitigation_strategies: vec![
                "Diversify information sources".to_string(),
                "Cross-reference findings across multiple platforms".to_string(),
                "Include academic and industry sources".to_string(),
            ],
        }
    }

    fn confirmation_bias_pattern() -> BiasPattern {
        BiasPattern {
            name: "Confirmation Bias".to_string(),
            description: "Preference for information that confirms existing beliefs".to_string(),
            detection_rules: vec![
                // This would need more sophisticated analysis
                // For now, use a simple heuristic
            ],
            severity_threshold: 0.7,
            mitigation_strategies: vec![
                "Actively seek contradictory evidence".to_string(),
                "Include diverse perspectives and opinions".to_string(),
                "Use structured evaluation criteria".to_string(),
                "Implement peer review processes".to_string(),
            ],
        }
    }
}