// Migration Analyzer for IDE Migration Research
// Analyzes migration technical approaches, outcomes, and patterns

use super::{
    MigrationProject, TechnicalDetails, MigrationStrategy, TechnicalChallenge,
    SourceTechnology, TargetTechnology, MigrationApproach, ChallengeOutcome,
    ResearchResult, ResearchError, PerformanceMetric
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MigrationAnalyzer {
    analysis_frameworks: Vec<AnalysisFramework>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisFramework {
    pub name: String,
    pub criteria: Vec<AnalysisCriterion>,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisCriterion {
    pub name: String,
    pub description: String,
    pub weight: f64,
    pub evaluation_method: EvaluationMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvaluationMethod {
    Quantitative { min_value: f64, max_value: f64 },
    Qualitative { scale: Vec<String> },
    Boolean,
    Categorical { categories: Vec<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationAnalysisResult {
    pub project_name: String,
    pub technical_approach_score: f64,
    pub migration_complexity: MigrationComplexity,
    pub success_indicators: Vec<SuccessIndicator>,
    pub risk_factors: Vec<RiskFactor>,
    pub lessons_learned: Vec<LessonLearned>,
    pub applicability_assessment: ApplicabilityAssessment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MigrationComplexity {
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessIndicator {
    pub category: String,
    pub description: String,
    pub evidence: Vec<String>,
    pub confidence_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    pub category: String,
    pub description: String,
    pub severity: RiskSeverity,
    pub mitigation_strategies: Vec<String>,
    pub observed_outcomes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LessonLearned {
    pub category: String,
    pub lesson: String,
    pub context: String,
    pub applicability: f64,
    pub source_projects: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicabilityAssessment {
    pub overall_score: f64,
    pub technical_similarity: f64,
    pub architectural_alignment: f64,
    pub resource_requirements_match: f64,
    pub risk_profile_compatibility: f64,
    pub recommendations: Vec<String>,
}

impl MigrationAnalyzer {
    pub fn new() -> Self {
        Self {
            analysis_frameworks: Self::create_default_frameworks(),
        }
    }

    fn create_default_frameworks() -> Vec<AnalysisFramework> {
        vec![
            // Technical Approach Analysis Framework
            AnalysisFramework {
                name: "Technical Approach".to_string(),
                criteria: vec![
                    AnalysisCriterion {
                        name: "Technology Stack Maturity".to_string(),
                        description: "Maturity and stability of target technology stack".to_string(),
                        weight: 0.25,
                        evaluation_method: EvaluationMethod::Qualitative {
                            scale: vec!["Experimental".to_string(), "Beta".to_string(), "Stable".to_string(), "Mature".to_string()]
                        },
                    },
                    AnalysisCriterion {
                        name: "Performance Improvement".to_string(),
                        description: "Measurable performance gains from migration".to_string(),
                        weight: 0.30,
                        evaluation_method: EvaluationMethod::Quantitative { min_value: 0.0, max_value: 10.0 },
                    },
                    AnalysisCriterion {
                        name: "Development Velocity Impact".to_string(),
                        description: "Impact on development speed and productivity".to_string(),
                        weight: 0.20,
                        evaluation_method: EvaluationMethod::Qualitative {
                            scale: vec!["Significantly Slower".to_string(), "Slower".to_string(), "Neutral".to_string(), "Faster".to_string(), "Significantly Faster".to_string()]
                        },
                    },
                    AnalysisCriterion {
                        name: "Ecosystem Compatibility".to_string(),
                        description: "Compatibility with existing tools and extensions".to_string(),
                        weight: 0.25,
                        evaluation_method: EvaluationMethod::Quantitative { min_value: 0.0, max_value: 100.0 },
                    },
                ],
                weight: 0.4,
            },

            // Migration Strategy Analysis Framework
            AnalysisFramework {
                name: "Migration Strategy".to_string(),
                criteria: vec![
                    AnalysisCriterion {
                        name: "Migration Approach Effectiveness".to_string(),
                        description: "Effectiveness of chosen migration approach".to_string(),
                        weight: 0.35,
                        evaluation_method: EvaluationMethod::Categorical {
                            categories: vec!["Big Bang".to_string(), "Gradual".to_string(), "Hybrid".to_string(), "Incremental".to_string()]
                        },
                    },
                    AnalysisCriterion {
                        name: "Timeline Adherence".to_string(),
                        description: "Adherence to planned migration timeline".to_string(),
                        weight: 0.25,
                        evaluation_method: EvaluationMethod::Quantitative { min_value: 0.0, max_value: 100.0 },
                    },
                    AnalysisCriterion {
                        name: "Resource Efficiency".to_string(),
                        description: "Efficiency of resource utilization during migration".to_string(),
                        weight: 0.25,
                        evaluation_method: EvaluationMethod::Qualitative {
                            scale: vec!["Poor".to_string(), "Fair".to_string(), "Good".to_string(), "Excellent".to_string()]
                        },
                    },
                    AnalysisCriterion {
                        name: "Risk Management".to_string(),
                        description: "Effectiveness of risk identification and mitigation".to_string(),
                        weight: 0.15,
                        evaluation_method: EvaluationMethod::Qualitative {
                            scale: vec!["Poor".to_string(), "Fair".to_string(), "Good".to_string(), "Excellent".to_string()]
                        },
                    },
                ],
                weight: 0.3,
            },

            // Outcome Analysis Framework
            AnalysisFramework {
                name: "Migration Outcomes".to_string(),
                criteria: vec![
                    AnalysisCriterion {
                        name: "User Satisfaction".to_string(),
                        description: "User satisfaction with migrated application".to_string(),
                        weight: 0.30,
                        evaluation_method: EvaluationMethod::Quantitative { min_value: 1.0, max_value: 10.0 },
                    },
                    AnalysisCriterion {
                        name: "Stability and Reliability".to_string(),
                        description: "Stability and reliability of migrated application".to_string(),
                        weight: 0.25,
                        evaluation_method: EvaluationMethod::Quantitative { min_value: 0.0, max_value: 100.0 },
                    },
                    AnalysisCriterion {
                        name: "Maintenance Burden".to_string(),
                        description: "Ongoing maintenance requirements".to_string(),
                        weight: 0.20,
                        evaluation_method: EvaluationMethod::Qualitative {
                            scale: vec!["Very High".to_string(), "High".to_string(), "Medium".to_string(), "Low".to_string(), "Very Low".to_string()]
                        },
                    },
                    AnalysisCriterion {
                        name: "Feature Parity".to_string(),
                        description: "Feature parity with original application".to_string(),
                        weight: 0.25,
                        evaluation_method: EvaluationMethod::Quantitative { min_value: 0.0, max_value: 100.0 },
                    },
                ],
                weight: 0.3,
            },
        ]
    }

    /// Analyze a migration project's technical approach and outcomes
    pub async fn analyze_migration_project(&self, project: &MigrationProject) -> ResearchResult<MigrationAnalysisResult> {
        let technical_approach_score = self.calculate_technical_approach_score(project).await?;
        let migration_complexity = self.assess_migration_complexity(project);
        let success_indicators = self.identify_success_indicators(project).await?;
        let risk_factors = self.identify_risk_factors(project).await?;
        let lessons_learned = self.extract_lessons_learned(project).await?;
        let applicability_assessment = self.assess_applicability_to_kiro(project).await?;

        Ok(MigrationAnalysisResult {
            project_name: project.name.clone(),
            technical_approach_score,
            migration_complexity,
            success_indicators,
            risk_factors,
            lessons_learned,
            applicability_assessment,
        })
    }

    async fn calculate_technical_approach_score(&self, project: &MigrationProject) -> ResearchResult<f64> {
        let mut total_score = 0.0;
        let mut total_weight = 0.0;

        for framework in &self.analysis_frameworks {
            if framework.name == "Technical Approach" {
                let framework_score = self.evaluate_framework(project, framework).await?;
                total_score += framework_score * framework.weight;
                total_weight += framework.weight;
            }
        }

        Ok(if total_weight > 0.0 { total_score / total_weight } else { 0.0 })
    }

    async fn evaluate_framework(&self, project: &MigrationProject, framework: &AnalysisFramework) -> ResearchResult<f64> {
        let mut framework_score = 0.0;
        let mut total_weight = 0.0;

        for criterion in &framework.criteria {
            let criterion_score = self.evaluate_criterion(project, criterion).await?;
            framework_score += criterion_score * criterion.weight;
            total_weight += criterion.weight;
        }

        Ok(if total_weight > 0.0 { framework_score / total_weight } else { 0.0 })
    }

    async fn evaluate_criterion(&self, project: &MigrationProject, criterion: &AnalysisCriterion) -> ResearchResult<f64> {
        match &criterion.name[..] {
            "Technology Stack Maturity" => self.evaluate_technology_maturity(project),
            "Performance Improvement" => self.evaluate_performance_improvement(project),
            "Development Velocity Impact" => self.evaluate_development_velocity(project),
            "Ecosystem Compatibility" => self.evaluate_ecosystem_compatibility(project),
            "Migration Approach Effectiveness" => self.evaluate_migration_approach(project),
            "Timeline Adherence" => self.evaluate_timeline_adherence(project),
            "Resource Efficiency" => self.evaluate_resource_efficiency(project),
            "Risk Management" => self.evaluate_risk_management(project),
            "User Satisfaction" => self.evaluate_user_satisfaction(project),
            "Stability and Reliability" => self.evaluate_stability(project),
            "Maintenance Burden" => self.evaluate_maintenance_burden(project),
            "Feature Parity" => self.evaluate_feature_parity(project),
            _ => Ok(5.0), // Default neutral score
        }
    }

    fn evaluate_technology_maturity(&self, project: &MigrationProject) -> ResearchResult<f64> {
        if let Some(tech_details) = &project.technical_details {
            match tech_details.target_technology {
                TargetTechnology::Rust => Ok(8.0), // Rust is mature for systems programming
                TargetTechnology::Cpp => Ok(9.0),  // C++ is very mature
                TargetTechnology::Go => Ok(7.0),   // Go is mature but newer
                TargetTechnology::Wasm => Ok(6.0), // WASM is stable but still evolving
                TargetTechnology::Native => Ok(9.0), // Native is most mature
                TargetTechnology::Tauri => Ok(6.0), // Tauri is newer but promising
                TargetTechnology::Flutter => Ok(7.0), // Flutter is mature for UI
            }
        } else {
            Ok(5.0) // Default score when no technical details available
        }
    }

    fn evaluate_performance_improvement(&self, project: &MigrationProject) -> ResearchResult<f64> {
        if let Some(tech_details) = &project.technical_details {
            let mut improvement_score = 0.0;
            let mut metric_count = 0;

            for (metric_name, metric) in &tech_details.performance_metrics {
                if let Some(improvement) = metric.improvement_percentage {
                    improvement_score += improvement.min(100.0).max(0.0) / 10.0; // Scale to 0-10
                    metric_count += 1;
                }
            }

            if metric_count > 0 {
                Ok(improvement_score / metric_count as f64)
            } else {
                // Estimate based on technology stack
                match tech_details.target_technology {
                    TargetTechnology::Rust => Ok(8.0), // Rust typically shows good performance
                    TargetTechnology::Cpp => Ok(9.0),  // C++ typically best performance
                    TargetTechnology::Go => Ok(6.0),   // Go good but not as fast as Rust/C++
                    TargetTechnology::Wasm => Ok(7.0), // WASM can be quite fast
                    TargetTechnology::Native => Ok(9.0), // Native typically fastest
                    TargetTechnology::Tauri => Ok(7.0), // Tauri shows good performance
                    TargetTechnology::Flutter => Ok(6.0), // Flutter decent performance
                }
            }
        } else {
            Ok(5.0)
        }
    }

    fn evaluate_development_velocity(&self, project: &MigrationProject) -> ResearchResult<f64> {
        // This would typically require analysis of commit history, issue resolution times, etc.
        // For now, provide estimates based on technology and project characteristics
        if let Some(tech_details) = &project.technical_details {
            match tech_details.target_technology {
                TargetTechnology::Rust => Ok(6.0), // Rust has learning curve but good tooling
                TargetTechnology::Cpp => Ok(4.0),  // C++ can be slower to develop
                TargetTechnology::Go => Ok(8.0),   // Go designed for fast development
                TargetTechnology::Wasm => Ok(5.0), // WASM tooling still maturing
                TargetTechnology::Native => Ok(5.0), // Depends on platform
                TargetTechnology::Tauri => Ok(7.0), // Tauri aims for good DX
                TargetTechnology::Flutter => Ok(7.0), // Flutter has good DX
            }
        } else {
            Ok(5.0)
        }
    }

    fn evaluate_ecosystem_compatibility(&self, project: &MigrationProject) -> ResearchResult<f64> {
        match project.category {
            super::ProjectCategory::VsCodeFork => Ok(9.0), // VS Code forks maintain high compatibility
            super::ProjectCategory::IdeMigration => Ok(6.0), // Migration projects vary
            super::ProjectCategory::RustIde => Ok(5.0), // Rust IDEs often have limited compatibility
            super::ProjectCategory::AiIde => Ok(7.0), // AI IDEs often maintain some compatibility
            super::ProjectCategory::WasmApp => Ok(6.0), // WASM apps vary in compatibility
        }
    }

    fn evaluate_migration_approach(&self, project: &MigrationProject) -> ResearchResult<f64> {
        if let Some(tech_details) = &project.technical_details {
            match tech_details.migration_approach {
                MigrationApproach::Gradual => Ok(8.0), // Gradual is often safer
                MigrationApproach::CompleteRewrite => Ok(6.0), // Risky but can be effective
                MigrationApproach::Hybrid => Ok(7.0), // Balanced approach
                MigrationApproach::Incremental => Ok(9.0), // Often most effective
            }
        } else {
            Ok(5.0)
        }
    }

    fn evaluate_timeline_adherence(&self, project: &MigrationProject) -> ResearchResult<f64> {
        // This would require detailed project timeline analysis
        // For now, provide estimates based on project characteristics
        if project.status == super::ProjectStatus::Active {
            Ok(7.0) // Active projects likely meeting timelines
        } else if project.status == super::ProjectStatus::Archived {
            Ok(5.0) // Archived projects may have had timeline issues
        } else {
            Ok(3.0) // Deprecated projects likely had significant issues
        }
    }

    fn evaluate_resource_efficiency(&self, project: &MigrationProject) -> ResearchResult<f64> {
        if let Some(tech_details) = &project.technical_details {
            if let Some(team_size) = tech_details.team_size {
                if let Some(timeline) = tech_details.migration_timeline_months {
                    // Smaller teams with reasonable timelines suggest good efficiency
                    let efficiency_score = match (team_size, timeline) {
                        (1..=5, 1..=12) => 9.0,   // Small team, fast migration
                        (1..=5, 13..=24) => 7.0,  // Small team, longer migration
                        (6..=15, 1..=12) => 7.0,  // Medium team, fast migration
                        (6..=15, 13..=24) => 6.0, // Medium team, longer migration
                        _ => 5.0,                  // Large team or very long migration
                    };
                    Ok(efficiency_score)
                } else {
                    Ok(6.0) // No timeline data
                }
            } else {
                Ok(5.0) // No team size data
            }
        } else {
            Ok(5.0) // No technical details
        }
    }

    fn evaluate_risk_management(&self, project: &MigrationProject) -> ResearchResult<f64> {
        // This would require analysis of project documentation, issues, etc.
        // For now, provide estimates based on project success indicators
        match project.status {
            super::ProjectStatus::Active => {
                if project.stars > 1000 {
                    Ok(8.0) // Popular active projects likely have good risk management
                } else {
                    Ok(6.0)
                }
            },
            super::ProjectStatus::Archived => Ok(5.0), // Archived projects may have had risk issues
            super::ProjectStatus::Deprecated => Ok(3.0), // Deprecated projects likely had poor risk management
        }
    }

    fn evaluate_user_satisfaction(&self, project: &MigrationProject) -> ResearchResult<f64> {
        // This would typically require user surveys, reviews, etc.
        // For now, use GitHub stars as a proxy
        let satisfaction_score = match project.stars {
            0..=100 => 4.0,
            101..=500 => 5.0,
            501..=1000 => 6.0,
            1001..=5000 => 7.0,
            5001..=10000 => 8.0,
            _ => 9.0,
        };
        Ok(satisfaction_score)
    }

    fn evaluate_stability(&self, project: &MigrationProject) -> ResearchResult<f64> {
        // This would require analysis of bug reports, crash rates, etc.
        // For now, estimate based on project maturity and activity
        match project.status {
            super::ProjectStatus::Active => {
                if project.stars > 5000 {
                    Ok(8.0) // Popular active projects likely stable
                } else if project.stars > 1000 {
                    Ok(7.0)
                } else {
                    Ok(6.0)
                }
            },
            super::ProjectStatus::Archived => Ok(6.0), // May be stable but not maintained
            super::ProjectStatus::Deprecated => Ok(4.0), // Likely has stability issues
        }
    }

    fn evaluate_maintenance_burden(&self, project: &MigrationProject) -> ResearchResult<f64> {
        if let Some(tech_details) = &project.technical_details {
            match tech_details.target_technology {
                TargetTechnology::Rust => Ok(7.0), // Rust has good maintainability
                TargetTechnology::Cpp => Ok(5.0),  // C++ can be harder to maintain
                TargetTechnology::Go => Ok(8.0),   // Go designed for maintainability
                TargetTechnology::Wasm => Ok(6.0), // WASM maintenance varies
                TargetTechnology::Native => Ok(6.0), // Depends on implementation
                TargetTechnology::Tauri => Ok(7.0), // Tauri aims for good maintainability
                TargetTechnology::Flutter => Ok(7.0), // Flutter has good tooling
            }
        } else {
            Ok(5.0)
        }
    }

    fn evaluate_feature_parity(&self, project: &MigrationProject) -> ResearchResult<f64> {
        match project.category {
            super::ProjectCategory::VsCodeFork => Ok(9.0), // Forks typically maintain high parity
            super::ProjectCategory::IdeMigration => Ok(7.0), // Migrations aim for parity
            super::ProjectCategory::RustIde => Ok(6.0), // New IDEs may have different feature sets
            super::ProjectCategory::AiIde => Ok(8.0), // AI IDEs often extend rather than replace
            super::ProjectCategory::WasmApp => Ok(6.0), // WASM apps may have limitations
        }
    }

    fn assess_migration_complexity(&self, project: &MigrationProject) -> MigrationComplexity {
        let mut complexity_score = 0;

        // Factor in source technology complexity
        if let Some(tech_details) = &project.technical_details {
            complexity_score += match tech_details.source_technology {
                SourceTechnology::Electron => 2, // Electron is moderately complex to migrate from
                SourceTechnology::Native => 3,   // Native migrations can be complex
                SourceTechnology::Web => 1,      // Web is typically simpler
                SourceTechnology::Java => 3,     // Java migrations can be complex
                SourceTechnology::DotNet => 3,   // .NET migrations can be complex
            };

            // Factor in target technology complexity
            complexity_score += match tech_details.target_technology {
                TargetTechnology::Rust => 3,     // Rust has learning curve
                TargetTechnology::Cpp => 4,      // C++ is complex
                TargetTechnology::Go => 2,       // Go is simpler
                TargetTechnology::Wasm => 3,     // WASM has complexity
                TargetTechnology::Native => 4,   // Native can be very complex
                TargetTechnology::Tauri => 2,    // Tauri simplifies some aspects
                TargetTechnology::Flutter => 2,  // Flutter is relatively straightforward
            };

            // Factor in migration approach
            complexity_score += match tech_details.migration_approach {
                MigrationApproach::Gradual => 2,      // Gradual is moderately complex
                MigrationApproach::CompleteRewrite => 4, // Complete rewrite is very complex
                MigrationApproach::Hybrid => 3,       // Hybrid is complex
                MigrationApproach::Incremental => 1,  // Incremental is simpler
            };
        } else {
            complexity_score = 5; // Default medium complexity
        }

        match complexity_score {
            0..=3 => MigrationComplexity::Low,
            4..=6 => MigrationComplexity::Medium,
            7..=9 => MigrationComplexity::High,
            _ => MigrationComplexity::VeryHigh,
        }
    }

    async fn identify_success_indicators(&self, project: &MigrationProject) -> ResearchResult<Vec<SuccessIndicator>> {
        let mut indicators = Vec::new();

        // Performance success indicators
        if let Some(tech_details) = &project.technical_details {
            for (metric_name, metric) in &tech_details.performance_metrics {
                if let Some(improvement) = metric.improvement_percentage {
                    if improvement > 20.0 {
                        indicators.push(SuccessIndicator {
                            category: "Performance".to_string(),
                            description: format!("{} improved by {:.1}%", metric_name, improvement),
                            evidence: vec![
                                metric.source_url.clone().unwrap_or_default(),
                                format!("Before: {:?}, After: {:?}", metric.before_value, metric.after_value),
                            ],
                            confidence_level: if metric.verified { 0.9 } else { 0.6 },
                        });
                    }
                }
            }
        }

        // Community adoption indicators
        if project.stars > 1000 {
            indicators.push(SuccessIndicator {
                category: "Community Adoption".to_string(),
                description: format!("High community adoption with {} stars", project.stars),
                evidence: vec![
                    project.github_url.clone().unwrap_or_default(),
                    format!("{} contributors", project.contributors),
                ],
                confidence_level: 0.8,
            });
        }

        // Project sustainability indicators
        if project.status == super::ProjectStatus::Active {
            indicators.push(SuccessIndicator {
                category: "Sustainability".to_string(),
                description: "Project remains actively maintained".to_string(),
                evidence: vec![
                    format!("Status: {:?}", project.status),
                    project.last_updated.map(|d| format!("Last updated: {}", d)).unwrap_or_default(),
                ],
                confidence_level: 0.7,
            });
        }

        Ok(indicators)
    }

    async fn identify_risk_factors(&self, project: &MigrationProject) -> ResearchResult<Vec<RiskFactor>> {
        let mut risk_factors = Vec::new();

        // Technology maturity risks
        if let Some(tech_details) = &project.technical_details {
            match tech_details.target_technology {
                TargetTechnology::Wasm => {
                    risk_factors.push(RiskFactor {
                        category: "Technology Maturity".to_string(),
                        description: "WASM ecosystem still evolving".to_string(),
                        severity: RiskSeverity::Medium,
                        mitigation_strategies: vec![
                            "Use stable WASM features only".to_string(),
                            "Have fallback plans for WASM limitations".to_string(),
                        ],
                        observed_outcomes: vec![
                            "Some projects faced tooling limitations".to_string(),
                        ],
                    });
                },
                TargetTechnology::Rust => {
                    risk_factors.push(RiskFactor {
                        category: "Learning Curve".to_string(),
                        description: "Rust has steep learning curve for teams".to_string(),
                        severity: RiskSeverity::Medium,
                        mitigation_strategies: vec![
                            "Invest in team training".to_string(),
                            "Start with experienced Rust developers".to_string(),
                        ],
                        observed_outcomes: vec![
                            "Initial development slower but improves over time".to_string(),
                        ],
                    });
                },
                _ => {},
            }

            // Migration approach risks
            match tech_details.migration_approach {
                MigrationApproach::CompleteRewrite => {
                    risk_factors.push(RiskFactor {
                        category: "Migration Strategy".to_string(),
                        description: "Complete rewrite carries high risk of scope creep and delays".to_string(),
                        severity: RiskSeverity::High,
                        mitigation_strategies: vec![
                            "Strict scope control".to_string(),
                            "Frequent milestone reviews".to_string(),
                            "Maintain feature parity focus".to_string(),
                        ],
                        observed_outcomes: vec![
                            "Many complete rewrites exceed timeline estimates".to_string(),
                        ],
                    });
                },
                _ => {},
            }
        }

        // Project sustainability risks
        if project.contributors < 5 {
            risk_factors.push(RiskFactor {
                category: "Project Sustainability".to_string(),
                description: "Low contributor count may indicate sustainability risk".to_string(),
                severity: RiskSeverity::Medium,
                mitigation_strategies: vec![
                    "Build larger contributor community".to_string(),
                    "Ensure good documentation and onboarding".to_string(),
                ],
                observed_outcomes: vec![
                    "Projects with few contributors may become abandoned".to_string(),
                ],
            });
        }

        Ok(risk_factors)
    }

    async fn extract_lessons_learned(&self, project: &MigrationProject) -> ResearchResult<Vec<LessonLearned>> {
        let mut lessons = Vec::new();

        // Technology-specific lessons
        if let Some(tech_details) = &project.technical_details {
            match tech_details.target_technology {
                TargetTechnology::Rust => {
                    lessons.push(LessonLearned {
                        category: "Technology Choice".to_string(),
                        lesson: "Rust provides excellent performance but requires significant upfront investment in learning".to_string(),
                        context: "Multiple Rust IDE projects show this pattern".to_string(),
                        applicability: 0.9,
                        source_projects: vec![project.name.clone()],
                    });
                },
                TargetTechnology::Tauri => {
                    lessons.push(LessonLearned {
                        category: "Migration Strategy".to_string(),
                        lesson: "Tauri enables gradual migration from Electron with minimal disruption".to_string(),
                        context: "Tauri projects show smooth migration paths".to_string(),
                        applicability: 0.8,
                        source_projects: vec![project.name.clone()],
                    });
                },
                _ => {},
            }

            // Migration approach lessons
            match tech_details.migration_approach {
                MigrationApproach::Gradual => {
                    lessons.push(LessonLearned {
                        category: "Migration Approach".to_string(),
                        lesson: "Gradual migration reduces risk but requires careful interface design".to_string(),
                        context: "Gradual migrations show better success rates".to_string(),
                        applicability: 0.8,
                        source_projects: vec![project.name.clone()],
                    });
                },
                _ => {},
            }
        }

        // Performance lessons
        if let Some(tech_details) = &project.technical_details {
            let has_significant_improvements = tech_details.performance_metrics.values()
                .any(|metric| metric.improvement_percentage.unwrap_or(0.0) > 50.0);
            
            if has_significant_improvements {
                lessons.push(LessonLearned {
                    category: "Performance Optimization".to_string(),
                    lesson: "Native implementations can provide dramatic performance improvements over Electron".to_string(),
                    context: "Projects show 50%+ improvements in key metrics".to_string(),
                    applicability: 0.9,
                    source_projects: vec![project.name.clone()],
                });
            }
        }

        Ok(lessons)
    }

    async fn assess_applicability_to_kiro(&self, project: &MigrationProject) -> ResearchResult<ApplicabilityAssessment> {
        let technical_similarity = self.calculate_technical_similarity(project);
        let architectural_alignment = self.calculate_architectural_alignment(project);
        let resource_requirements_match = self.calculate_resource_requirements_match(project);
        let risk_profile_compatibility = self.calculate_risk_profile_compatibility(project);

        let overall_score = (technical_similarity + architectural_alignment + 
                           resource_requirements_match + risk_profile_compatibility) / 4.0;

        let recommendations = self.generate_applicability_recommendations(project, overall_score);

        Ok(ApplicabilityAssessment {
            overall_score,
            technical_similarity,
            architectural_alignment,
            resource_requirements_match,
            risk_profile_compatibility,
            recommendations,
        })
    }

    fn calculate_technical_similarity(&self, project: &MigrationProject) -> f64 {
        let mut similarity_score = 0.0;

        // Kiro is an Electron-based VS Code fork, so projects with similar characteristics score higher
        match project.category {
            super::ProjectCategory::VsCodeFork => similarity_score += 0.4,
            super::ProjectCategory::IdeMigration => similarity_score += 0.3,
            super::ProjectCategory::AiIde => similarity_score += 0.2,
            _ => similarity_score += 0.1,
        }

        if let Some(tech_details) = &project.technical_details {
            // Source technology similarity
            match tech_details.source_technology {
                SourceTechnology::Electron => similarity_score += 0.3, // Perfect match
                SourceTechnology::Web => similarity_score += 0.2,      // Similar web tech
                _ => similarity_score += 0.1,
            }

            // Target technology relevance
            match tech_details.target_technology {
                TargetTechnology::Rust => similarity_score += 0.2,  // Our target
                TargetTechnology::Wasm => similarity_score += 0.2,  // Our target
                TargetTechnology::Tauri => similarity_score += 0.15, // Rust-based Electron alternative
                _ => similarity_score += 0.05,
            }
        }

        similarity_score.min(1.0)
    }

    fn calculate_architectural_alignment(&self, project: &MigrationProject) -> f64 {
        let mut alignment_score = 0.5; // Base score

        // VS Code forks have high architectural alignment
        if matches!(project.category, super::ProjectCategory::VsCodeFork) {
            alignment_score += 0.3;
        }

        // AI IDEs have some alignment due to AI integration requirements
        if matches!(project.category, super::ProjectCategory::AiIde) {
            alignment_score += 0.2;
        }

        alignment_score.min(1.0)
    }

    fn calculate_resource_requirements_match(&self, project: &MigrationProject) -> f64 {
        if let Some(tech_details) = &project.technical_details {
            if let Some(team_size) = tech_details.team_size {
                // Kiro likely has a small to medium team, so similar-sized projects are more applicable
                match team_size {
                    1..=5 => 0.9,   // Small team - highly applicable
                    6..=15 => 0.8,  // Medium team - very applicable
                    16..=30 => 0.6, // Large team - moderately applicable
                    _ => 0.4,       // Very large team - less applicable
                }
            } else {
                0.6 // No data, assume medium applicability
            }
        } else {
            0.5 // No technical details available
        }
    }

    fn calculate_risk_profile_compatibility(&self, project: &MigrationProject) -> f64 {
        let mut compatibility_score = 0.5; // Base score

        // Active projects suggest lower risk
        if matches!(project.status, super::ProjectStatus::Active) {
            compatibility_score += 0.2;
        }

        // Popular projects suggest proven approaches
        if project.stars > 1000 {
            compatibility_score += 0.2;
        }

        // Technology-specific risk considerations
        if let Some(tech_details) = &project.technical_details {
            match tech_details.target_technology {
                TargetTechnology::Rust => compatibility_score += 0.1, // Good match for our goals
                TargetTechnology::Wasm => compatibility_score += 0.1, // Good match for our goals
                TargetTechnology::Tauri => compatibility_score += 0.1, // Lower risk migration path
                _ => {},
            }
        }

        compatibility_score.min(1.0)
    }

    fn generate_applicability_recommendations(&self, project: &MigrationProject, overall_score: f64) -> Vec<String> {
        let mut recommendations = Vec::new();

        if overall_score > 0.8 {
            recommendations.push("Highly applicable - study this project's approach in detail".to_string());
        } else if overall_score > 0.6 {
            recommendations.push("Moderately applicable - extract relevant patterns and lessons".to_string());
        } else if overall_score > 0.4 {
            recommendations.push("Limited applicability - focus on specific technical insights".to_string());
        } else {
            recommendations.push("Low applicability - use only for general reference".to_string());
        }

        // Technology-specific recommendations
        if let Some(tech_details) = &project.technical_details {
            match tech_details.target_technology {
                TargetTechnology::Rust => {
                    recommendations.push("Study Rust-specific implementation patterns and tooling choices".to_string());
                },
                TargetTechnology::Wasm => {
                    recommendations.push("Analyze WASM integration strategies and performance characteristics".to_string());
                },
                TargetTechnology::Tauri => {
                    recommendations.push("Consider Tauri as potential migration path for gradual transition".to_string());
                },
                _ => {},
            }
        }

        // Category-specific recommendations
        match project.category {
            super::ProjectCategory::VsCodeFork => {
                recommendations.push("Study extension compatibility and marketplace strategies".to_string());
            },
            super::ProjectCategory::AiIde => {
                recommendations.push("Analyze AI integration patterns and user experience approaches".to_string());
            },
            _ => {},
        }

        recommendations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migration_analyzer_initialization() {
        let analyzer = MigrationAnalyzer::new();
        assert!(!analyzer.analysis_frameworks.is_empty());
        assert_eq!(analyzer.analysis_frameworks.len(), 3);
    }

    #[test]
    fn test_migration_complexity_assessment() {
        let analyzer = MigrationAnalyzer::new();
        
        let project = MigrationProject {
            id: None,
            name: "Test Project".to_string(),
            url: "https://example.com".to_string(),
            github_url: None,
            category: super::ProjectCategory::IdeMigration,
            status: super::ProjectStatus::Active,
            last_updated: None,
            stars: 1000,
            contributors: 10,
            technical_details: Some(TechnicalDetails {
                source_technology: SourceTechnology::Electron,
                target_technology: TargetTechnology::Rust,
                migration_approach: MigrationApproach::Gradual,
                architecture_patterns: vec![],
                performance_metrics: HashMap::new(),
                migration_timeline_months: Some(12),
                team_size: Some(5),
            }),
            analysis: None,
        };

        let complexity = analyzer.assess_migration_complexity(&project);
        assert!(matches!(complexity, MigrationComplexity::Medium | MigrationComplexity::High));
    }

    #[test]
    fn test_technical_similarity_calculation() {
        let analyzer = MigrationAnalyzer::new();
        
        let vscode_fork = MigrationProject {
            id: None,
            name: "VS Code Fork".to_string(),
            url: "https://example.com".to_string(),
            github_url: None,
            category: super::ProjectCategory::VsCodeFork,
            status: super::ProjectStatus::Active,
            last_updated: None,
            stars: 1000,
            contributors: 10,
            technical_details: Some(TechnicalDetails {
                source_technology: SourceTechnology::Electron,
                target_technology: TargetTechnology::Rust,
                migration_approach: MigrationApproach::Gradual,
                architecture_patterns: vec![],
                performance_metrics: HashMap::new(),
                migration_timeline_months: None,
                team_size: None,
            }),
            analysis: None,
        };

        let similarity = analyzer.calculate_technical_similarity(&vscode_fork);
        assert!(similarity > 0.8); // Should be high similarity
    }
}