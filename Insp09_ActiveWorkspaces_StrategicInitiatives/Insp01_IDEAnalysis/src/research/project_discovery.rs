// Project Discovery Engine for IDE Migration Research
// Systematically identifies and catalogs relevant migration projects

use super::{MigrationProject, ProjectCategory, ProjectStatus, ResearchResult, ResearchError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct ProjectDiscoveryEngine {
    github_client: Option<octocrab::Octocrab>,
    search_keywords: Vec<String>,
    exclusion_patterns: Vec<String>,
}

impl ProjectDiscoveryEngine {
    pub fn new() -> Self {
        Self {
            github_client: None,
            search_keywords: Self::default_search_keywords(),
            exclusion_patterns: Self::default_exclusion_patterns(),
        }
    }

    pub async fn with_github_token(token: String) -> ResearchResult<Self> {
        let client = octocrab::OctocrabBuilder::new()
            .personal_token(token)
            .build()?;
        
        Ok(Self {
            github_client: Some(client),
            search_keywords: Self::default_search_keywords(),
            exclusion_patterns: Self::default_exclusion_patterns(),
        })
    }

    fn default_search_keywords() -> Vec<String> {
        vec![
            // Electron migration keywords
            "electron migration".to_string(),
            "electron to native".to_string(),
            "electron to rust".to_string(),
            "electron to tauri".to_string(),
            "electron alternative".to_string(),
            "electron performance".to_string(),
            
            // IDE migration keywords
            "IDE migration".to_string(),
            "editor migration".to_string(),
            "vscode fork".to_string(),
            "vscode alternative".to_string(),
            
            // Technology-specific keywords
            "rust IDE".to_string(),
            "rust editor".to_string(),
            "wasm IDE".to_string(),
            "native IDE".to_string(),
            "tauri IDE".to_string(),
            
            // Performance-focused keywords
            "fast IDE".to_string(),
            "lightweight editor".to_string(),
            "performance IDE".to_string(),
            "memory efficient editor".to_string(),
        ]
    }

    fn default_exclusion_patterns() -> Vec<String> {
        vec![
            "tutorial".to_string(),
            "example".to_string(),
            "demo".to_string(),
            "test".to_string(),
            "playground".to_string(),
            "hello-world".to_string(),
        ]
    }

    /// Discover IDE migration projects using multiple search strategies
    pub async fn discover_migration_projects(&self) -> ResearchResult<Vec<MigrationProject>> {
        let mut discovered_projects = Vec::new();

        // Strategy 1: GitHub search for known migration projects
        if let Some(github_projects) = self.search_github_projects().await? {
            discovered_projects.extend(github_projects);
        }

        // Strategy 2: Curated list of known migration projects
        let curated_projects = self.get_curated_migration_projects();
        discovered_projects.extend(curated_projects);

        // Strategy 3: Community recommendations and references
        let community_projects = self.get_community_recommended_projects();
        discovered_projects.extend(community_projects);

        // Deduplicate and validate
        let unique_projects = self.deduplicate_projects(discovered_projects);
        
        Ok(unique_projects)
    }

    async fn search_github_projects(&self) -> ResearchResult<Option<Vec<MigrationProject>>> {
        let Some(client) = &self.github_client else {
            return Ok(None);
        };

        let mut projects = Vec::new();

        for keyword in &self.search_keywords {
            let search_results = client
                .search()
                .repositories(keyword)
                .sort("stars")
                .order("desc")
                .per_page(50)
                .send()
                .await?;

            for repo in search_results.items {
                if self.should_include_repository(&repo) {
                    let project = self.convert_github_repo_to_project(repo).await?;
                    projects.push(project);
                }
            }

            // Rate limiting - GitHub API allows 30 requests per minute for search
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }

        Ok(Some(projects))
    }

    fn should_include_repository(&self, repo: &octocrab::models::Repository) -> bool {
        let name_lower = repo.name.to_lowercase();
        let description_lower = repo.description.as_ref()
            .map(|d| d.to_lowercase())
            .unwrap_or_default();

        // Exclude based on patterns
        for pattern in &self.exclusion_patterns {
            if name_lower.contains(pattern) || description_lower.contains(pattern) {
                return false;
            }
        }

        // Include if it has reasonable activity
        repo.stargazers_count.unwrap_or(0) >= 10 &&
        repo.language.as_ref().map_or(false, |lang| {
            matches!(lang.as_str(), "Rust" | "TypeScript" | "JavaScript" | "C++" | "Go" | "C#")
        })
    }

    async fn convert_github_repo_to_project(&self, repo: octocrab::models::Repository) -> ResearchResult<MigrationProject> {
        let category = self.determine_project_category(&repo);
        let status = if repo.archived.unwrap_or(false) {
            ProjectStatus::Archived
        } else {
            ProjectStatus::Active
        };

        Ok(MigrationProject {
            id: None,
            name: repo.name,
            url: repo.html_url.to_string(),
            github_url: Some(repo.html_url.to_string()),
            category,
            status,
            last_updated: repo.updated_at.map(|dt| dt.date_naive()),
            stars: repo.stargazers_count.unwrap_or(0) as i32,
            contributors: 0, // Will be filled by detailed analysis
            technical_details: None,
            analysis: None,
        })
    }

    fn determine_project_category(&self, repo: &octocrab::models::Repository) -> ProjectCategory {
        let name_lower = repo.name.to_lowercase();
        let description_lower = repo.description.as_ref()
            .map(|d| d.to_lowercase())
            .unwrap_or_default();
        let combined = format!("{} {}", name_lower, description_lower);

        if combined.contains("vscode") || combined.contains("vs code") {
            ProjectCategory::VsCodeFork
        } else if combined.contains("rust") && (combined.contains("ide") || combined.contains("editor")) {
            ProjectCategory::RustIde
        } else if combined.contains("ai") || combined.contains("copilot") || combined.contains("gpt") {
            ProjectCategory::AiIde
        } else if combined.contains("wasm") || combined.contains("webassembly") {
            ProjectCategory::WasmApp
        } else {
            ProjectCategory::IdeMigration
        }
    }

    /// Curated list of known IDE migration projects for comprehensive analysis
    fn get_curated_migration_projects(&self) -> Vec<MigrationProject> {
        vec![
            // Tauri-based migrations
            MigrationProject {
                id: None,
                name: "Tauri".to_string(),
                url: "https://tauri.app/".to_string(),
                github_url: Some("https://github.com/tauri-apps/tauri".to_string()),
                category: ProjectCategory::IdeMigration,
                status: ProjectStatus::Active,
                last_updated: None,
                stars: 0, // Will be updated by detailed analysis
                contributors: 0,
                technical_details: None,
                analysis: None,
            },
            
            // Zed Editor - Rust-based
            MigrationProject {
                id: None,
                name: "Zed".to_string(),
                url: "https://zed.dev/".to_string(),
                github_url: Some("https://github.com/zed-industries/zed".to_string()),
                category: ProjectCategory::RustIde,
                status: ProjectStatus::Active,
                last_updated: None,
                stars: 0,
                contributors: 0,
                technical_details: None,
                analysis: None,
            },

            // Lapce - Rust-based IDE
            MigrationProject {
                id: None,
                name: "Lapce".to_string(),
                url: "https://lapce.dev/".to_string(),
                github_url: Some("https://github.com/lapce/lapce".to_string()),
                category: ProjectCategory::RustIde,
                status: ProjectStatus::Active,
                last_updated: None,
                stars: 0,
                contributors: 0,
                technical_details: None,
                analysis: None,
            },

            // Helix - Terminal-based Rust editor
            MigrationProject {
                id: None,
                name: "Helix".to_string(),
                url: "https://helix-editor.com/".to_string(),
                github_url: Some("https://github.com/helix-editor/helix".to_string()),
                category: ProjectCategory::RustIde,
                status: ProjectStatus::Active,
                last_updated: None,
                stars: 0,
                contributors: 0,
                technical_details: None,
                analysis: None,
            },

            // VSCodium - VS Code without Microsoft branding
            MigrationProject {
                id: None,
                name: "VSCodium".to_string(),
                url: "https://vscodium.com/".to_string(),
                github_url: Some("https://github.com/VSCodium/vscodium".to_string()),
                category: ProjectCategory::VsCodeFork,
                status: ProjectStatus::Active,
                last_updated: None,
                stars: 0,
                contributors: 0,
                technical_details: None,
                analysis: None,
            },

            // Code - OSS (Open Source VS Code)
            MigrationProject {
                id: None,
                name: "Code - OSS".to_string(),
                url: "https://github.com/microsoft/vscode".to_string(),
                github_url: Some("https://github.com/microsoft/vscode".to_string()),
                category: ProjectCategory::VsCodeFork,
                status: ProjectStatus::Active,
                last_updated: None,
                stars: 0,
                contributors: 0,
                technical_details: None,
                analysis: None,
            },

            // Theia - Eclipse Foundation IDE
            MigrationProject {
                id: None,
                name: "Eclipse Theia".to_string(),
                url: "https://theia-ide.org/".to_string(),
                github_url: Some("https://github.com/eclipse-theia/theia".to_string()),
                category: ProjectCategory::IdeMigration,
                status: ProjectStatus::Active,
                last_updated: None,
                stars: 0,
                contributors: 0,
                technical_details: None,
                analysis: None,
            },

            // Fleet - JetBrains next-gen IDE
            MigrationProject {
                id: None,
                name: "Fleet".to_string(),
                url: "https://www.jetbrains.com/fleet/".to_string(),
                github_url: None,
                category: ProjectCategory::IdeMigration,
                status: ProjectStatus::Active,
                last_updated: None,
                stars: 0,
                contributors: 0,
                technical_details: None,
                analysis: None,
            },

            // Cursor - AI-powered VS Code fork
            MigrationProject {
                id: None,
                name: "Cursor".to_string(),
                url: "https://cursor.sh/".to_string(),
                github_url: None,
                category: ProjectCategory::AiIde,
                status: ProjectStatus::Active,
                last_updated: None,
                stars: 0,
                contributors: 0,
                technical_details: None,
                analysis: None,
            },

            // Warp Terminal - Rust-based terminal with IDE features
            MigrationProject {
                id: None,
                name: "Warp".to_string(),
                url: "https://www.warp.dev/".to_string(),
                github_url: None,
                category: ProjectCategory::RustIde,
                status: ProjectStatus::Active,
                last_updated: None,
                stars: 0,
                contributors: 0,
                technical_details: None,
                analysis: None,
            },
        ]
    }

    /// Community-recommended projects from forums, discussions, and expert recommendations
    fn get_community_recommended_projects(&self) -> Vec<MigrationProject> {
        vec![
            // Xi Editor - Google's Rust-based editor (archived but influential)
            MigrationProject {
                id: None,
                name: "Xi Editor".to_string(),
                url: "https://xi-editor.io/".to_string(),
                github_url: Some("https://github.com/xi-editor/xi-editor".to_string()),
                category: ProjectCategory::RustIde,
                status: ProjectStatus::Archived,
                last_updated: None,
                stars: 0,
                contributors: 0,
                technical_details: None,
                analysis: None,
            },

            // Neovim - Vim fork with modern architecture
            MigrationProject {
                id: None,
                name: "Neovim".to_string(),
                url: "https://neovim.io/".to_string(),
                github_url: Some("https://github.com/neovim/neovim".to_string()),
                category: ProjectCategory::IdeMigration,
                status: ProjectStatus::Active,
                last_updated: None,
                stars: 0,
                contributors: 0,
                technical_details: None,
                analysis: None,
            },

            // Kakoune - Modal editor with different approach
            MigrationProject {
                id: None,
                name: "Kakoune".to_string(),
                url: "http://kakoune.org/".to_string(),
                github_url: Some("https://github.com/mawww/kakoune".to_string()),
                category: ProjectCategory::IdeMigration,
                status: ProjectStatus::Active,
                last_updated: None,
                stars: 0,
                contributors: 0,
                technical_details: None,
                analysis: None,
            },

            // Amp - Rust-based text editor
            MigrationProject {
                id: None,
                name: "Amp".to_string(),
                url: "https://amp.rs/".to_string(),
                github_url: Some("https://github.com/jmacdonald/amp".to_string()),
                category: ProjectCategory::RustIde,
                status: ProjectStatus::Active,
                last_updated: None,
                stars: 0,
                contributors: 0,
                technical_details: None,
                analysis: None,
            },
        ]
    }

    fn deduplicate_projects(&self, projects: Vec<MigrationProject>) -> Vec<MigrationProject> {
        let mut unique_projects = Vec::new();
        let mut seen_urls = std::collections::HashSet::new();

        for project in projects {
            let key = project.github_url.as_ref().unwrap_or(&project.url).clone();
            if !seen_urls.contains(&key) {
                seen_urls.insert(key);
                unique_projects.push(project);
            }
        }

        unique_projects
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_discovery_initialization() {
        let discovery = ProjectDiscoveryEngine::new();
        assert!(!discovery.search_keywords.is_empty());
        assert!(!discovery.exclusion_patterns.is_empty());
    }

    #[test]
    fn test_curated_projects_list() {
        let discovery = ProjectDiscoveryEngine::new();
        let projects = discovery.get_curated_migration_projects();
        
        assert!(!projects.is_empty());
        assert!(projects.len() >= 10); // Should have at least 10 curated projects
        
        // Verify we have different categories
        let categories: std::collections::HashSet<_> = projects.iter()
            .map(|p| &p.category)
            .collect();
        assert!(categories.len() > 1);
    }

    #[test]
    fn test_deduplication() {
        let discovery = ProjectDiscoveryEngine::new();
        let mut projects = vec![
            MigrationProject {
                id: None,
                name: "Test Project".to_string(),
                url: "https://example.com".to_string(),
                github_url: Some("https://github.com/test/project".to_string()),
                category: ProjectCategory::IdeMigration,
                status: ProjectStatus::Active,
                last_updated: None,
                stars: 0,
                contributors: 0,
                technical_details: None,
                analysis: None,
            },
            MigrationProject {
                id: None,
                name: "Test Project Duplicate".to_string(),
                url: "https://example2.com".to_string(),
                github_url: Some("https://github.com/test/project".to_string()), // Same GitHub URL
                category: ProjectCategory::IdeMigration,
                status: ProjectStatus::Active,
                last_updated: None,
                stars: 0,
                contributors: 0,
                technical_details: None,
                analysis: None,
            },
        ];

        let unique = discovery.deduplicate_projects(projects);
        assert_eq!(unique.len(), 1);
    }
}