use crate::models::{ValidationResult, ConfidenceLevel};
use crate::errors::{Result, ResearchError};
use chrono::Utc;
use uuid::Uuid;

pub struct DataValidator {
    completeness_rules: std::collections::HashMap<String, CompletenessRules>,
    accuracy_rules: std::collections::HashMap<String, AccuracyRules>,
}

#[derive(Debug, Clone)]
pub struct CompletenessRules {
    pub required_fields: Vec<String>,
    pub optional_fields: Vec<String>,
    pub min_field_lengths: std::collections::HashMap<String, usize>,
    pub field_formats: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct AccuracyRules {
    pub url_validation: bool,
    pub date_validation: bool,
    pub numeric_ranges: std::collections::HashMap<String, (f64, f64)>,
    pub enum_validations: std::collections::HashMap<String, Vec<String>>,
}

impl DataValidator {
    pub fn new() -> Self {
        let mut completeness_rules = std::collections::HashMap::new();
        let mut accuracy_rules = std::collections::HashMap::new();

        // Define rules for each entity type
        completeness_rules.insert("project".to_string(), Self::project_completeness_rules());
        completeness_rules.insert("technical_analysis".to_string(), Self::technical_analysis_completeness_rules());
        completeness_rules.insert("research_source".to_string(), Self::research_source_completeness_rules());
        completeness_rules.insert("research_finding".to_string(), Self::research_finding_completeness_rules());

        accuracy_rules.insert("project".to_string(), Self::project_accuracy_rules());
        accuracy_rules.insert("technical_analysis".to_string(), Self::technical_analysis_accuracy_rules());
        accuracy_rules.insert("research_source".to_string(), Self::research_source_accuracy_rules());
        accuracy_rules.insert("research_finding".to_string(), Self::research_finding_accuracy_rules());

        Self {
            completeness_rules,
            accuracy_rules,
        }
    }

    /// Validate data completeness for an entity
    pub async fn validate_completeness(
        &self,
        entity_type: &str,
        entity_id: Uuid,
        entity_data: &serde_json::Value,
    ) -> Result<ValidationResult> {
        let rules = self.completeness_rules.get(entity_type).ok_or_else(|| {
            ResearchError::Validation {
                field: "entity_type".to_string(),
                message: format!("No completeness rules defined for: {}", entity_type),
            }
        })?;

        let mut issues = Vec::new();
        let mut recommendations = Vec::new();
        let mut score = 100.0;
        let total_fields = rules.required_fields.len() + rules.optional_fields.len();

        // Check required fields
        for field in &rules.required_fields {
            if !self.field_exists_and_valid(entity_data, field) {
                issues.push(format!("Missing required field: {}", field));
                score -= 100.0 / total_fields as f32;
            }
        }

        // Check field lengths
        for (field, min_length) in &rules.min_field_lengths {
            if let Some(value) = entity_data.get(field) {
                if let Some(text) = value.as_str() {
                    if text.len() < *min_length {
                        issues.push(format!("Field '{}' too short: {} chars (min: {})", field, text.len(), min_length));
                        score -= 10.0;
                    }
                }
            }
        }

        // Check field formats
        for (field, format) in &rules.field_formats {
            if let Some(value) = entity_data.get(field) {
                if let Some(text) = value.as_str() {
                    if !self.validate_format(text, format) {
                        issues.push(format!("Field '{}' has invalid format: expected {}", field, format));
                        score -= 15.0;
                    }
                }
            }
        }

        // Generate recommendations
        for field in &rules.optional_fields {
            if !self.field_exists_and_valid(entity_data, field) {
                recommendations.push(format!("Consider adding optional field '{}' for better data quality", field));
            }
        }

        let status = if issues.is_empty() {
            "passed"
        } else if score > 70.0 {
            "warning"
        } else {
            "failed"
        };

        Ok(ValidationResult {
            id: Uuid::new_v4(),
            entity_type: entity_type.to_string(),
            entity_id,
            validation_type: "completeness".to_string(),
            status: status.to_string(),
            score: Some(score.max(0.0) / 100.0),
            issues,
            recommendations,
            validated_at: Utc::now(),
            validator: "DataValidator::completeness".to_string(),
        })
    }

    /// Validate data accuracy for an entity
    pub async fn validate_accuracy(
        &self,
        entity_type: &str,
        entity_id: Uuid,
        entity_data: &serde_json::Value,
    ) -> Result<ValidationResult> {
        let rules = self.accuracy_rules.get(entity_type).ok_or_else(|| {
            ResearchError::Validation {
                field: "entity_type".to_string(),
                message: format!("No accuracy rules defined for: {}", entity_type),
            }
        })?;

        let mut issues = Vec::new();
        let mut recommendations = Vec::new();
        let mut score = 100.0;

        // URL validation
        if rules.url_validation {
            if let Some(url) = entity_data.get("url").and_then(|v| v.as_str()) {
                if !self.validate_url(url) {
                    issues.push(format!("Invalid URL format: {}", url));
                    score -= 20.0;
                }
            }
        }

        // Date validation
        if rules.date_validation {
            for field in ["created_at", "updated_at", "published_date", "last_updated"] {
                if let Some(date_str) = entity_data.get(field).and_then(|v| v.as_str()) {
                    if !self.validate_date(date_str) {
                        issues.push(format!("Invalid date format in field '{}': {}", field, date_str));
                        score -= 10.0;
                    }
                }
            }
        }

        // Numeric range validation
        for (field, (min_val, max_val)) in &rules.numeric_ranges {
            if let Some(value) = entity_data.get(field) {
                if let Some(num) = value.as_f64() {
                    if num < *min_val || num > *max_val {
                        issues.push(format!("Field '{}' out of range: {} (expected: {}-{})", field, num, min_val, max_val));
                        score -= 15.0;
                    }
                }
            }
        }

        // Enum validation
        for (field, valid_values) in &rules.enum_validations {
            if let Some(value) = entity_data.get(field).and_then(|v| v.as_str()) {
                if !valid_values.contains(&value.to_string()) {
                    issues.push(format!("Field '{}' has invalid value: {} (expected one of: {:?})", field, value, valid_values));
                    score -= 20.0;
                }
            }
        }

        // Cross-field validation
        self.validate_cross_field_consistency(entity_data, &mut issues, &mut score);

        // Generate recommendations
        if score < 90.0 {
            recommendations.push("Review data entry processes to improve accuracy".to_string());
        }
        if issues.iter().any(|i| i.contains("URL")) {
            recommendations.push("Implement URL validation at data entry point".to_string());
        }

        let status = if issues.is_empty() {
            "passed"
        } else if score > 70.0 {
            "warning"
        } else {
            "failed"
        };

        Ok(ValidationResult {
            id: Uuid::new_v4(),
            entity_type: entity_type.to_string(),
            entity_id,
            validation_type: "accuracy".to_string(),
            status: status.to_string(),
            score: Some(score.max(0.0) / 100.0),
            issues,
            recommendations,
            validated_at: Utc::now(),
            validator: "DataValidator::accuracy".to_string(),
        })
    }

    /// Check if field exists and has valid content
    fn field_exists_and_valid(&self, data: &serde_json::Value, field: &str) -> bool {
        match data.get(field) {
            Some(serde_json::Value::Null) => false,
            Some(serde_json::Value::String(s)) => !s.is_empty(),
            Some(serde_json::Value::Array(arr)) => !arr.is_empty(),
            Some(serde_json::Value::Object(obj)) => !obj.is_empty(),
            Some(_) => true,
            None => false,
        }
    }

    /// Validate field format
    fn validate_format(&self, value: &str, format: &str) -> bool {
        match format {
            "url" => self.validate_url(value),
            "email" => value.contains('@') && value.contains('.'),
            "uuid" => Uuid::parse_str(value).is_ok(),
            "date" => self.validate_date(value),
            _ => true, // Unknown format, assume valid
        }
    }

    /// Validate URL format
    fn validate_url(&self, url: &str) -> bool {
        url::Url::parse(url).is_ok()
    }

    /// Validate date format
    fn validate_date(&self, date_str: &str) -> bool {
        chrono::DateTime::parse_from_rfc3339(date_str).is_ok()
    }

    /// Validate cross-field consistency
    fn validate_cross_field_consistency(&self, data: &serde_json::Value, issues: &mut Vec<String>, score: &mut f32) {
        // Check created_at vs updated_at
        if let (Some(created), Some(updated)) = (
            data.get("created_at").and_then(|v| v.as_str()),
            data.get("updated_at").and_then(|v| v.as_str()),
        ) {
            if let (Ok(created_dt), Ok(updated_dt)) = (
                chrono::DateTime::parse_from_rfc3339(created),
                chrono::DateTime::parse_from_rfc3339(updated),
            ) {
                if updated_dt < created_dt {
                    issues.push("updated_at is before created_at".to_string());
                    *score -= 10.0;
                }
            }
        }

        // Check stars vs forks ratio (for projects)
        if let (Some(stars), Some(forks)) = (
            data.get("stars").and_then(|v| v.as_i64()),
            data.get("metadata").and_then(|m| m.get("forks_count")).and_then(|v| v.as_i64()),
        ) {
            if forks > stars * 2 {
                issues.push("Unusual forks to stars ratio - may indicate data quality issue".to_string());
                *score -= 5.0;
            }
        }

        // Check confidence level consistency
        if let Some(confidence) = data.get("confidence_level").and_then(|v| v.as_str()) {
            if let Some(score_val) = data.get("score").and_then(|v| v.as_f64()) {
                let expected_confidence = match score_val {
                    s if s >= 0.8 => "high",
                    s if s >= 0.6 => "medium",
                    _ => "low",
                };
                if confidence != expected_confidence {
                    issues.push(format!("Confidence level '{}' inconsistent with score {}", confidence, score_val));
                    *score -= 5.0;
                }
            }
        }
    }

    // Completeness rules definitions
    fn project_completeness_rules() -> CompletenessRules {
        let mut min_field_lengths = std::collections::HashMap::new();
        min_field_lengths.insert("name".to_string(), 3);
        min_field_lengths.insert("description".to_string(), 20);

        let mut field_formats = std::collections::HashMap::new();
        field_formats.insert("url".to_string(), "url".to_string());
        field_formats.insert("id".to_string(), "uuid".to_string());

        CompletenessRules {
            required_fields: vec![
                "id".to_string(),
                "name".to_string(),
                "url".to_string(),
                "category".to_string(),
                "status".to_string(),
            ],
            optional_fields: vec![
                "description".to_string(),
                "stars".to_string(),
                "contributors".to_string(),
                "last_updated".to_string(),
                "metadata".to_string(),
            ],
            min_field_lengths,
            field_formats,
        }
    }

    fn technical_analysis_completeness_rules() -> CompletenessRules {
        let mut min_field_lengths = std::collections::HashMap::new();
        min_field_lengths.insert("analysis_notes".to_string(), 50);

        let mut field_formats = std::collections::HashMap::new();
        field_formats.insert("id".to_string(), "uuid".to_string());
        field_formats.insert("project_id".to_string(), "uuid".to_string());

        CompletenessRules {
            required_fields: vec![
                "id".to_string(),
                "project_id".to_string(),
                "applicability_to_kiro".to_string(),
                "confidence_level".to_string(),
            ],
            optional_fields: vec![
                "source_technology".to_string(),
                "target_technology".to_string(),
                "migration_approach".to_string(),
                "architecture_patterns".to_string(),
                "performance_metrics".to_string(),
                "strengths".to_string(),
                "weaknesses".to_string(),
                "lessons_learned".to_string(),
                "analysis_notes".to_string(),
            ],
            min_field_lengths,
            field_formats,
        }
    }

    fn research_source_completeness_rules() -> CompletenessRules {
        let mut min_field_lengths = std::collections::HashMap::new();
        min_field_lengths.insert("title".to_string(), 5);

        let mut field_formats = std::collections::HashMap::new();
        field_formats.insert("id".to_string(), "uuid".to_string());
        field_formats.insert("url".to_string(), "url".to_string());

        CompletenessRules {
            required_fields: vec![
                "id".to_string(),
                "url".to_string(),
                "source_type".to_string(),
                "verification_status".to_string(),
            ],
            optional_fields: vec![
                "project_id".to_string(),
                "title".to_string(),
                "author".to_string(),
                "published_date".to_string(),
                "reliability_score".to_string(),
                "content_hash".to_string(),
            ],
            min_field_lengths,
            field_formats,
        }
    }

    fn research_finding_completeness_rules() -> CompletenessRules {
        let mut min_field_lengths = std::collections::HashMap::new();
        min_field_lengths.insert("title".to_string(), 10);
        min_field_lengths.insert("description".to_string(), 50);

        let mut field_formats = std::collections::HashMap::new();
        field_formats.insert("id".to_string(), "uuid".to_string());

        CompletenessRules {
            required_fields: vec![
                "id".to_string(),
                "category".to_string(),
                "title".to_string(),
                "description".to_string(),
                "confidence_level".to_string(),
            ],
            optional_fields: vec![
                "project_id".to_string(),
                "evidence".to_string(),
                "tags".to_string(),
                "source_ids".to_string(),
            ],
            min_field_lengths,
            field_formats,
        }
    }

    // Accuracy rules definitions
    fn project_accuracy_rules() -> AccuracyRules {
        let mut numeric_ranges = std::collections::HashMap::new();
        numeric_ranges.insert("stars".to_string(), (0.0, 1_000_000.0));
        numeric_ranges.insert("contributors".to_string(), (0.0, 10_000.0));

        let mut enum_validations = std::collections::HashMap::new();
        enum_validations.insert("category".to_string(), vec![
            "ide_migration".to_string(),
            "vscode_fork".to_string(),
            "rust_ide".to_string(),
            "ai_ide".to_string(),
            "wasm_app".to_string(),
            "technical_pattern".to_string(),
        ]);
        enum_validations.insert("status".to_string(), vec![
            "discovered".to_string(),
            "analyzing".to_string(),
            "analyzed".to_string(),
            "verified".to_string(),
            "archived".to_string(),
        ]);

        AccuracyRules {
            url_validation: true,
            date_validation: true,
            numeric_ranges,
            enum_validations,
        }
    }

    fn technical_analysis_accuracy_rules() -> AccuracyRules {
        let mut enum_validations = std::collections::HashMap::new();
        enum_validations.insert("applicability_to_kiro".to_string(), vec![
            "high".to_string(),
            "medium".to_string(),
            "low".to_string(),
        ]);
        enum_validations.insert("confidence_level".to_string(), vec![
            "high".to_string(),
            "medium".to_string(),
            "low".to_string(),
        ]);

        AccuracyRules {
            url_validation: false,
            date_validation: true,
            numeric_ranges: std::collections::HashMap::new(),
            enum_validations,
        }
    }

    fn research_source_accuracy_rules() -> AccuracyRules {
        let mut numeric_ranges = std::collections::HashMap::new();
        numeric_ranges.insert("reliability_score".to_string(), (0.0, 1.0));

        let mut enum_validations = std::collections::HashMap::new();
        enum_validations.insert("verification_status".to_string(), vec![
            "verified".to_string(),
            "pending".to_string(),
            "failed".to_string(),
        ]);

        AccuracyRules {
            url_validation: true,
            date_validation: true,
            numeric_ranges,
            enum_validations,
        }
    }

    fn research_finding_accuracy_rules() -> AccuracyRules {
        let mut enum_validations = std::collections::HashMap::new();
        enum_validations.insert("confidence_level".to_string(), vec![
            "high".to_string(),
            "medium".to_string(),
            "low".to_string(),
        ]);
        enum_validations.insert("category".to_string(), vec![
            "pattern".to_string(),
            "success_factor".to_string(),
            "failure_mode".to_string(),
            "recommendation".to_string(),
            "technical_insight".to_string(),
            "business_insight".to_string(),
        ]);

        AccuracyRules {
            url_validation: false,
            date_validation: true,
            numeric_ranges: std::collections::HashMap::new(),
            enum_validations,
        }
    }
}