pub mod data_validator;
pub mod quality_assurance;
pub mod bias_detector;
pub mod source_verifier;

use crate::models::{ValidationResult, ConfidenceLevel};
use crate::errors::Result;
use uuid::Uuid;

pub use data_validator::DataValidator;
pub use quality_assurance::QualityAssurance;
pub use bias_detector::BiasDetector;
pub use source_verifier::SourceVerifier;

/// Comprehensive validation framework for research data
pub struct ValidationFramework {
    data_validator: DataValidator,
    quality_assurance: QualityAssurance,
    bias_detector: BiasDetector,
    source_verifier: SourceVerifier,
}

impl ValidationFramework {
    pub fn new() -> Self {
        Self {
            data_validator: DataValidator::new(),
            quality_assurance: QualityAssurance::new(),
            bias_detector: BiasDetector::new(),
            source_verifier: SourceVerifier::new(),
        }
    }

    /// Run comprehensive validation on an entity
    pub async fn validate_entity(
        &self,
        entity_type: &str,
        entity_id: Uuid,
        entity_data: &serde_json::Value,
    ) -> Result<Vec<ValidationResult>> {
        let mut results = Vec::new();

        // Data completeness validation
        let completeness_result = self
            .data_validator
            .validate_completeness(entity_type, entity_id, entity_data)
            .await?;
        results.push(completeness_result);

        // Data accuracy validation
        let accuracy_result = self
            .data_validator
            .validate_accuracy(entity_type, entity_id, entity_data)
            .await?;
        results.push(accuracy_result);

        // Quality assurance checks
        let quality_result = self
            .quality_assurance
            .assess_quality(entity_type, entity_id, entity_data)
            .await?;
        results.push(quality_result);

        // Bias detection
        let bias_result = self
            .bias_detector
            .detect_bias(entity_type, entity_id, entity_data)
            .await?;
        results.push(bias_result);

        // Source verification (if applicable)
        if entity_type == "project" || entity_type == "research_source" {
            let verification_result = self
                .source_verifier
                .verify_source(entity_type, entity_id, entity_data)
                .await?;
            results.push(verification_result);
        }

        Ok(results)
    }

    /// Validate research dataset for completeness and coverage
    pub async fn validate_research_coverage(
        &self,
        category: &str,
        target_count: i32,
        current_projects: &[serde_json::Value],
    ) -> Result<ValidationResult> {
        self.quality_assurance
            .validate_research_coverage(category, target_count, current_projects)
            .await
    }

    /// Cross-validate findings across multiple sources
    pub async fn cross_validate_findings(
        &self,
        findings: &[serde_json::Value],
        sources: &[serde_json::Value],
    ) -> Result<ValidationResult> {
        self.quality_assurance
            .cross_validate_findings(findings, sources)
            .await
    }

    /// Generate validation summary report
    pub async fn generate_validation_summary(
        &self,
        validation_results: &[ValidationResult],
    ) -> Result<serde_json::Value> {
        let mut summary = serde_json::json!({
            "total_validations": validation_results.len(),
            "passed": 0,
            "failed": 0,
            "warnings": 0,
            "average_score": 0.0,
            "critical_issues": [],
            "recommendations": []
        });

        let mut total_score = 0.0;
        let mut score_count = 0;

        for result in validation_results {
            match result.status.as_str() {
                "passed" => summary["passed"] = serde_json::json!(summary["passed"].as_i64().unwrap_or(0) + 1),
                "failed" => summary["failed"] = serde_json::json!(summary["failed"].as_i64().unwrap_or(0) + 1),
                "warning" => summary["warnings"] = serde_json::json!(summary["warnings"].as_i64().unwrap_or(0) + 1),
                _ => {}
            }

            if let Some(score) = result.score {
                total_score += score;
                score_count += 1;
            }

            // Collect critical issues
            if result.status == "failed" && !result.issues.is_empty() {
                for issue in &result.issues {
                    summary["critical_issues"]
                        .as_array_mut()
                        .unwrap()
                        .push(serde_json::json!({
                            "entity_type": result.entity_type,
                            "entity_id": result.entity_id,
                            "validation_type": result.validation_type,
                            "issue": issue
                        }));
                }
            }

            // Collect recommendations
            for recommendation in &result.recommendations {
                summary["recommendations"]
                    .as_array_mut()
                    .unwrap()
                    .push(serde_json::json!(recommendation));
            }
        }

        if score_count > 0 {
            summary["average_score"] = serde_json::json!(total_score / score_count as f32);
        }

        Ok(summary)
    }
}