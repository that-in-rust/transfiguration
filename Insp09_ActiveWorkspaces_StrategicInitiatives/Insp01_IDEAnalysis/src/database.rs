use crate::models::*;
use crate::errors::{Result, ResearchError};
use sqlx::{PgPool, Row};
use uuid::Uuid;

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    /// Create a new database connection
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPool::connect(database_url).await?;
        
        // Run migrations
        sqlx::migrate!("./migrations").run(&pool).await?;
        
        Ok(Self { pool })
    }

    /// Create a new project
    pub async fn create_project(&self, project: &Project) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO projects (id, name, url, category, status, description, stars, contributors, last_updated, metadata, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            "#,
            project.id,
            project.name,
            project.url,
            project.category as ProjectCategory,
            project.status as ProjectStatus,
            project.description,
            project.stars,
            project.contributors,
            project.last_updated,
            project.metadata,
            project.created_at,
            project.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get project by ID
    pub async fn get_project(&self, id: Uuid) -> Result<Option<Project>> {
        let project = sqlx::query_as!(
            Project,
            "SELECT * FROM projects WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(project)
    }

    /// Search projects with full-text search
    pub async fn search_projects(&self, query: &str, category: Option<ProjectCategory>) -> Result<Vec<Project>> {
        let projects = match category {
            Some(cat) => {
                sqlx::query_as!(
                    Project,
                    r#"
                    SELECT * FROM projects 
                    WHERE category = $1 
                    AND to_tsvector('english', name || ' ' || COALESCE(description, '')) @@ plainto_tsquery('english', $2)
                    ORDER BY ts_rank(to_tsvector('english', name || ' ' || COALESCE(description, '')), plainto_tsquery('english', $2)) DESC
                    "#,
                    cat as ProjectCategory,
                    query
                )
                .fetch_all(&self.pool)
                .await?
            }
            None => {
                sqlx::query_as!(
                    Project,
                    r#"
                    SELECT * FROM projects 
                    WHERE to_tsvector('english', name || ' ' || COALESCE(description, '')) @@ plainto_tsquery('english', $1)
                    ORDER BY ts_rank(to_tsvector('english', name || ' ' || COALESCE(description, '')), plainto_tsquery('english', $1)) DESC
                    "#,
                    query
                )
                .fetch_all(&self.pool)
                .await?
            }
        };

        Ok(projects)
    }

    /// Get projects by category
    pub async fn get_projects_by_category(&self, category: ProjectCategory) -> Result<Vec<Project>> {
        let projects = sqlx::query_as!(
            Project,
            "SELECT * FROM projects WHERE category = $1 ORDER BY created_at DESC",
            category as ProjectCategory
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(projects)
    }

    /// Update project status
    pub async fn update_project_status(&self, id: Uuid, status: ProjectStatus) -> Result<()> {
        sqlx::query!(
            "UPDATE projects SET status = $1, updated_at = NOW() WHERE id = $2",
            status as ProjectStatus,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Create technical analysis
    pub async fn create_technical_analysis(&self, analysis: &TechnicalAnalysis) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO technical_analyses (
                id, project_id, source_technology, target_technology, migration_approach,
                architecture_patterns, performance_metrics, strengths, weaknesses, lessons_learned,
                applicability_to_kiro, confidence_level, analysis_notes, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            "#,
            analysis.id,
            analysis.project_id,
            analysis.source_technology,
            analysis.target_technology,
            analysis.migration_approach,
            &analysis.architecture_patterns,
            analysis.performance_metrics,
            &analysis.strengths,
            &analysis.weaknesses,
            &analysis.lessons_learned,
            analysis.applicability_to_kiro as ConfidenceLevel,
            analysis.confidence_level as ConfidenceLevel,
            analysis.analysis_notes,
            analysis.created_at,
            analysis.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get technical analysis for project
    pub async fn get_technical_analysis(&self, project_id: Uuid) -> Result<Option<TechnicalAnalysis>> {
        let analysis = sqlx::query_as!(
            TechnicalAnalysis,
            "SELECT * FROM technical_analyses WHERE project_id = $1",
            project_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(analysis)
    }

    /// Create research source
    pub async fn create_research_source(&self, source: &ResearchSource) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO research_sources (
                id, project_id, url, source_type, title, author, published_date,
                reliability_score, verification_status, content_hash, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            "#,
            source.id,
            source.project_id,
            source.url,
            source.source_type,
            source.title,
            source.author,
            source.published_date,
            source.reliability_score,
            source.verification_status,
            source.content_hash,
            source.created_at,
            source.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get research sources for project
    pub async fn get_research_sources(&self, project_id: Uuid) -> Result<Vec<ResearchSource>> {
        let sources = sqlx::query_as!(
            ResearchSource,
            "SELECT * FROM research_sources WHERE project_id = $1 ORDER BY created_at DESC",
            project_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(sources)
    }

    /// Create research finding
    pub async fn create_research_finding(&self, finding: &ResearchFinding) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO research_findings (
                id, project_id, category, title, description, evidence,
                confidence_level, tags, source_ids, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
            finding.id,
            finding.project_id,
            finding.category,
            finding.title,
            finding.description,
            finding.evidence,
            finding.confidence_level as ConfidenceLevel,
            &finding.tags,
            &finding.source_ids,
            finding.created_at,
            finding.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Search research findings
    pub async fn search_research_findings(&self, query: &str, category: Option<&str>) -> Result<Vec<ResearchFinding>> {
        let findings = match category {
            Some(cat) => {
                sqlx::query_as!(
                    ResearchFinding,
                    r#"
                    SELECT * FROM research_findings 
                    WHERE category = $1 
                    AND to_tsvector('english', title || ' ' || description) @@ plainto_tsquery('english', $2)
                    ORDER BY ts_rank(to_tsvector('english', title || ' ' || description), plainto_tsquery('english', $2)) DESC
                    "#,
                    cat,
                    query
                )
                .fetch_all(&self.pool)
                .await?
            }
            None => {
                sqlx::query_as!(
                    ResearchFinding,
                    r#"
                    SELECT * FROM research_findings 
                    WHERE to_tsvector('english', title || ' ' || description) @@ plainto_tsquery('english', $1)
                    ORDER BY ts_rank(to_tsvector('english', title || ' ' || description), plainto_tsquery('english', $1)) DESC
                    "#,
                    query
                )
                .fetch_all(&self.pool)
                .await?
            }
        };

        Ok(findings)
    }

    /// Get research progress
    pub async fn get_research_progress(&self) -> Result<Vec<ResearchProgress>> {
        let progress = sqlx::query_as!(
            ResearchProgress,
            "SELECT * FROM research_progress ORDER BY category"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(progress)
    }

    /// Update research progress
    pub async fn update_research_progress(&self, category: ProjectCategory, current_count: i32) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE research_progress 
            SET current_count = $1, 
                completion_percentage = (current_count::REAL / target_count::REAL) * 100,
                last_updated = NOW()
            WHERE category = $2
            "#,
            current_count,
            category as ProjectCategory
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Create validation result
    pub async fn create_validation_result(&self, validation: &ValidationResult) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO validation_results (
                id, entity_type, entity_id, validation_type, status, score,
                issues, recommendations, validated_at, validator
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            validation.id,
            validation.entity_type,
            validation.entity_id,
            validation.validation_type,
            validation.status,
            validation.score,
            &validation.issues,
            &validation.recommendations,
            validation.validated_at,
            validation.validator
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get validation results for entity
    pub async fn get_validation_results(&self, entity_type: &str, entity_id: Uuid) -> Result<Vec<ValidationResult>> {
        let results = sqlx::query_as!(
            ValidationResult,
            "SELECT * FROM validation_results WHERE entity_type = $1 AND entity_id = $2 ORDER BY validated_at DESC",
            entity_type,
            entity_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }

    /// Get database statistics
    pub async fn get_statistics(&self) -> Result<serde_json::Value> {
        let stats = sqlx::query!(
            r#"
            SELECT 
                (SELECT COUNT(*) FROM projects) as total_projects,
                (SELECT COUNT(*) FROM projects WHERE status = 'analyzed') as analyzed_projects,
                (SELECT COUNT(*) FROM technical_analyses) as technical_analyses,
                (SELECT COUNT(*) FROM research_sources) as research_sources,
                (SELECT COUNT(*) FROM research_findings) as research_findings,
                (SELECT COUNT(*) FROM validation_results WHERE status = 'passed') as passed_validations
            "#
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(serde_json::json!({
            "total_projects": stats.total_projects,
            "analyzed_projects": stats.analyzed_projects,
            "technical_analyses": stats.technical_analyses,
            "research_sources": stats.research_sources,
            "research_findings": stats.research_findings,
            "passed_validations": stats.passed_validations
        }))
    }
}