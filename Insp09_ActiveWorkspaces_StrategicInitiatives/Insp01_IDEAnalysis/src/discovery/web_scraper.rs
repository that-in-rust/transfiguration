use crate::models::{Project, ProjectCategory, ResearchSource};
use crate::errors::{Result, ResearchError};
use scraper::{Html, Selector};
use serde_json::json;

pub struct WebScraper {
    client: reqwest::Client,
}

impl WebScraper {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent("Research Bot 1.0")
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap(),
        }
    }

    /// Discover projects through web scraping
    pub async fn discover_projects(&self, keywords: &[String], category: &ProjectCategory) -> Result<Vec<Project>> {
        let mut projects = Vec::new();

        // Search different platforms
        for keyword in keywords {
            // Search awesome lists
            let awesome_projects = self.search_awesome_lists(keyword, category).await?;
            projects.extend(awesome_projects);

            // Search documentation sites
            let doc_projects = self.search_documentation_sites(keyword, category).await?;
            projects.extend(doc_projects);

            // Rate limiting
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }

        Ok(projects)
    }

    /// Search awesome lists for projects
    async fn search_awesome_lists(&self, keyword: &str, category: &ProjectCategory) -> Result<Vec<Project>> {
        let mut projects = Vec::new();

        let awesome_urls = match category {
            ProjectCategory::RustIde => vec![
                "https://raw.githubusercontent.com/rust-unofficial/awesome-rust/master/README.md",
            ],
            ProjectCategory::WasmApp => vec![
                "https://raw.githubusercontent.com/mbasso/awesome-wasm/master/README.md",
            ],
            ProjectCategory::AiIde => vec![
                "https://raw.githubusercontent.com/steven2358/awesome-generative-ai/master/README.md",
            ],
            _ => vec![],
        };

        for url in awesome_urls {
            if let Ok(content) = self.fetch_content(url).await {
                let extracted_projects = self.extract_projects_from_markdown(&content, keyword, category.clone()).await?;
                projects.extend(extracted_projects);
            }
        }

        Ok(projects)
    }

    /// Search documentation and blog sites
    async fn search_documentation_sites(&self, _keyword: &str, _category: &ProjectCategory) -> Result<Vec<Project>> {
        // This would implement searches on sites like:
        // - Rust blog posts
        // - Developer documentation sites
        // - Technical blogs
        // For now, return empty to avoid rate limiting issues
        Ok(Vec::new())
    }

    /// Fetch content from URL
    async fn fetch_content(&self, url: &str) -> Result<String> {
        let response = self.client.get(url).send().await?;
        
        if !response.status().is_success() {
            return Err(ResearchError::Http(reqwest::Error::from(
                reqwest::ErrorKind::Request
            )));
        }

        let content = response.text().await?;
        Ok(content)
    }

    /// Extract projects from markdown content
    async fn extract_projects_from_markdown(&self, content: &str, keyword: &str, category: ProjectCategory) -> Result<Vec<Project>> {
        let mut projects = Vec::new();
        let keyword_lower = keyword.to_lowercase();

        // Simple regex to find GitHub links in markdown
        let github_regex = regex::Regex::new(r"\[([^\]]+)\]\((https://github\.com/[^)]+)\)").unwrap();
        
        for line in content.lines() {
            let line_lower = line.to_lowercase();
            
            // Check if line contains our keyword
            if line_lower.contains(&keyword_lower) {
                // Extract GitHub links from this line
                for captures in github_regex.captures_iter(line) {
                    let name = captures.get(1).unwrap().as_str();
                    let url = captures.get(2).unwrap().as_str();
                    
                    let mut project = Project::new(name.to_string(), url.to_string(), category.clone());
                    
                    // Try to extract description from the line
                    if let Some(desc_start) = line.find(" - ") {
                        let description = line[desc_start + 3..].trim();
                        if !description.is_empty() {
                            project.description = Some(description.to_string());
                        }
                    }
                    
                    project.metadata = json!({
                        "source": "awesome_list",
                        "discovery_method": "web_scraping",
                        "keyword_match": keyword
                    });
                    
                    projects.push(project);
                }
            }
        }

        Ok(projects)
    }

    /// Extract projects from HTML content
    async fn extract_projects_from_html(&self, content: &str, _keyword: &str, category: ProjectCategory) -> Result<Vec<Project>> {
        let document = Html::parse_document(content);
        let mut projects = Vec::new();

        // Look for GitHub links
        let link_selector = Selector::parse("a[href*='github.com']").unwrap();
        
        for element in document.select(&link_selector) {
            if let Some(href) = element.value().attr("href") {
                if href.starts_with("https://github.com/") {
                    let name = element.text().collect::<Vec<_>>().join(" ");
                    
                    if !name.trim().is_empty() {
                        let mut project = Project::new(name.trim().to_string(), href.to_string(), category.clone());
                        
                        project.metadata = json!({
                            "source": "web_scraping",
                            "discovery_method": "html_parsing"
                        });
                        
                        projects.push(project);
                    }
                }
            }
        }

        Ok(projects)
    }

    /// Create research source from web content
    pub async fn create_research_source(&self, url: &str, source_type: &str, project_id: Option<uuid::Uuid>) -> Result<ResearchSource> {
        let mut source = ResearchSource::new(url.to_string(), source_type.to_string());
        source.project_id = project_id;
        
        // Try to fetch and analyze the content
        if let Ok(content) = self.fetch_content(url).await {
            // Extract title from HTML if possible
            if let Ok(title) = self.extract_title_from_html(&content).await {
                source.title = Some(title);
            }
            
            // Calculate content hash for verification
            source.content_hash = Some(self.calculate_content_hash(&content));
            
            // Set reliability score based on domain
            source.reliability_score = Some(self.assess_domain_reliability(url));
            
            source.verification_status = "verified".to_string();
        } else {
            source.verification_status = "failed".to_string();
            source.reliability_score = Some(0.0);
        }

        Ok(source)
    }

    /// Extract title from HTML content
    async fn extract_title_from_html(&self, content: &str) -> Result<String> {
        let document = Html::parse_document(content);
        
        // Try to find title tag
        if let Ok(title_selector) = Selector::parse("title") {
            if let Some(title_element) = document.select(&title_selector).next() {
                let title = title_element.text().collect::<Vec<_>>().join(" ");
                if !title.trim().is_empty() {
                    return Ok(title.trim().to_string());
                }
            }
        }

        // Try to find h1 tag
        if let Ok(h1_selector) = Selector::parse("h1") {
            if let Some(h1_element) = document.select(&h1_selector).next() {
                let title = h1_element.text().collect::<Vec<_>>().join(" ");
                if !title.trim().is_empty() {
                    return Ok(title.trim().to_string());
                }
            }
        }

        Err(ResearchError::SourceVerification {
            url: "content".to_string(),
            reason: "No title found".to_string(),
        })
    }

    /// Calculate content hash for verification
    fn calculate_content_hash(&self, content: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    /// Assess domain reliability
    fn assess_domain_reliability(&self, url: &str) -> f32 {
        if let Ok(parsed_url) = url::Url::parse(url) {
            if let Some(domain) = parsed_url.domain() {
                match domain {
                    "github.com" => 0.9,
                    "gitlab.com" => 0.85,
                    "bitbucket.org" => 0.8,
                    "docs.rs" => 0.9,
                    "crates.io" => 0.9,
                    "rust-lang.org" => 0.95,
                    "mozilla.org" => 0.9,
                    domain if domain.ends_with(".edu") => 0.85,
                    domain if domain.ends_with(".org") => 0.75,
                    domain if domain.ends_with(".gov") => 0.9,
                    _ => 0.6, // Default reliability for unknown domains
                }
            } else {
                0.3
            }
        } else {
            0.1
        }
    }
}