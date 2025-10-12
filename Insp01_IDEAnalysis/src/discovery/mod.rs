pub mod github;
pub mod web_scraper;
pub mod project_evaluator;

use crate::models::{Project, ProjectCategory, ResearchSource};
use crate::errors::Result;
use crate::Database;

pub use github::GitHubDiscovery;
pub use web_scraper::WebScraper;
pub use project_evaluator::ProjectEvaluator;

/// Main discovery engine that coordinates all discovery methods
pub struct DiscoveryEngine {
    github: GitHubDiscovery,
    web_scraper: WebScraper,
    evaluator: ProjectEvaluator,
    database: Database,
}

impl DiscoveryEngine {
    pub fn new(github_token: Option<String>, database: Database) -> Self {
        Self {
            github: GitHubDiscovery::new(github_token),
            web_scraper: WebScraper::new(),
            evaluator: ProjectEvaluator::new(),
            database,
        }
    }

    /// Discover projects for a specific category
    pub async fn discover_projects(&self, category: ProjectCategory, keywords: &[String]) -> Result<Vec<Project>> {
        let mut discovered_projects = Vec::new();

        // GitHub discovery
        let github_projects = self.github.search_repositories(keywords, &category).await?;
        discovered_projects.extend(github_projects);

        // Web scraping for additional sources
        let web_projects = self.web_scraper.discover_projects(keywords, &category).await?;
        discovered_projects.extend(web_projects);

        // Evaluate and filter projects
        let mut evaluated_projects = Vec::new();
        for project in discovered_projects {
            if self.evaluator.meets_criteria(&project, &category).await? {
                evaluated_projects.push(project);
            }
        }

        // Store discovered projects in database
        for project in &evaluated_projects {
            if let Err(e) = self.database.create_project(project).await {
                tracing::warn!("Failed to store project {}: {}", project.name, e);
            }
        }

        Ok(evaluated_projects)
    }

    /// Discover IDE migration projects
    pub async fn discover_ide_migrations(&self) -> Result<Vec<Project>> {
        let keywords = vec![
            "electron to native".to_string(),
            "ide migration".to_string(),
            "electron alternative".to_string(),
            "native ide".to_string(),
            "performance migration".to_string(),
        ];

        self.discover_projects(ProjectCategory::IdeMigration, &keywords).await
    }

    /// Discover VS Code forks
    pub async fn discover_vscode_forks(&self) -> Result<Vec<Project>> {
        let keywords = vec![
            "vscode fork".to_string(),
            "visual studio code fork".to_string(),
            "code-oss".to_string(),
            "vscode alternative".to_string(),
            "custom ide".to_string(),
        ];

        self.discover_projects(ProjectCategory::VscodeFork, &keywords).await
    }

    /// Discover Rust IDEs
    pub async fn discover_rust_ides(&self) -> Result<Vec<Project>> {
        let keywords = vec![
            "rust ide".to_string(),
            "rust editor".to_string(),
            "native rust development".to_string(),
            "rust gui application".to_string(),
            "tauri ide".to_string(),
        ];

        self.discover_projects(ProjectCategory::RustIde, &keywords).await
    }

    /// Discover AI IDEs
    pub async fn discover_ai_ides(&self) -> Result<Vec<Project>> {
        let keywords = vec![
            "ai ide".to_string(),
            "ai code editor".to_string(),
            "copilot alternative".to_string(),
            "ai development environment".to_string(),
            "intelligent code editor".to_string(),
        ];

        self.discover_projects(ProjectCategory::AiIde, &keywords).await
    }

    /// Discover WASM applications
    pub async fn discover_wasm_apps(&self) -> Result<Vec<Project>> {
        let keywords = vec![
            "wasm ide".to_string(),
            "webassembly editor".to_string(),
            "wasm development tools".to_string(),
            "browser ide".to_string(),
            "wasm gui".to_string(),
        ];

        self.discover_projects(ProjectCategory::WasmApp, &keywords).await
    }

    /// Run comprehensive discovery for all categories
    pub async fn run_comprehensive_discovery(&self) -> Result<()> {
        tracing::info!("Starting comprehensive project discovery");

        let categories = vec![
            ("IDE Migrations", ProjectCategory::IdeMigration),
            ("VS Code Forks", ProjectCategory::VscodeFork),
            ("Rust IDEs", ProjectCategory::RustIde),
            ("AI IDEs", ProjectCategory::AiIde),
            ("WASM Apps", ProjectCategory::WasmApp),
        ];

        for (name, category) in categories {
            tracing::info!("Discovering {} projects", name);
            
            let projects = match category {
                ProjectCategory::IdeMigration => self.discover_ide_migrations().await?,
                ProjectCategory::VscodeFork => self.discover_vscode_forks().await?,
                ProjectCategory::RustIde => self.discover_rust_ides().await?,
                ProjectCategory::AiIde => self.discover_ai_ides().await?,
                ProjectCategory::WasmApp => self.discover_wasm_apps().await?,
                _ => Vec::new(),
            };

            tracing::info!("Discovered {} {} projects", projects.len(), name);
            
            // Update progress tracking
            self.database.update_research_progress(category, projects.len() as i32).await?;
        }

        tracing::info!("Comprehensive discovery completed");
        Ok(())
    }
}