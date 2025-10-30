use static_analysis_phase2::{Database, Result};
use static_analysis_phase2::discovery::DiscoveryEngine;
use static_analysis_phase2::validation::ValidationFramework;
use tracing::{info, error};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    info!("Starting Static Analysis Phase 2 - Research Data Management System");

    // Get database URL from environment or use default
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://localhost/research_db".to_string());

    // Get GitHub token if available
    let github_token = env::var("GITHUB_TOKEN").ok();

    // Initialize database
    info!("Connecting to database: {}", database_url);
    let database = Database::new(&database_url).await?;

    // Initialize discovery engine
    info!("Initializing discovery engine");
    let discovery_engine = DiscoveryEngine::new(github_token, database.clone());

    // Initialize validation framework
    info!("Initializing validation framework");
    let validation_framework = ValidationFramework::new();

    // Run comprehensive discovery
    info!("Starting comprehensive project discovery");
    if let Err(e) = discovery_engine.run_comprehensive_discovery().await {
        error!("Discovery failed: {}", e);
        return Err(e);
    }

    // Get research progress
    let progress = database.get_research_progress().await?;
    info!("Research progress:");
    for p in progress {
        info!("  {:?}: {}/{} ({:.1}%)", 
            p.category, p.current_count, p.target_count, p.completion_percentage);
    }

    // Get database statistics
    let stats = database.get_statistics().await?;
    info!("Database statistics: {}", serde_json::to_string_pretty(&stats)?);

    info!("Research data management system initialized successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use static_analysis_phase2::models::*;

    #[tokio::test]
    async fn test_database_operations() -> Result<()> {
        // Use in-memory database for testing
        let database = Database::new("postgresql://localhost/test_research_db").await?;

        // Create a test project
        let project = Project::new(
            "Test Project".to_string(),
            "https://github.com/test/project".to_string(),
            ProjectCategory::RustIde,
        );

        // Test project creation
        database.create_project(&project).await?;

        // Test project retrieval
        let retrieved = database.get_project(project.id).await?;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "Test Project");

        // Test search
        let search_results = database.search_projects("Test", None).await?;
        assert!(!search_results.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_validation_framework() -> Result<()> {
        let validation_framework = ValidationFramework::new();

        // Create test data
        let test_data = serde_json::json!({
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "name": "Test Project",
            "url": "https://github.com/test/project",
            "category": "rust_ide",
            "status": "discovered",
            "description": "A test project for validation",
            "stars": 100,
            "metadata": {
                "language": "rust",
                "forks_count": 20
            }
        });

        let entity_id = uuid::Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();

        // Test validation
        let results = validation_framework
            .validate_entity("project", entity_id, &test_data)
            .await?;

        assert!(!results.is_empty());
        
        // Check that we have different types of validation results
        let validation_types: std::collections::HashSet<String> = results
            .iter()
            .map(|r| r.validation_type.clone())
            .collect();
        
        assert!(validation_types.contains("completeness"));
        assert!(validation_types.contains("accuracy"));

        Ok(())
    }
}