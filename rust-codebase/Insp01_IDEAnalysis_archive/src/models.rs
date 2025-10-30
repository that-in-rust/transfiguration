use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Project categories for research classification
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "project_category", rename_all = "snake_case")]
pub enum ProjectCategory {
    IdeMigration,
    VscodeFork,
    RustIde,
    AiIde,
    WasmApp,
    TechnicalPattern,
}

/// Project status for tracking research progress
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "project_status", rename_all = "snake_case")]
pub enum ProjectStatus {
    Discovered,
    Analyzing,
    Analyzed,
    Verified,
    Archived,
}

/// Confidence levels for research findings
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "confidence_level", rename_all = "snake_case")]
pub enum ConfidenceLevel {
    High,
    Medium,
    Low,
}

/// Main project entity for research database
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub category: ProjectCategory,
    pub status: ProjectStatus,
    pub description: Option<String>,
    pub stars: Option<i32>,
    pub contributors: Option<i32>,
    pub last_updated: Option<DateTime<Utc>>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Technical analysis data for projects
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TechnicalAnalysis {
    pub id: Uuid,
    pub project_id: Uuid,
    pub source_technology: Option<String>,
    pub target_technology: Option<String>,
    pub migration_approach: Option<String>,
    pub architecture_patterns: Vec<String>,
    pub performance_metrics: serde_json::Value,
    pub strengths: Vec<String>,
    pub weaknesses: Vec<String>,
    pub lessons_learned: Vec<String>,
    pub applicability_to_kiro: ConfidenceLevel,
    pub confidence_level: ConfidenceLevel,
    pub analysis_notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Research sources for attribution and verification
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ResearchSource {
    pub id: Uuid,
    pub project_id: Option<Uuid>,
    pub url: String,
    pub source_type: String, // "github", "documentation", "blog", "paper", etc.
    pub title: Option<String>,
    pub author: Option<String>,
    pub published_date: Option<DateTime<Utc>>,
    pub reliability_score: Option<f32>,
    pub verification_status: String, // "verified", "pending", "failed"
    pub content_hash: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Research findings and insights
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ResearchFinding {
    pub id: Uuid,
    pub project_id: Option<Uuid>,
    pub category: String, // "pattern", "success_factor", "failure_mode", "recommendation"
    pub title: String,
    pub description: String,
    pub evidence: serde_json::Value,
    pub confidence_level: ConfidenceLevel,
    pub tags: Vec<String>,
    pub source_ids: Vec<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Competitive analysis data
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CompetitiveAnalysis {
    pub id: Uuid,
    pub project_id: Uuid,
    pub ai_integration: serde_json::Value,
    pub extension_system: serde_json::Value,
    pub business_model: serde_json::Value,
    pub user_experience: serde_json::Value,
    pub performance_data: serde_json::Value,
    pub market_position: Option<String>,
    pub differentiation_factors: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Research progress tracking
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ResearchProgress {
    pub id: Uuid,
    pub category: ProjectCategory,
    pub target_count: i32,
    pub current_count: i32,
    pub completion_percentage: f32,
    pub quality_score: Option<f32>,
    pub last_updated: DateTime<Utc>,
    pub notes: Option<String>,
}

/// Data validation results
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ValidationResult {
    pub id: Uuid,
    pub entity_type: String, // "project", "analysis", "source", etc.
    pub entity_id: Uuid,
    pub validation_type: String, // "completeness", "accuracy", "bias", "quality"
    pub status: String, // "passed", "failed", "warning"
    pub score: Option<f32>,
    pub issues: Vec<String>,
    pub recommendations: Vec<String>,
    pub validated_at: DateTime<Utc>,
    pub validator: String,
}

impl Project {
    pub fn new(name: String, url: String, category: ProjectCategory) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            url,
            category,
            status: ProjectStatus::Discovered,
            description: None,
            stars: None,
            contributors: None,
            last_updated: None,
            metadata: serde_json::Value::Object(serde_json::Map::new()),
            created_at: now,
            updated_at: now,
        }
    }
}

impl TechnicalAnalysis {
    pub fn new(project_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            project_id,
            source_technology: None,
            target_technology: None,
            migration_approach: None,
            architecture_patterns: Vec::new(),
            performance_metrics: serde_json::Value::Object(serde_json::Map::new()),
            strengths: Vec::new(),
            weaknesses: Vec::new(),
            lessons_learned: Vec::new(),
            applicability_to_kiro: ConfidenceLevel::Medium,
            confidence_level: ConfidenceLevel::Medium,
            analysis_notes: None,
            created_at: now,
            updated_at: now,
        }
    }
}

impl ResearchSource {
    pub fn new(url: String, source_type: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            project_id: None,
            url,
            source_type,
            title: None,
            author: None,
            published_date: None,
            reliability_score: None,
            verification_status: "pending".to_string(),
            content_hash: None,
            created_at: now,
            updated_at: now,
        }
    }
}