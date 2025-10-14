use crate::models::{Project, ProjectCategory, ResearchSource};
use crate::errors::{Result, ResearchError};
use octocrab::Octocrab;
use serde_json::json;
use chrono::{DateTime, Utc};

pub struct GitHubDiscovery {
    client: Octocrab,
}

impl GitHubDiscovery {
    pub fn new(token: Option<String>) -> Self {
        let client = match token {
            Some(token) => Octocrab::builder().personal_token(token).build().unwrap(),
            None => Octocrab::builder().build().unwrap(),
        };

        Self { client }
    }

    /// Search GitHub repositories based on keywords and category
    pub async fn search_repositories(&self, keywords: &[String], category: &ProjectCategory) -> Result<Vec<Project>> {
        let mut projects = Vec::new();

        for keyword in keywords {
            let query = self.build_search_query(keyword, category);
            
            let search_result = self.client
                .search()
                .repositories(&query)
                .sort("stars")
                .order("desc")
                .per_page(50)
                .send()
                .await?;

            for repo in search_result.items {
                let project = self.convert_repo_to_project(repo, category.clone()).await?;
                projects.push(project);
            }

            // Rate limiting - GitHub allows 30 requests per minute for search
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }

        // Deduplicate by URL
        projects.sort_by(|a, b| a.url.cmp(&b.url));
        projects.dedup_by(|a, b| a.url == b.url);

        Ok(projects)
    }

    /// Build search query based on category
    fn build_search_query(&self, keyword: &str, category: &ProjectCategory) -> String {
        let base_query = keyword.to_string();
        
        match category {
            ProjectCategory::IdeMigration => {
                format!("{} (electron OR native OR migration OR performance) language:rust OR language:cpp OR language:go", base_query)
            }
            ProjectCategory::VscodeFork => {
                format!("{} (vscode OR \"visual studio code\" OR code-oss) fork", base_query)
            }
            ProjectCategory::RustIde => {
                format!("{} language:rust (ide OR editor OR development)", base_query)
            }
            ProjectCategory::AiIde => {
                format!("{} (ai OR copilot OR gpt OR llm) (ide OR editor)", base_query)
            }
            ProjectCategory::WasmApp => {
                format!("{} (wasm OR webassembly) (ide OR editor OR tools)", base_query)
            }
            ProjectCategory::TechnicalPattern => {
                format!("{} (architecture OR pattern OR design)", base_query)
            }
        }
    }

    /// Convert GitHub repository to Project model
    async fn convert_repo_to_project(&self, repo: octocrab::models::Repository, category: ProjectCategory) -> Result<Project> {
        let mut project = Project::new(
            repo.name.clone(),
            repo.html_url.to_string(),
            category,
        );

        project.description = repo.description;
        project.stars = Some(repo.stargazers_count.unwrap_or(0) as i32);
        project.last_updated = repo.updated_at.map(|dt| dt.with_timezone(&Utc));

        // Fetch additional metadata
        let metadata = self.fetch_repository_metadata(&repo).await?;
        project.metadata = metadata;

        Ok(project)
    }

    /// Fetch detailed repository metadata
    async fn fetch_repository_metadata(&self, repo: &octocrab::models::Repository) -> Result<serde_json::Value> {
        let mut metadata = json!({
            "github_id": repo.id,
            "full_name": repo.full_name,
            "owner": repo.owner.as_ref().map(|o| &o.login),
            "language": repo.language,
            "forks_count": repo.forks_count,
            "open_issues_count": repo.open_issues_count,
            "default_branch": repo.default_branch,
            "archived": repo.archived,
            "disabled": repo.disabled,
            "private": repo.private,
            "fork": repo.fork,
            "has_issues": repo.has_issues,
            "has_projects": repo.has_projects,
            "has_wiki": repo.has_wiki,
            "has_pages": repo.has_pages,
            "has_downloads": repo.has_downloads,
            "license": repo.license.as_ref().map(|l| &l.name),
            "topics": repo.topics,
        });

        // Fetch contributors count
        if let Ok(contributors) = self.fetch_contributors_count(&repo.owner.as_ref().unwrap().login, &repo.name).await {
            metadata["contributors_count"] = json!(contributors);
        }

        // Fetch recent activity
        if let Ok(activity) = self.fetch_recent_activity(&repo.owner.as_ref().unwrap().login, &repo.name).await {
            metadata["recent_activity"] = activity;
        }

        // Fetch README content for analysis
        if let Ok(readme) = self.fetch_readme_content(&repo.owner.as_ref().unwrap().login, &repo.name).await {
            metadata["readme_content"] = json!(readme);
        }

        Ok(metadata)
    }

    /// Fetch contributors count
    async fn fetch_contributors_count(&self, owner: &str, repo: &str) -> Result<i32> {
        let contributors = self.client
            .repos(owner, repo)
            .list_contributors()
            .per_page(1)
            .send()
            .await?;

        // GitHub doesn't provide total count directly, so we estimate from the first page
        Ok(contributors.items.len() as i32)
    }

    /// Fetch recent activity metrics
    async fn fetch_recent_activity(&self, owner: &str, repo: &str) -> Result<serde_json::Value> {
        let mut activity = json!({});

        // Fetch recent commits (last 30 days)
        let since = Utc::now() - chrono::Duration::days(30);
        if let Ok(commits) = self.client
            .repos(owner, repo)
            .list_commits()
            .since(since)
            .per_page(100)
            .send()
            .await
        {
            activity["commits_last_30_days"] = json!(commits.items.len());
        }

        // Fetch recent releases
        if let Ok(releases) = self.client
            .repos(owner, repo)
            .releases()
            .list()
            .per_page(10)
            .send()
            .await
        {
            activity["recent_releases"] = json!(releases.items.len());
            if let Some(latest) = releases.items.first() {
                activity["latest_release"] = json!({
                    "tag_name": latest.tag_name,
                    "published_at": latest.published_at,
                    "prerelease": latest.prerelease,
                });
            }
        }

        Ok(activity)
    }

    /// Fetch README content for analysis
    async fn fetch_readme_content(&self, owner: &str, repo: &str) -> Result<String> {
        let readme_files = vec!["README.md", "README.rst", "README.txt", "README"];
        
        for readme_file in readme_files {
            if let Ok(content) = self.client
                .repos(owner, repo)
                .get_content()
                .path(readme_file)
                .send()
                .await
            {
                if let Some(file) = content.items.first() {
                    if let Some(content_str) = &file.content {
                        // Decode base64 content
                        if let Ok(decoded) = base64::decode(content_str.replace('\n', "")) {
                            if let Ok(text) = String::from_utf8(decoded) {
                                return Ok(text);
                            }
                        }
                    }
                }
            }
        }

        Err(ResearchError::SourceVerification {
            url: format!("https://github.com/{}/{}", owner, repo),
            reason: "No README file found".to_string(),
        })
    }

    /// Create research source from GitHub repository
    pub async fn create_research_source(&self, repo_url: &str, project_id: uuid::Uuid) -> Result<ResearchSource> {
        let mut source = ResearchSource::new(repo_url.to_string(), "github".to_string());
        source.project_id = Some(project_id);
        source.verification_status = "verified".to_string();
        source.reliability_score = Some(0.9); // GitHub repos are generally reliable

        // Extract owner/repo from URL
        if let Some(captures) = regex::Regex::new(r"github\.com/([^/]+)/([^/]+)")
            .unwrap()
            .captures(repo_url)
        {
            let owner = &captures[1];
            let repo_name = &captures[2];

            if let Ok(repo) = self.client.repos(owner, repo_name).get().await {
                source.title = Some(format!("{}/{}", owner, repo_name));
                source.author = repo.owner.map(|o| o.login);
                source.published_date = repo.created_at.map(|dt| dt.with_timezone(&Utc));
            }
        }

        Ok(source)
    }

    /// Analyze repository for technical patterns
    pub async fn analyze_repository_patterns(&self, owner: &str, repo: &str) -> Result<Vec<String>> {
        let mut patterns = Vec::new();

        // Analyze languages
        if let Ok(languages) = self.client.repos(owner, repo).list_languages().await {
            for (language, _) in languages {
                patterns.push(format!("language:{}", language.to_lowercase()));
            }
        }

        // Analyze repository structure through contents API
        if let Ok(contents) = self.client
            .repos(owner, repo)
            .get_content()
            .send()
            .await
        {
            for item in contents.items {
                match item.name.as_str() {
                    "Cargo.toml" => patterns.push("rust_project".to_string()),
                    "package.json" => patterns.push("node_project".to_string()),
                    "Dockerfile" => patterns.push("containerized".to_string()),
                    "docker-compose.yml" => patterns.push("docker_compose".to_string()),
                    ".github" => patterns.push("github_actions".to_string()),
                    "src" => patterns.push("source_directory".to_string()),
                    "tests" => patterns.push("test_directory".to_string()),
                    "docs" => patterns.push("documentation".to_string()),
                    _ => {}
                }
            }
        }

        Ok(patterns)
    }
}