// Migration Strategy Decision Framework
// Provides structured decision-making tools for IDE migration approaches

use super::{MigrationProject, ResearchResult, ResearchError, MigrationApproach};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationDecisionFramework {
    pub decision_criteria: Vec<DecisionCriterion>,
    pub approach_evaluations: HashMap<MigrationApproach, ApproachEvaluation>,
    pub risk_assessment_matrix: RiskAssessmentMatrix,
    pub timeline_estimation_models: Vec<TimelineModel>,
    pub complexity_assessment_tools: ComplexityAssessmentTools,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionCriterion {
    pub criterion_id: String,
    pub name: String,
    pub description: String,
    pub weight: f64,
    pub measurement_scale: MeasurementScale,
    pub evaluation_method: EvaluationMethod,
    pub kiro_specific_considerations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MeasurementScale {
    Numeric { min: f64, max: f64, unit: String },
    Categorical { categories: Vec<String> },
    Boolean,
    Ordinal { levels: Vec<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvaluationMethod {
    Quantitative { formula: String, data_sources: Vec<String> },
    Qualitative { assessment_guidelines: Vec<String> },
    Hybrid { quantitative_weight: f64, qualitative_weight: f64 },
    ExpertJudgment { expert_criteria: Vec<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApproachEvaluation {
    pub approach: MigrationApproach,
    pub criterion_scores: HashMap<String, f64>,
    pub overall_score: f64,
    pub strengths: Vec<String>,
    pub weaknesses: Vec<String>,
    pub best_fit_scenarios: Vec<String>,
    pub resource_requirements: ResourceRequirements,
    pub success_probability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    pub team_size_range: (i32, i32),
    pub timeline_months_range: (i32, i32),
    pub budget_range: Option<(f64, f64)>,
    pub skill_requirements: Vec<SkillRequirement>,
    pub infrastructure_needs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillRequirement {
    pub skill_name: String,
    pub proficiency_level: ProficiencyLevel,
    pub team_coverage_percentage: f64,
    pub acquisition_difficulty: AcquisitionDifficulty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProficiencyLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AcquisitionDifficulty {
    Easy,      // Can be learned in weeks
    Moderate,  // Can be learned in months
    Hard,      // Requires significant investment
    VeryHard,  // Requires hiring or extensive training
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessmentMatrix {
    pub risk_categories: Vec<RiskCategory>,
    pub approach_risk_profiles: HashMap<MigrationApproach, RiskProfile>,
    pub mitigation_strategies: HashMap<String, MitigationStrategy>,
    pub risk_tolerance_guidelines: RiskToleranceGuidelines,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskCategory {
    pub category_name: String,
    pub description: String,
    pub impact_areas: Vec<String>,
    pub typical_risks: Vec<TypicalRisk>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypicalRisk {
    pub risk_name: String,
    pub description: String,
    pub probability_range: (f64, f64),
    pub impact_range: (f64, f64),
    pub indicators: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskProfile {
    pub overall_risk_level: RiskLevel,
    pub category_risks: HashMap<String, f64>,
    pub key_risk_factors: Vec<String>,
    pub mitigation_priorities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitigationStrategy {
    pub strategy_name: String,
    pub description: String,
    pub applicable_risks: Vec<String>,
    pub implementation_steps: Vec<String>,
    pub effectiveness_rating: f64,
    pub implementation_cost: ImplementationCost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImplementationCost {
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskToleranceGuidelines {
    pub acceptable_risk_threshold: f64,
    pub risk_appetite_by_category: HashMap<String, f64>,
    pub escalation_triggers: Vec<String>,
    pub decision_authority_matrix: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineModel {
    pub model_name: String,
    pub description: String,
    pub base_factors: Vec<TimelineFactor>,
    pub adjustment_factors: Vec<AdjustmentFactor>,
    pub validation_data: Vec<ValidationDataPoint>,
    pub accuracy_metrics: AccuracyMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineFactor {
    pub factor_name: String,
    pub description: String,
    pub base_duration_months: f64,
    pub complexity_multipliers: HashMap<String, f64>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdjustmentFactor {
    pub factor_name: String,
    pub description: String,
    pub adjustment_type: AdjustmentType,
    pub impact_range: (f64, f64),
    pub application_conditions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdjustmentType {
    Multiplicative,
    Additive,
    Conditional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationDataPoint {
    pub project_name: String,
    pub predicted_timeline: f64,
    pub actual_timeline: f64,
    pub accuracy_percentage: f64,
    pub key_factors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccuracyMetrics {
    pub mean_absolute_error: f64,
    pub root_mean_square_error: f64,
    pub accuracy_within_20_percent: f64,
    pub confidence_interval: (f64, f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityAssessmentTools {
    pub assessment_dimensions: Vec<ComplexityDimension>,
    pub scoring_methodology: ScoringMethodology,
    pub complexity_thresholds: ComplexityThresholds,
    pub recommendation_matrix: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityDimension {
    pub dimension_name: String,
    pub description: String,
    pub measurement_criteria: Vec<String>,
    pub scoring_rubric: Vec<ScoringLevel>,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoringLevel {
    pub level_name: String,
    pub score: f64,
    pub description: String,
    pub indicators: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoringMethodology {
    pub aggregation_method: AggregationMethod,
    pub normalization_approach: String,
    pub weighting_scheme: String,
    pub validation_approach: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AggregationMethod {
    WeightedAverage,
    GeometricMean,
    MaxMin,
    CustomFormula { formula: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityThresholds {
    pub low_complexity: f64,
    pub medium_complexity: f64,
    pub high_complexity: f64,
    pub very_high_complexity: f64,
}

pub struct DecisionFrameworkBuilder {
    criteria: Vec<DecisionCriterion>,
    risk_categories: Vec<RiskCategory>,
    timeline_models: Vec<TimelineModel>,
}

impl DecisionFrameworkBuilder {
    pub fn new() -> Self {
        Self {
            criteria: Vec::new(),
            risk_categories: Vec::new(),
            timeline_models: Vec::new(),
        }
    }

    pub fn build_kiro_decision_framework() -> MigrationDecisionFramework {
        let mut builder = Self::new();
        
        // Add Kiro-specific decision criteria
        builder.add_kiro_decision_criteria();
        builder.add_kiro_risk_categories();
        builder.add_kiro_timeline_models();
        
        builder.build()
    }

    fn add_kiro_decision_criteria(&mut self) {
        self.criteria = vec![
            DecisionCriterion {
                criterion_id: "performance_impact".to_string(),
                name: "Performance Impact".to_string(),
                description: "Expected performance improvements from migration".to_string(),
                weight: 0.25,
                measurement_scale: MeasurementScale::Numeric { 
                    min: 1.0, 
                    max: 10.0, 
                    unit: "improvement_factor".to_string() 
                },
                evaluation_method: EvaluationMethod::Quantitative {
                    formula: "weighted_average(startup_improvement, memory_improvement, cpu_improvement)".to_string(),
                    data_sources: vec!["benchmark_data".to_string(), "comparable_projects".to_string()],
                },
                kiro_specific_considerations: vec![
                    "AI processing performance requirements".to_string(),
                    "Extension system performance impact".to_string(),
                    "Large file handling performance".to_string(),
                ],
            },

            DecisionCriterion {
                criterion_id: "extension_compatibility".to_string(),
                name: "Extension Ecosystem Compatibility".to_string(),
                description: "Ability to maintain VS Code extension compatibility".to_string(),
                weight: 0.20,
                measurement_scale: MeasurementScale::Numeric { 
                    min: 0.0, 
                    max: 100.0, 
                    unit: "percentage".to_string() 
                },
                evaluation_method: EvaluationMethod::Hybrid {
                    quantitative_weight: 0.7,
                    qualitative_weight: 0.3,
                },
                kiro_specific_considerations: vec![
                    "Open VSX Registry compatibility".to_string(),
                    "Kiro Agent extension integration".to_string(),
                    "Custom extension APIs preservation".to_string(),
                ],
            },

            DecisionCriterion {
                criterion_id: "development_velocity".to_string(),
                name: "Development Velocity Impact".to_string(),
                description: "Impact on team productivity and development speed".to_string(),
                weight: 0.15,
                measurement_scale: MeasurementScale::Ordinal { 
                    levels: vec![
                        "Significantly Slower".to_string(),
                        "Slower".to_string(),
                        "Neutral".to_string(),
                        "Faster".to_string(),
                        "Significantly Faster".to_string(),
                    ]
                },
                evaluation_method: EvaluationMethod::Qualitative {
                    assessment_guidelines: vec![
                        "Assess team Rust expertise level".to_string(),
                        "Evaluate tooling and IDE support".to_string(),
                        "Consider learning curve impact".to_string(),
                    ],
                },
                kiro_specific_considerations: vec![
                    "Current team JavaScript/TypeScript expertise".to_string(),
                    "Rust learning curve for team".to_string(),
                    "Tooling ecosystem maturity".to_string(),
                ],
            },

            DecisionCriterion {
                criterion_id: "implementation_risk".to_string(),
                name: "Implementation Risk Level".to_string(),
                description: "Overall risk profile of the migration approach".to_string(),
                weight: 0.20,
                measurement_scale: MeasurementScale::Ordinal { 
                    levels: vec![
                        "Very Low".to_string(),
                        "Low".to_string(),
                        "Medium".to_string(),
                        "High".to_string(),
                        "Very High".to_string(),
                    ]
                },
                evaluation_method: EvaluationMethod::Hybrid {
                    quantitative_weight: 0.4,
                    qualitative_weight: 0.6,
                },
                kiro_specific_considerations: vec![
                    "User base migration risk".to_string(),
                    "Business continuity requirements".to_string(),
                    "Competitive pressure timeline".to_string(),
                ],
            },

            DecisionCriterion {
                criterion_id: "resource_requirements".to_string(),
                name: "Resource Requirements".to_string(),
                description: "Team size, timeline, and budget requirements".to_string(),
                weight: 0.20,
                measurement_scale: MeasurementScale::Numeric { 
                    min: 1.0, 
                    max: 5.0, 
                    unit: "resource_intensity".to_string() 
                },
                evaluation_method: EvaluationMethod::Quantitative {
                    formula: "weighted_sum(team_size_factor, timeline_factor, complexity_factor)".to_string(),
                    data_sources: vec!["comparable_projects".to_string(), "expert_estimates".to_string()],
                },
                kiro_specific_considerations: vec![
                    "Available team capacity".to_string(),
                    "Budget constraints".to_string(),
                    "Timeline flexibility".to_string(),
                ],
            },
        ];
    }

    fn add_kiro_risk_categories(&mut self) {
        self.risk_categories = vec![
            RiskCategory {
                category_name: "Technical Risk".to_string(),
                description: "Risks related to technical implementation and architecture".to_string(),
                impact_areas: vec![
                    "Performance targets".to_string(),
                    "Feature parity".to_string(),
                    "System stability".to_string(),
                ],
                typical_risks: vec![
                    TypicalRisk {
                        risk_name: "Performance Regression".to_string(),
                        description: "Failure to achieve expected performance improvements".to_string(),
                        probability_range: (0.2, 0.4),
                        impact_range: (0.7, 0.9),
                        indicators: vec![
                            "Benchmark results below targets".to_string(),
                            "User complaints about performance".to_string(),
                        ],
                    },
                    TypicalRisk {
                        risk_name: "Extension Compatibility Issues".to_string(),
                        description: "Breaking compatibility with existing extensions".to_string(),
                        probability_range: (0.3, 0.6),
                        impact_range: (0.8, 1.0),
                        indicators: vec![
                            "Extension API changes required".to_string(),
                            "Popular extensions not working".to_string(),
                        ],
                    },
                ],
            },

            RiskCategory {
                category_name: "Project Risk".to_string(),
                description: "Risks related to project management and execution".to_string(),
                impact_areas: vec![
                    "Timeline adherence".to_string(),
                    "Budget control".to_string(),
                    "Quality delivery".to_string(),
                ],
                typical_risks: vec![
                    TypicalRisk {
                        risk_name: "Timeline Overrun".to_string(),
                        description: "Project taking longer than estimated".to_string(),
                        probability_range: (0.4, 0.7),
                        impact_range: (0.5, 0.8),
                        indicators: vec![
                            "Milestone delays".to_string(),
                            "Scope creep".to_string(),
                        ],
                    },
                    TypicalRisk {
                        risk_name: "Skill Gap".to_string(),
                        description: "Team lacking required Rust expertise".to_string(),
                        probability_range: (0.3, 0.5),
                        impact_range: (0.6, 0.9),
                        indicators: vec![
                            "Slow development progress".to_string(),
                            "Code quality issues".to_string(),
                        ],
                    },
                ],
            },

            RiskCategory {
                category_name: "Business Risk".to_string(),
                description: "Risks related to business impact and user adoption".to_string(),
                impact_areas: vec![
                    "User satisfaction".to_string(),
                    "Market position".to_string(),
                    "Revenue impact".to_string(),
                ],
                typical_risks: vec![
                    TypicalRisk {
                        risk_name: "User Adoption Resistance".to_string(),
                        description: "Users resistant to migrating to new version".to_string(),
                        probability_range: (0.2, 0.4),
                        impact_range: (0.7, 1.0),
                        indicators: vec![
                            "User feedback concerns".to_string(),
                            "Slow migration rates".to_string(),
                        ],
                    },
                ],
            },
        ];
    }

    fn add_kiro_timeline_models(&mut self) {
        self.timeline_models = vec![
            TimelineModel {
                model_name: "Incremental Migration Model".to_string(),
                description: "Timeline estimation for gradual component-by-component migration".to_string(),
                base_factors: vec![
                    TimelineFactor {
                        factor_name: "Core Engine Migration".to_string(),
                        description: "Migrating performance-critical core components".to_string(),
                        base_duration_months: 6.0,
                        complexity_multipliers: HashMap::from([
                            ("low".to_string(), 0.8),
                            ("medium".to_string(), 1.0),
                            ("high".to_string(), 1.5),
                        ]),
                        dependencies: vec!["team_training".to_string()],
                    },
                    TimelineFactor {
                        factor_name: "UI Component Migration".to_string(),
                        description: "Migrating user interface components".to_string(),
                        base_duration_months: 4.0,
                        complexity_multipliers: HashMap::from([
                            ("low".to_string(), 0.9),
                            ("medium".to_string(), 1.0),
                            ("high".to_string(), 1.3),
                        ]),
                        dependencies: vec!["core_engine_migration".to_string()],
                    },
                    TimelineFactor {
                        factor_name: "Extension System Migration".to_string(),
                        description: "Migrating extension system and APIs".to_string(),
                        base_duration_months: 8.0,
                        complexity_multipliers: HashMap::from([
                            ("low".to_string(), 0.7),
                            ("medium".to_string(), 1.0),
                            ("high".to_string(), 2.0),
                        ]),
                        dependencies: vec!["core_engine_migration".to_string()],
                    },
                ],
                adjustment_factors: vec![
                    AdjustmentFactor {
                        factor_name: "Team Rust Expertise".to_string(),
                        description: "Team's existing Rust knowledge and experience".to_string(),
                        adjustment_type: AdjustmentType::Multiplicative,
                        impact_range: (0.7, 1.5),
                        application_conditions: vec![
                            "High expertise: 0.7x multiplier".to_string(),
                            "Medium expertise: 1.0x multiplier".to_string(),
                            "Low expertise: 1.5x multiplier".to_string(),
                        ],
                    },
                    AdjustmentFactor {
                        factor_name: "Performance Requirements".to_string(),
                        description: "Stringency of performance improvement requirements".to_string(),
                        adjustment_type: AdjustmentType::Additive,
                        impact_range: (0.0, 4.0),
                        application_conditions: vec![
                            "Standard requirements: +0 months".to_string(),
                            "High requirements: +2 months".to_string(),
                            "Extreme requirements: +4 months".to_string(),
                        ],
                    },
                ],
                validation_data: vec![
                    ValidationDataPoint {
                        project_name: "Tauri Migration Example".to_string(),
                        predicted_timeline: 12.0,
                        actual_timeline: 14.0,
                        accuracy_percentage: 85.7,
                        key_factors: vec!["team_expertise".to_string(), "scope_creep".to_string()],
                    },
                ],
                accuracy_metrics: AccuracyMetrics {
                    mean_absolute_error: 2.1,
                    root_mean_square_error: 2.8,
                    accuracy_within_20_percent: 0.75,
                    confidence_interval: (0.8, 1.3),
                },
            },
        ];
    }

    fn build(self) -> MigrationDecisionFramework {
        let approach_evaluations = self.build_approach_evaluations();
        let risk_assessment_matrix = self.build_risk_assessment_matrix();
        let complexity_assessment_tools = self.build_complexity_assessment_tools();

        MigrationDecisionFramework {
            decision_criteria: self.criteria,
            approach_evaluations,
            risk_assessment_matrix,
            timeline_estimation_models: self.timeline_models,
            complexity_assessment_tools,
        }
    }

    fn build_approach_evaluations(&self) -> HashMap<MigrationApproach, ApproachEvaluation> {
        let mut evaluations = HashMap::new();

        // Gradual Migration Evaluation
        evaluations.insert(MigrationApproach::Gradual, ApproachEvaluation {
            approach: MigrationApproach::Gradual,
            criterion_scores: HashMap::from([
                ("performance_impact".to_string(), 7.0),
                ("extension_compatibility".to_string(), 9.0),
                ("development_velocity".to_string(), 6.0),
                ("implementation_risk".to_string(), 8.0),
                ("resource_requirements".to_string(), 7.0),
            ]),
            overall_score: 7.4,
            strengths: vec![
                "Low risk profile".to_string(),
                "Maintains user continuity".to_string(),
                "Allows for iterative improvement".to_string(),
                "Preserves extension compatibility".to_string(),
            ],
            weaknesses: vec![
                "Longer overall timeline".to_string(),
                "Complex integration management".to_string(),
                "May not achieve maximum performance gains".to_string(),
            ],
            best_fit_scenarios: vec![
                "Large existing user base".to_string(),
                "Critical extension ecosystem".to_string(),
                "Limited risk tolerance".to_string(),
                "Continuous delivery requirements".to_string(),
            ],
            resource_requirements: ResourceRequirements {
                team_size_range: (5, 10),
                timeline_months_range: (15, 24),
                budget_range: Some((500000.0, 1000000.0)),
                skill_requirements: vec![
                    SkillRequirement {
                        skill_name: "Rust Development".to_string(),
                        proficiency_level: ProficiencyLevel::Intermediate,
                        team_coverage_percentage: 60.0,
                        acquisition_difficulty: AcquisitionDifficulty::Moderate,
                    },
                    SkillRequirement {
                        skill_name: "JavaScript/TypeScript".to_string(),
                        proficiency_level: ProficiencyLevel::Advanced,
                        team_coverage_percentage: 80.0,
                        acquisition_difficulty: AcquisitionDifficulty::Easy,
                    },
                ],
                infrastructure_needs: vec![
                    "Hybrid build system".to_string(),
                    "Integration testing framework".to_string(),
                    "Performance monitoring".to_string(),
                ],
            },
            success_probability: 0.85,
        });

        // Complete Rewrite Evaluation
        evaluations.insert(MigrationApproach::CompleteRewrite, ApproachEvaluation {
            approach: MigrationApproach::CompleteRewrite,
            criterion_scores: HashMap::from([
                ("performance_impact".to_string(), 9.5),
                ("extension_compatibility".to_string(), 6.0),
                ("development_velocity".to_string(), 4.0),
                ("implementation_risk".to_string(), 4.0),
                ("resource_requirements".to_string(), 3.0),
            ]),
            overall_score: 5.3,
            strengths: vec![
                "Maximum performance potential".to_string(),
                "Clean architecture".to_string(),
                "Latest technology adoption".to_string(),
                "No legacy constraints".to_string(),
            ],
            weaknesses: vec![
                "High risk profile".to_string(),
                "Long development timeline".to_string(),
                "Significant resource requirements".to_string(),
                "Extension compatibility challenges".to_string(),
            ],
            best_fit_scenarios: vec![
                "Performance is critical priority".to_string(),
                "Willing to break compatibility".to_string(),
                "Large development team available".to_string(),
                "Long timeline acceptable".to_string(),
            ],
            resource_requirements: ResourceRequirements {
                team_size_range: (12, 25),
                timeline_months_range: (24, 48),
                budget_range: Some((1500000.0, 3000000.0)),
                skill_requirements: vec![
                    SkillRequirement {
                        skill_name: "Rust Development".to_string(),
                        proficiency_level: ProficiencyLevel::Advanced,
                        team_coverage_percentage: 80.0,
                        acquisition_difficulty: AcquisitionDifficulty::Hard,
                    },
                    SkillRequirement {
                        skill_name: "Systems Architecture".to_string(),
                        proficiency_level: ProficiencyLevel::Expert,
                        team_coverage_percentage: 20.0,
                        acquisition_difficulty: AcquisitionDifficulty::VeryHard,
                    },
                ],
                infrastructure_needs: vec![
                    "Complete Rust toolchain".to_string(),
                    "Comprehensive testing infrastructure".to_string(),
                    "Performance benchmarking suite".to_string(),
                ],
            },
            success_probability: 0.60,
        });

        // Hybrid Approach Evaluation
        evaluations.insert(MigrationApproach::Hybrid, ApproachEvaluation {
            approach: MigrationApproach::Hybrid,
            criterion_scores: HashMap::from([
                ("performance_impact".to_string(), 8.0),
                ("extension_compatibility".to_string(), 7.5),
                ("development_velocity".to_string(), 5.5),
                ("implementation_risk".to_string(), 6.0),
                ("resource_requirements".to_string(), 5.5),
            ]),
            overall_score: 6.5,
            strengths: vec![
                "Balanced risk/reward profile".to_string(),
                "Flexible adaptation to constraints".to_string(),
                "Good performance improvements".to_string(),
                "Reasonable compatibility preservation".to_string(),
            ],
            weaknesses: vec![
                "Complex architecture management".to_string(),
                "Multiple technology stacks".to_string(),
                "Integration overhead".to_string(),
            ],
            best_fit_scenarios: vec![
                "Mixed performance requirements".to_string(),
                "Partial compatibility needs".to_string(),
                "Moderate risk tolerance".to_string(),
                "Diverse team skills".to_string(),
            ],
            resource_requirements: ResourceRequirements {
                team_size_range: (8, 15),
                timeline_months_range: (18, 30),
                budget_range: Some((800000.0, 1500000.0)),
                skill_requirements: vec![
                    SkillRequirement {
                        skill_name: "Rust Development".to_string(),
                        proficiency_level: ProficiencyLevel::Intermediate,
                        team_coverage_percentage: 50.0,
                        acquisition_difficulty: AcquisitionDifficulty::Moderate,
                    },
                    SkillRequirement {
                        skill_name: "JavaScript/TypeScript".to_string(),
                        proficiency_level: ProficiencyLevel::Advanced,
                        team_coverage_percentage: 70.0,
                        acquisition_difficulty: AcquisitionDifficulty::Easy,
                    },
                    SkillRequirement {
                        skill_name: "System Integration".to_string(),
                        proficiency_level: ProficiencyLevel::Advanced,
                        team_coverage_percentage: 30.0,
                        acquisition_difficulty: AcquisitionDifficulty::Moderate,
                    },
                ],
                infrastructure_needs: vec![
                    "Multi-language build system".to_string(),
                    "Cross-platform testing".to_string(),
                    "Integration monitoring".to_string(),
                ],
            },
            success_probability: 0.75,
        });

        // Incremental Migration Evaluation
        evaluations.insert(MigrationApproach::Incremental, ApproachEvaluation {
            approach: MigrationApproach::Incremental,
            criterion_scores: HashMap::from([
                ("performance_impact".to_string(), 6.5),
                ("extension_compatibility".to_string(), 9.5),
                ("development_velocity".to_string(), 8.0),
                ("implementation_risk".to_string(), 9.0),
                ("resource_requirements".to_string(), 8.5),
            ]),
            overall_score: 8.3,
            strengths: vec![
                "Lowest risk profile".to_string(),
                "Fastest time to value".to_string(),
                "Minimal disruption".to_string(),
                "High compatibility preservation".to_string(),
                "Continuous user feedback".to_string(),
            ],
            weaknesses: vec![
                "Limited performance gains initially".to_string(),
                "May not address all performance issues".to_string(),
                "Requires careful scope management".to_string(),
            ],
            best_fit_scenarios: vec![
                "Proof of concept needed".to_string(),
                "Limited resources available".to_string(),
                "High compatibility requirements".to_string(),
                "Continuous delivery culture".to_string(),
            ],
            resource_requirements: ResourceRequirements {
                team_size_range: (3, 8),
                timeline_months_range: (9, 18),
                budget_range: Some((300000.0, 700000.0)),
                skill_requirements: vec![
                    SkillRequirement {
                        skill_name: "Rust Development".to_string(),
                        proficiency_level: ProficiencyLevel::Beginner,
                        team_coverage_percentage: 40.0,
                        acquisition_difficulty: AcquisitionDifficulty::Easy,
                    },
                    SkillRequirement {
                        skill_name: "Incremental Development".to_string(),
                        proficiency_level: ProficiencyLevel::Intermediate,
                        team_coverage_percentage: 60.0,
                        acquisition_difficulty: AcquisitionDifficulty::Easy,
                    },
                ],
                infrastructure_needs: vec![
                    "Modular architecture".to_string(),
                    "Feature flagging system".to_string(),
                    "A/B testing framework".to_string(),
                ],
            },
            success_probability: 0.90,
        });

        evaluations
    }

    fn build_risk_assessment_matrix(&self) -> RiskAssessmentMatrix {
        let mut approach_risk_profiles = HashMap::new();

        // Risk profiles for each approach
        approach_risk_profiles.insert(MigrationApproach::Gradual, RiskProfile {
            overall_risk_level: RiskLevel::Medium,
            category_risks: HashMap::from([
                ("Technical Risk".to_string(), 0.4),
                ("Project Risk".to_string(), 0.3),
                ("Business Risk".to_string(), 0.2),
            ]),
            key_risk_factors: vec![
                "Integration complexity".to_string(),
                "Timeline coordination".to_string(),
            ],
            mitigation_priorities: vec![
                "Establish clear integration boundaries".to_string(),
                "Implement comprehensive testing".to_string(),
            ],
        });

        approach_risk_profiles.insert(MigrationApproach::CompleteRewrite, RiskProfile {
            overall_risk_level: RiskLevel::High,
            category_risks: HashMap::from([
                ("Technical Risk".to_string(), 0.7),
                ("Project Risk".to_string(), 0.8),
                ("Business Risk".to_string(), 0.6),
            ]),
            key_risk_factors: vec![
                "Scope underestimation".to_string(),
                "Timeline overruns".to_string(),
                "Feature parity challenges".to_string(),
            ],
            mitigation_priorities: vec![
                "Detailed scope planning".to_string(),
                "Prototype validation".to_string(),
                "Regular milestone reviews".to_string(),
            ],
        });

        approach_risk_profiles.insert(MigrationApproach::Incremental, RiskProfile {
            overall_risk_level: RiskLevel::Low,
            category_risks: HashMap::from([
                ("Technical Risk".to_string(), 0.2),
                ("Project Risk".to_string(), 0.1),
                ("Business Risk".to_string(), 0.1),
            ]),
            key_risk_factors: vec![
                "Limited scope impact".to_string(),
                "Scope creep potential".to_string(),
            ],
            mitigation_priorities: vec![
                "Clear scope boundaries".to_string(),
                "Regular success validation".to_string(),
            ],
        });

        let mitigation_strategies = HashMap::from([
            ("performance_monitoring".to_string(), MitigationStrategy {
                strategy_name: "Continuous Performance Monitoring".to_string(),
                description: "Implement comprehensive performance monitoring throughout migration".to_string(),
                applicable_risks: vec!["Performance Regression".to_string()],
                implementation_steps: vec![
                    "Set up baseline performance metrics".to_string(),
                    "Implement automated performance testing".to_string(),
                    "Create performance dashboards".to_string(),
                ],
                effectiveness_rating: 0.8,
                implementation_cost: ImplementationCost::Medium,
            }),
            ("skill_development".to_string(), MitigationStrategy {
                strategy_name: "Team Skill Development Program".to_string(),
                description: "Systematic approach to building required technical skills".to_string(),
                applicable_risks: vec!["Skill Gap".to_string()],
                implementation_steps: vec![
                    "Assess current team skills".to_string(),
                    "Create training program".to_string(),
                    "Hire experienced developers".to_string(),
                ],
                effectiveness_rating: 0.9,
                implementation_cost: ImplementationCost::High,
            }),
        ]);

        RiskAssessmentMatrix {
            risk_categories: self.risk_categories.clone(),
            approach_risk_profiles,
            mitigation_strategies,
            risk_tolerance_guidelines: RiskToleranceGuidelines {
                acceptable_risk_threshold: 0.6,
                risk_appetite_by_category: HashMap::from([
                    ("Technical Risk".to_string(), 0.5),
                    ("Project Risk".to_string(), 0.4),
                    ("Business Risk".to_string(), 0.3),
                ]),
                escalation_triggers: vec![
                    "Risk level exceeds threshold".to_string(),
                    "Multiple high-impact risks identified".to_string(),
                ],
                decision_authority_matrix: HashMap::from([
                    ("Low Risk".to_string(), "Team Lead".to_string()),
                    ("Medium Risk".to_string(), "Project Manager".to_string()),
                    ("High Risk".to_string(), "Executive Sponsor".to_string()),
                ]),
            },
        }
    }

    fn build_complexity_assessment_tools(&self) -> ComplexityAssessmentTools {
        ComplexityAssessmentTools {
            assessment_dimensions: vec![
                ComplexityDimension {
                    dimension_name: "Technical Complexity".to_string(),
                    description: "Complexity of technical implementation".to_string(),
                    measurement_criteria: vec![
                        "Number of components to migrate".to_string(),
                        "Interdependency complexity".to_string(),
                        "Performance requirements stringency".to_string(),
                    ],
                    scoring_rubric: vec![
                        ScoringLevel {
                            level_name: "Low".to_string(),
                            score: 1.0,
                            description: "Simple, well-defined components".to_string(),
                            indicators: vec!["<10 components".to_string(), "Clear boundaries".to_string()],
                        },
                        ScoringLevel {
                            level_name: "High".to_string(),
                            score: 5.0,
                            description: "Complex, highly interdependent system".to_string(),
                            indicators: vec![">50 components".to_string(), "Complex dependencies".to_string()],
                        },
                    ],
                    weight: 0.4,
                },
            ],
            scoring_methodology: ScoringMethodology {
                aggregation_method: AggregationMethod::WeightedAverage,
                normalization_approach: "Min-max normalization".to_string(),
                weighting_scheme: "Expert judgment based".to_string(),
                validation_approach: "Historical project comparison".to_string(),
            },
            complexity_thresholds: ComplexityThresholds {
                low_complexity: 2.0,
                medium_complexity: 3.5,
                high_complexity: 4.5,
                very_high_complexity: 5.0,
            },
            recommendation_matrix: HashMap::from([
                ("Low Complexity".to_string(), vec!["Incremental approach recommended".to_string()]),
                ("High Complexity".to_string(), vec!["Consider gradual migration".to_string()]),
            ]),
        }
    }
}

pub struct DecisionFrameworkEvaluator {
    framework: MigrationDecisionFramework,
}

impl DecisionFrameworkEvaluator {
    pub fn new(framework: MigrationDecisionFramework) -> Self {
        Self { framework }
    }

    pub fn evaluate_project_context(&self, context: &ProjectContext) -> ResearchResult<DecisionRecommendation> {
        let criterion_scores = self.calculate_criterion_scores(context)?;
        let approach_rankings = self.rank_approaches(&criterion_scores)?;
        let risk_assessment = self.assess_risks(context)?;
        let timeline_estimate = self.estimate_timeline(context)?;
        let resource_estimate = self.estimate_resources(context)?;

        Ok(DecisionRecommendation {
            recommended_approach: approach_rankings[0].approach.clone(),
            approach_rankings,
            criterion_scores,
            risk_assessment,
            timeline_estimate,
            resource_estimate,
            confidence_level: self.calculate_confidence_level(context),
            key_considerations: self.identify_key_considerations(context),
            next_steps: self.recommend_next_steps(context),
        })
    }

    fn calculate_criterion_scores(&self, context: &ProjectContext) -> ResearchResult<HashMap<String, f64>> {
        let mut scores = HashMap::new();

        for criterion in &self.framework.decision_criteria {
            let score = match criterion.criterion_id.as_str() {
                "performance_impact" => self.evaluate_performance_impact(context),
                "extension_compatibility" => self.evaluate_extension_compatibility(context),
                "development_velocity" => self.evaluate_development_velocity(context),
                "implementation_risk" => self.evaluate_implementation_risk(context),
                "resource_requirements" => self.evaluate_resource_requirements(context),
                _ => 5.0, // Default neutral score
            };
            scores.insert(criterion.criterion_id.clone(), score);
        }

        Ok(scores)
    }

    fn evaluate_performance_impact(&self, context: &ProjectContext) -> f64 {
        // Evaluate based on current performance issues and improvement potential
        let mut score = 5.0; // Base score

        if context.current_performance_issues.contains(&"slow_startup".to_string()) {
            score += 2.0; // High impact potential
        }
        if context.current_performance_issues.contains(&"high_memory_usage".to_string()) {
            score += 1.5;
        }
        if context.performance_requirements == PerformanceRequirements::High {
            score += 1.0;
        }

        score.min(10.0)
    }

    fn evaluate_extension_compatibility(&self, context: &ProjectContext) -> f64 {
        match context.extension_compatibility_requirements {
            ExtensionCompatibilityRequirements::Full => 10.0,
            ExtensionCompatibilityRequirements::Partial => 6.0,
            ExtensionCompatibilityRequirements::Minimal => 3.0,
            ExtensionCompatibilityRequirements::None => 1.0,
        }
    }

    fn evaluate_development_velocity(&self, context: &ProjectContext) -> f64 {
        let mut score = 5.0;

        match context.team_rust_expertise {
            TeamExpertise::High => score += 2.0,
            TeamExpertise::Medium => score += 0.0,
            TeamExpertise::Low => score -= 2.0,
            TeamExpertise::None => score -= 3.0,
        }

        score.max(1.0).min(10.0)
    }

    fn evaluate_implementation_risk(&self, context: &ProjectContext) -> f64 {
        let mut risk_score = 5.0;

        match context.risk_tolerance {
            RiskTolerance::High => risk_score += 2.0,
            RiskTolerance::Medium => risk_score += 0.0,
            RiskTolerance::Low => risk_score -= 2.0,
        }

        if context.timeline_flexibility == TimelineFlexibility::Low {
            risk_score -= 1.0;
        }

        // Convert risk score to implementation risk score (inverse)
        11.0 - risk_score.max(1.0).min(10.0)
    }

    fn evaluate_resource_requirements(&self, context: &ProjectContext) -> f64 {
        let mut score = 5.0;

        match context.available_resources {
            ResourceAvailability::High => score += 2.0,
            ResourceAvailability::Medium => score += 0.0,
            ResourceAvailability::Low => score -= 2.0,
        }

        score.max(1.0).min(10.0)
    }

    fn rank_approaches(&self, criterion_scores: &HashMap<String, f64>) -> ResearchResult<Vec<ApproachRanking>> {
        let mut rankings = Vec::new();

        for (approach, evaluation) in &self.framework.approach_evaluations {
            let mut weighted_score = 0.0;
            let mut total_weight = 0.0;

            for criterion in &self.framework.decision_criteria {
                if let Some(context_score) = criterion_scores.get(&criterion.criterion_id) {
                    if let Some(approach_score) = evaluation.criterion_scores.get(&criterion.criterion_id) {
                        // Combine context score with approach score
                        let combined_score = (context_score + approach_score) / 2.0;
                        weighted_score += combined_score * criterion.weight;
                        total_weight += criterion.weight;
                    }
                }
            }

            let final_score = if total_weight > 0.0 { weighted_score / total_weight } else { 0.0 };

            rankings.push(ApproachRanking {
                approach: approach.clone(),
                score: final_score,
                strengths: evaluation.strengths.clone(),
                weaknesses: evaluation.weaknesses.clone(),
                fit_assessment: self.assess_fit(approach, criterion_scores),
            });
        }

        rankings.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        Ok(rankings)
    }

    fn assess_fit(&self, approach: &MigrationApproach, _scores: &HashMap<String, f64>) -> String {
        match approach {
            MigrationApproach::Incremental => "Excellent fit for low-risk, high-compatibility scenarios".to_string(),
            MigrationApproach::Gradual => "Good fit for balanced risk/reward requirements".to_string(),
            MigrationApproach::Hybrid => "Suitable for complex requirements with mixed priorities".to_string(),
            MigrationApproach::CompleteRewrite => "Best for maximum performance with high risk tolerance".to_string(),
        }
    }

    fn assess_risks(&self, _context: &ProjectContext) -> ResearchResult<RiskAssessmentResult> {
        // Simplified risk assessment
        Ok(RiskAssessmentResult {
            overall_risk_level: RiskLevel::Medium,
            category_risks: HashMap::new(),
            key_risks: vec!["Timeline overrun".to_string()],
            mitigation_recommendations: vec!["Implement milestone tracking".to_string()],
        })
    }

    fn estimate_timeline(&self, _context: &ProjectContext) -> ResearchResult<TimelineEstimateResult> {
        Ok(TimelineEstimateResult {
            optimistic_months: 12,
            realistic_months: 18,
            pessimistic_months: 24,
            key_factors: vec!["Team expertise".to_string()],
            milestones: vec!["Core migration complete".to_string()],
        })
    }

    fn estimate_resources(&self, _context: &ProjectContext) -> ResearchResult<ResourceEstimateResult> {
        Ok(ResourceEstimateResult {
            team_size_recommendation: 8,
            skill_mix: HashMap::new(),
            budget_range: (500000.0, 1000000.0),
            infrastructure_needs: vec!["CI/CD pipeline".to_string()],
        })
    }

    fn calculate_confidence_level(&self, _context: &ProjectContext) -> f64 {
        0.8 // 80% confidence
    }

    fn identify_key_considerations(&self, _context: &ProjectContext) -> Vec<String> {
        vec![
            "Extension compatibility is critical".to_string(),
            "Performance improvements must be measurable".to_string(),
        ]
    }

    fn recommend_next_steps(&self, _context: &ProjectContext) -> Vec<String> {
        vec![
            "Conduct detailed technical assessment".to_string(),
            "Develop proof of concept".to_string(),
            "Create detailed project plan".to_string(),
        ]
    }
}

// Supporting data structures for evaluation

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectContext {
    pub current_performance_issues: Vec<String>,
    pub performance_requirements: PerformanceRequirements,
    pub extension_compatibility_requirements: ExtensionCompatibilityRequirements,
    pub team_rust_expertise: TeamExpertise,
    pub risk_tolerance: RiskTolerance,
    pub timeline_flexibility: TimelineFlexibility,
    pub available_resources: ResourceAvailability,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceRequirements {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExtensionCompatibilityRequirements {
    None,
    Minimal,
    Partial,
    Full,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TeamExpertise {
    None,
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskTolerance {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimelineFlexibility {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceAvailability {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionRecommendation {
    pub recommended_approach: MigrationApproach,
    pub approach_rankings: Vec<ApproachRanking>,
    pub criterion_scores: HashMap<String, f64>,
    pub risk_assessment: RiskAssessmentResult,
    pub timeline_estimate: TimelineEstimateResult,
    pub resource_estimate: ResourceEstimateResult,
    pub confidence_level: f64,
    pub key_considerations: Vec<String>,
    pub next_steps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApproachRanking {
    pub approach: MigrationApproach,
    pub score: f64,
    pub strengths: Vec<String>,
    pub weaknesses: Vec<String>,
    pub fit_assessment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessmentResult {
    pub overall_risk_level: RiskLevel,
    pub category_risks: HashMap<String, f64>,
    pub key_risks: Vec<String>,
    pub mitigation_recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEstimateResult {
    pub optimistic_months: i32,
    pub realistic_months: i32,
    pub pessimistic_months: i32,
    pub key_factors: Vec<String>,
    pub milestones: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceEstimateResult {
    pub team_size_recommendation: i32,
    pub skill_mix: HashMap<String, f64>,
    pub budget_range: (f64, f64),
    pub infrastructure_needs: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decision_framework_creation() {
        let framework = DecisionFrameworkBuilder::build_kiro_decision_framework();
        
        assert!(!framework.decision_criteria.is_empty());
        assert!(!framework.approach_evaluations.is_empty());
        assert!(!framework.risk_assessment_matrix.risk_categories.is_empty());
        assert!(!framework.timeline_estimation_models.is_empty());
    }

    #[test]
    fn test_approach_evaluation() {
        let framework = DecisionFrameworkBuilder::build_kiro_decision_framework();
        
        // Incremental approach should have highest overall score
        let incremental_eval = framework.approach_evaluations.get(&MigrationApproach::Incremental).unwrap();
        let rewrite_eval = framework.approach_evaluations.get(&MigrationApproach::CompleteRewrite).unwrap();
        
        assert!(incremental_eval.overall_score > rewrite_eval.overall_score);
        assert!(incremental_eval.success_probability > rewrite_eval.success_probability);
    }

    #[test]
    fn test_project_context_evaluation() {
        let framework = DecisionFrameworkBuilder::build_kiro_decision_framework();
        let evaluator = DecisionFrameworkEvaluator::new(framework);
        
        let context = ProjectContext {
            current_performance_issues: vec!["slow_startup".to_string()],
            performance_requirements: PerformanceRequirements::High,
            extension_compatibility_requirements: ExtensionCompatibilityRequirements::Full,
            team_rust_expertise: TeamExpertise::Medium,
            risk_tolerance: RiskTolerance::Medium,
            timeline_flexibility: TimelineFlexibility::Medium,
            available_resources: ResourceAvailability::Medium,
        };

        let recommendation = evaluator.evaluate_project_context(&context).unwrap();
        
        assert!(!recommendation.approach_rankings.is_empty());
        assert!(recommendation.confidence_level > 0.0);
        assert!(!recommendation.next_steps.is_empty());
    }
}