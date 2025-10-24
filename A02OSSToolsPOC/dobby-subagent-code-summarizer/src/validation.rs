//! Contract validation with TDD-First principles
//!
//! Executable Specifications:
//! - Validate processing results against TDD contracts
//! - Generate validation reports
//! - Track contract violations

use crate::config::SystemConfig;
use crate::results::ProcessingResults;
use crate::errors::Result;

/// Validation report with contract satisfaction metrics
#[derive(Debug)]
pub struct ValidationReport {
    pub all_contracts_satisfied: bool,
    pub violations: Vec<ContractViolation>,
    pub success_rate: f32,
    pub parallel_efficiency: f32,
    pub max_chunk_time_s: u64,
    pub memory_usage_mb: usize,
}

/// Contract violation details
#[derive(Debug, Clone)]
pub struct ContractViolation {
    pub contract_name: String,
    pub expected: String,
    pub actual: String,
    pub severity: ViolationSeverity,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ViolationSeverity {
    Critical,  // System cannot function
    High,      // Performance degraded significantly
    Medium,    // Acceptable but should be improved
    Low,       // Minor deviation
}

/// Contract validator trait for dependency injection
pub trait ContractValidator: Send + Sync {
    fn validate_processing_results(
        &self,
        results: &ProcessingResults,
        config: &SystemConfig,
    ) -> Result<ValidationReport>;
}

/// Default contract validator implementation
pub struct DefaultContractValidator;

impl ContractValidator for DefaultContractValidator {
    fn validate_processing_results(
        &self,
        results: &ProcessingResults,
        config: &SystemConfig,
    ) -> Result<ValidationReport> {
        let mut violations = Vec::new();

        // Contract 1: Success rate ≥95%
        let success_rate = results.successful_chunks as f32 / results.total_chunks as f32;
        if success_rate < 0.95 {
            violations.push(ContractViolation {
                contract_name: "Success Rate".to_string(),
                expected: "≥95%".to_string(),
                actual: format!("{:.1}%", success_rate * 100.0),
                severity: ViolationSeverity::Critical,
            });
        }

        // Contract 2: Parallel efficiency ≥ threshold
        if results.parallel_efficiency < config.min_parallel_efficiency {
            violations.push(ContractViolation {
                contract_name: "Parallel Efficiency".to_string(),
                expected: format!("≥{:.0}%", config.min_parallel_efficiency * 100.0),
                actual: format!("{:.1}%", results.parallel_efficiency * 100.0),
                severity: ViolationSeverity::High,
            });
        }

        // Contract 3: Max processing time per chunk
        let max_chunk_time = results.chunk_results
            .iter()
            .map(|r| r.processing_time.as_secs())
            .max()
            .unwrap_or(0);

        if max_chunk_time > config.max_chunk_processing_time_s {
            violations.push(ContractViolation {
                contract_name: "Max Chunk Processing Time".to_string(),
                expected: format!("≤{}s", config.max_chunk_processing_time_s),
                actual: format!("{}s", max_chunk_time),
                severity: ViolationSeverity::High,
            });
        }

        // Contract 4: Memory usage under limit
        if results.peak_memory_mb > config.max_memory_mb {
            violations.push(ContractViolation {
                contract_name: "Memory Usage".to_string(),
                expected: format!("≤{}MB", config.max_memory_mb),
                actual: format!("{}MB", results.peak_memory_mb),
                severity: ViolationSeverity::Critical,
            });
        }

        let all_contracts_satisfied = violations.is_empty();

        Ok(ValidationReport {
            all_contracts_satisfied,
            violations,
            success_rate,
            parallel_efficiency: results.parallel_efficiency,
            max_chunk_time_s: max_chunk_time,
            memory_usage_mb: results.peak_memory_mb,
        })
    }
}

impl ValidationReport {
    /// Generate human-readable report
    pub fn summary(&self) -> String {
        let status = if self.all_contracts_satisfied {
            "✅ ALL CONTRACTS SATISFIED"
        } else {
            "❌ CONTRACT VIOLATIONS DETECTED"
        };

        let mut report = format!(
            "{}\n\n\
             Performance Metrics:\n\
             - Success Rate: {:.1}%\n\
             - Parallel Efficiency: {:.1}%\n\
             - Max Chunk Time: {}s\n\
             - Memory Usage: {}MB\n",
            status,
            self.success_rate * 100.0,
            self.parallel_efficiency * 100.0,
            self.max_chunk_time_s,
            self.memory_usage_mb
        );

        if !self.violations.is_empty() {
            report.push_str("\nContract Violations:\n");
            for violation in &self.violations {
                report.push_str(&format!(
                    "- [{:?}] {}: expected {}, got {}\n",
                    violation.severity,
                    violation.contract_name,
                    violation.expected,
                    violation.actual
                ));
            }
        }

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::results::ChunkResult;
    use std::time::Duration;

    #[test]
    fn test_contract_validation_success() {
        // TDD-First: GREEN phase - all contracts satisfied
        let config = SystemConfig::test_config();
        let validator = DefaultContractValidator;

        let mut results = ProcessingResults::new(10);
        for i in 0..10 {
            results.add_result(ChunkResult {
                chunk_id: i,
                summary: format!("Summary {}", i),
                processing_time: Duration::from_millis(500),
                success: true,
                error_message: None,
            });
        }
        results.finalize();

        let report = validator.validate_processing_results(&results, &config).unwrap();
        assert!(report.all_contracts_satisfied);
        assert!(report.violations.is_empty());
    }

    #[test]
    fn test_contract_violation_detection() {
        // TDD-First: RED phase - detect violations
        let config = SystemConfig::test_config();
        let validator = DefaultContractValidator;

        let mut results = ProcessingResults::new(10);
        
        // Only 8 successful (80% success rate < 95% requirement)
        for i in 0..8 {
            results.add_result(ChunkResult {
                chunk_id: i,
                summary: format!("Summary {}", i),
                processing_time: Duration::from_millis(500),
                success: true,
                error_message: None,
            });
        }
        for i in 8..10 {
            results.add_result(ChunkResult {
                chunk_id: i,
                summary: String::new(),
                processing_time: Duration::from_millis(0),
                success: false,
                error_message: Some("Failed".to_string()),
            });
        }
        
        results.finalize();

        let report = validator.validate_processing_results(&results, &config).unwrap();
        assert!(!report.all_contracts_satisfied);
        assert!(!report.violations.is_empty());
        
        // Should have critical violation for success rate
        assert!(report.violations.iter().any(|v| 
            v.severity == ViolationSeverity::Critical && v.contract_name == "Success Rate"
        ));
    }
}
