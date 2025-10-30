// Research module for IDE migration project analysis
// Implements systematic data collection and analysis framework

pub mod project_discovery;
pub mod data_collector;
pub mod project_evaluator;
pub mod migration_analyzer;
pub mod performance_analyzer;
pub mod analysis_report;
pub mod decision_framework;
pub mod optimization_patterns;

#[cfg(test)]
mod integration_test;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc, NaiveDate};
use uuid::Uuid;

/// Core data structures for migration project research

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationProject {
    pub id: Option<i32>,
    pub name: String,
    pub url: String,
    pub github_url: Option<String>,
    pub category: ProjectCategory,
    pub status: ProjectStatus,
    pub last_updated: Option<NaiveDate>,
    pub stars: i32,
    pub contributors: i32,
    pub technical_details: Option<TechnicalDetails>,
    pub analysis: Option<ProjectAnalysis>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectCategory {
    IdeMigration,
    VsCodeFork,
    RustIde,
    AiIde,
    WasmApp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectStatus {
    Active,
    Archived,
    Deprecated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicalDetails {
    pub source_technology: SourceTechnology,
    pub target_technology: TargetTechnology,
    pub migration_approach: MigrationApproach,
    pub architecture_patterns: Vec<String>,
    pub performance_metrics: HashMap<String, PerformanceMetric>,
    pub migration_timeline_months: Option<i32>,
    pub team_size: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceTechnology {
    Electron,
    Native,
    Web,
    Java,
    DotNet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TargetTechnology {
    Rust,
    Cpp,
    Go,
    Wasm,
    Native,
    Tauri,
    Flutter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MigrationApproach {
    Gradual,
    CompleteRewrite,
    Hybrid,
    Incremental,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetric {
    pub before_value: Option<f64>,
    pub after_value: Option<f64>,
    pub unit: String,
    pub improvement_percentage: Option<f64>,
    pub measurement_context: Option<String>,
    pub source_url: Option<String>,
    pub verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectAnalysis {
    pub strengths: Vec<String>,
    pub weaknesses: Vec<String>,
    pub lessons_learned: Vec<String>,
    pub applicability_to_kiro: ApplicabilityLevel,
    pub confidence_level: ConfidenceLevel,
    pub technical_score: i32,
    pub adoption_score: i32,
    pub sustainability_score: i32,
    pub relevance_score: i32,
    pub overall_score: f64,
    pub analysis_notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApplicabilityLevel {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfidenceLevel {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationStrategy {
    pub strategy_name: String,
    pub description: String,
    pub phases: Vec<MigrationPhase>,
    pub challenges: Vec<TechnicalChallenge>,
    pub solutions: Vec<String>,
    pub timeline_estimate: Option<String>,
    pub resource_requirements: HashMap<String, String>,
    pub success_factors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationPhase {
    pub name: String,
    pub description: String,
    pub duration_estimate: Option<String>,
    pub deliverables: Vec<String>,
    pub risks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicalChallenge {
    pub category: String,
    pub description: String,
    pub solution_approach: Option<String>,
    pub implementation_details: Option<String>,
    pub outcome: ChallengeOutcome,
    pub lessons_learned: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChallengeOutcome {
    Successful,
    Partial,
    Failed,
    Ongoing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchSource {
    pub source_type: SourceType,
    pub source_url: String,
    pub source_title: Option<String>,
    pub author: Option<String>,
    pub publication_date: Option<NaiveDate>,
    pub reliability_score: i32,
    pub content_summary: Option<String>,
    pub key_insights: Vec<String>,
    pub verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceType {
    Github,
    Documentation,
    BlogPost,
    ConferenceTalk,
    Paper,
    Interview,
    ForumPost,
}

/// Research evaluation criteria and scoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationCriteria {
    pub technical_weight: f64,
    pub adoption_weight: f64,
    pub sustainability_weight: f64,
    pub relevance_weight: f64,
}

impl Default for EvaluationCriteria {
    fn default() -> Self {
        Self {
            technical_weight: 0.3,
            adoption_weight: 0.2,
            sustainability_weight: 0.2,
            relevance_weight: 0.3,
        }
    }
}

/// Research insights and patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchInsights {
    pub key_patterns: Vec<String>,
    pub success_factors: Vec<String>,
    pub risk_factors: Vec<String>,
    pub recommendations: Vec<String>,
    pub confidence_level: ConfidenceLevel,
}

/// Error types for research operations
#[derive(Debug, thiserror::Error)]
pub enum ResearchError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Invalid project data: {0}")]
    InvalidData(String),
    
    #[error("Source verification failed: {0}")]
    SourceVerification(String),
    
    #[error("Analysis error: {0}")]
    Analysis(String),
}

pub type ResearchResult<T> = Result<T, ResearchError>;