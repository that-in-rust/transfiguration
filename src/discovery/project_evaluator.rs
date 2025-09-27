use crate::models::{Project, ProjectCategory, TechnicalAnalysis, ConfidenceLevel};
use crate::errors::{Result, ResearchError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationCriteria {
    pub technical_quality: TechnicalQualityCriteria,
    pub adoption_metrics: AdoptionMetricsCriteria,
    pub sustainability: SustainabilityCriteria,
    pub relevance_to_kiro: RelevanceCriteria,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicalQualityCriteria {
    pub min_stars: i32,
    pub min_contributors: i32,
    pub required_languages: Vec<String>,
    pub architecture_patterns: Vec<String>,
    pub code_quality_indicators: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdoptionMetricsCriteria {
    pub min_forks: i32,
    pub min_downloads: Option<i32>,
    pub community_activity: CommunityActivityCriteria,
    pub documentation_quality: DocumentationCriteria,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityActivityCriteria {
    pub min_recent_commits: i32,
    pub max_days_since_update: i32,
    pub min_open_issues_ratio: f32,
    pub min_contributor_diversity: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationCriteria {
    pub has_readme: bool,
    pub has_api_docs: bool,
    pub has_examples: bool,
    pub min_readme_length: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SustainabilityCriteria {
    pub license_requirements: Vec<String>,
    pub maintenance_indicators: MaintenanceIndicators,
    pub funding_indicators: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceIndicators {
    pub regular_releases: bool,
    pub responsive_issues: bool,
    pub active_maintainers: bool,
    pub security_updates: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelevanceCriteria {
    pub technology_alignment: Vec<String>,
    pub use_case_similarity: Vec<String>,
    pub architectural_relevance: Vec<String>,
    pub migration_patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationResult {
    pub overall_score: f32,
    pub technical_score: f32,
    pub adoption_score: f32,
    pub sustainability_score: f32,
    pub relevance_score: f32,
    pub confidence_level: ConfidenceLevel,
    pub strengths: Vec<String>,
    pub weaknesses: Vec<String>,
    pub recommendations: Vec<String>,
    pub bias_indicators: Vec<BiasIndicator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiasIndicator {
    pub bias_type: String,
    pub description: String,
    pub severity: String, // "low", "medium", "high"
    pub mitigation: String,
}

pub struct ProjectEvaluator {
    criteria: HashMap<ProjectCategory, EvaluationCriteria>,
}

impl ProjectEvaluator {
    pub fn new() -> Self {
        let mut criteria = HashMap::new();
        
        // Define criteria for each project category
        criteria.insert(ProjectCategory::IdeMigration, Self::ide_migration_criteria());
        criteria.insert(ProjectCategory::VscodeFork, Self::vscode_fork_criteria());
        criteria.insert(ProjectCategory::RustIde, Self::rust_ide_criteria());
        criteria.insert(ProjectCategory::AiIde, Self::ai_ide_criteria());
        criteria.insert(ProjectCategory::WasmApp, Self::wasm_app_criteria());
        criteria.insert(ProjectCategory::TechnicalPattern, Self::technical_pattern_criteria());

        Self { criteria }
    }

    /// Check if project meets minimum criteria for inclusion
    pub async fn meets_criteria(&self, project: &Project, category: &ProjectCategory) -> Result<bool> {
        let criteria = self.criteria.get(category).ok_or_else(|| {
            ResearchError::Validation {
                field: "category".to_string(),
                message: format!("No criteria defined for category: {:?}", category),
            }
        })?;

        // Basic quality gates
        let stars = project.stars.unwrap_or(0);
        let has_description = project.description.is_some() && !project.description.as_ref().unwrap().is_empty();

        // Minimum thresholds
        if stars < criteria.technical_quality.min_stars {
            return Ok(false);
        }

        if !has_description {
            return Ok(false);
        }

        // Check for archived or deprecated projects
        if let Some(metadata) = project.metadata.as_object() {
            if metadata.get("archived").and_then(|v| v.as_bool()).unwrap_or(false) {
                return Ok(false);
            }
            if metadata.get("disabled").and_then(|v| v.as_bool()).unwrap_or(false) {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Perform comprehensive evaluation of a project
    pub async fn evaluate_project(&self, project: &Project) -> Result<EvaluationResult> {
        let criteria = self.criteria.get(&project.category).ok_or_else(|| {
            ResearchError::Validation {
                field: "category".to_string(),
                message: format!("No criteria defined for category: {:?}", project.category),
            }
        })?;

        let technical_score = self.evaluate_technical_quality(project, &criteria.technical_quality).await?;
        let adoption_score = self.evaluate_adoption_metrics(project, &criteria.adoption_metrics).await?;
        let sustainability_score = self.evaluate_sustainability(project, &criteria.sustainability).await?;
        let relevance_score = self.evaluate_relevance_to_kiro(project, &criteria.relevance_to_kiro).await?;

        // Calculate weighted overall score
        let overall_score = (technical_score * 0.3) + (adoption_score * 0.25) + (sustainability_score * 0.25) + (relevance_score * 0.2);

        // Determine confidence level
        let confidence_level = self.calculate_confidence_level(project, overall_score).await?;

        // Identify strengths and weaknesses
        let (strengths, weaknesses) = self.identify_strengths_weaknesses(project, &[technical_score, adoption_score, sustainability_score, relevance_score]).await?;

        // Generate recommendations
        let recommendations = self.generate_recommendations(project, &strengths, &weaknesses).await?;

        // Detect potential biases
        let bias_indicators = self.detect_bias_indicators(project).await?;

        Ok(EvaluationResult {
            overall_score,
            technical_score,
            adoption_score,
            sustainability_score,
            relevance_score,
            confidence_level,
            strengths,
            weaknesses,
            recommendations,
            bias_indicators,
        })
    }

    /// Evaluate technical quality aspects
    async fn evaluate_technical_quality(&self, project: &Project, criteria: &TechnicalQualityCriteria) -> Result<f32> {
        let mut score = 0.0;
        let mut max_score = 0.0;

        // Stars score (0-25 points)
        let stars = project.stars.unwrap_or(0) as f32;
        let stars_score = (stars / criteria.min_stars as f32).min(2.0) * 12.5;
        score += stars_score;
        max_score += 25.0;

        // Language alignment (0-20 points)
        if let Some(metadata) = project.metadata.as_object() {
            if let Some(language) = metadata.get("language").and_then(|v| v.as_str()) {
                if criteria.required_languages.contains(&language.to_lowercase()) {
                    score += 20.0;
                }
            }
        }
        max_score += 20.0;

        // Code quality indicators (0-25 points)
        let quality_score = self.assess_code_quality_indicators(project, &criteria.code_quality_indicators).await?;
        score += quality_score;
        max_score += 25.0;

        // Architecture patterns (0-30 points)
        let architecture_score = self.assess_architecture_patterns(project, &criteria.architecture_patterns).await?;
        score += architecture_score;
        max_score += 30.0;

        Ok((score / max_score) * 100.0)
    }

    /// Evaluate adoption metrics
    async fn evaluate_adoption_metrics(&self, project: &Project, criteria: &AdoptionMetricsCriteria) -> Result<f32> {
        let mut score = 0.0;
        let mut max_score = 0.0;

        // Forks score (0-20 points)
        if let Some(metadata) = project.metadata.as_object() {
            if let Some(forks) = metadata.get("forks_count").and_then(|v| v.as_i64()) {
                let forks_score = ((forks as f32) / (criteria.min_forks as f32)).min(2.0) * 10.0;
                score += forks_score;
            }
        }
        max_score += 20.0;

        // Community activity (0-40 points)
        let activity_score = self.assess_community_activity(project, &criteria.community_activity).await?;
        score += activity_score;
        max_score += 40.0;

        // Documentation quality (0-40 points)
        let docs_score = self.assess_documentation_quality(project, &criteria.documentation_quality).await?;
        score += docs_score;
        max_score += 40.0;

        Ok((score / max_score) * 100.0)
    }

    /// Evaluate sustainability factors
    async fn evaluate_sustainability(&self, project: &Project, criteria: &SustainabilityCriteria) -> Result<f32> {
        let mut score = 0.0;
        let mut max_score = 0.0;

        // License compatibility (0-30 points)
        if let Some(metadata) = project.metadata.as_object() {
            if let Some(license) = metadata.get("license").and_then(|v| v.as_str()) {
                if criteria.license_requirements.contains(&license.to_lowercase()) {
                    score += 30.0;
                }
            }
        }
        max_score += 30.0;

        // Maintenance indicators (0-50 points)
        let maintenance_score = self.assess_maintenance_indicators(project, &criteria.maintenance_indicators).await?;
        score += maintenance_score;
        max_score += 50.0;

        // Funding indicators (0-20 points)
        let funding_score = self.assess_funding_indicators(project, &criteria.funding_indicators).await?;
        score += funding_score;
        max_score += 20.0;

        Ok((score / max_score) * 100.0)
    }

    /// Evaluate relevance to Kiro project
    async fn evaluate_relevance_to_kiro(&self, project: &Project, criteria: &RelevanceCriteria) -> Result<f32> {
        let mut score = 0.0;
        let mut max_score = 0.0;

        // Technology alignment (0-30 points)
        let tech_score = self.assess_technology_alignment(project, &criteria.technology_alignment).await?;
        score += tech_score;
        max_score += 30.0;

        // Use case similarity (0-25 points)
        let usecase_score = self.assess_use_case_similarity(project, &criteria.use_case_similarity).await?;
        score += usecase_score;
        max_score += 25.0;

        // Architectural relevance (0-25 points)
        let arch_score = self.assess_architectural_relevance(project, &criteria.architectural_relevance).await?;
        score += arch_score;
        max_score += 25.0;

        // Migration patterns (0-20 points)
        let migration_score = self.assess_migration_patterns(project, &criteria.migration_patterns).await?;
        score += migration_score;
        max_score += 20.0;

        Ok((score / max_score) * 100.0)
    }

    /// Calculate confidence level based on data quality and completeness
    async fn calculate_confidence_level(&self, project: &Project, overall_score: f32) -> Result<ConfidenceLevel> {
        let mut confidence_factors = 0;
        let mut total_factors = 0;

        // Data completeness factors
        total_factors += 1;
        if project.description.is_some() && !project.description.as_ref().unwrap().is_empty() {
            confidence_factors += 1;
        }

        total_factors += 1;
        if project.stars.is_some() && project.stars.unwrap() > 0 {
            confidence_factors += 1;
        }

        total_factors += 1;
        if let Some(metadata) = project.metadata.as_object() {
            if metadata.contains_key("language") && metadata.contains_key("forks_count") {
                confidence_factors += 1;
            }
        }

        // Source reliability
        total_factors += 1;
        if project.url.contains("github.com") {
            confidence_factors += 1; // GitHub is a reliable source
        }

        let confidence_ratio = confidence_factors as f32 / total_factors as f32;

        match (confidence_ratio, overall_score) {
            (r, s) if r >= 0.8 && s >= 70.0 => Ok(ConfidenceLevel::High),
            (r, s) if r >= 0.6 && s >= 50.0 => Ok(ConfidenceLevel::Medium),
            _ => Ok(ConfidenceLevel::Low),
        }
    }

    /// Identify project strengths and weaknesses
    async fn identify_strengths_weaknesses(&self, project: &Project, scores: &[f32]) -> Result<(Vec<String>, Vec<String>)> {
        let mut strengths = Vec::new();
        let mut weaknesses = Vec::new();

        let [technical, adoption, sustainability, relevance] = scores else {
            return Err(ResearchError::Validation {
                field: "scores".to_string(),
                message: "Expected 4 scores".to_string(),
            });
        };

        // Analyze strengths
        if *technical >= 75.0 {
            strengths.push("High technical quality with strong code practices".to_string());
        }
        if *adoption >= 75.0 {
            strengths.push("Strong community adoption and active user base".to_string());
        }
        if *sustainability >= 75.0 {
            strengths.push("Excellent sustainability indicators and maintenance".to_string());
        }
        if *relevance >= 75.0 {
            strengths.push("Highly relevant to Kiro's architecture and goals".to_string());
        }

        if let Some(stars) = project.stars {
            if stars > 1000 {
                strengths.push(format!("Popular project with {} stars indicating community trust", stars));
            }
        }

        // Analyze weaknesses
        if *technical < 50.0 {
            weaknesses.push("Technical quality concerns requiring further investigation".to_string());
        }
        if *adoption < 50.0 {
            weaknesses.push("Limited community adoption or engagement".to_string());
        }
        if *sustainability < 50.0 {
            weaknesses.push("Sustainability concerns regarding long-term maintenance".to_string());
        }
        if *relevance < 50.0 {
            weaknesses.push("Limited relevance to Kiro's specific requirements".to_string());
        }

        Ok((strengths, weaknesses))
    }

    /// Generate actionable recommendations
    async fn generate_recommendations(&self, project: &Project, strengths: &[String], weaknesses: &[String]) -> Result<Vec<String>> {
        let mut recommendations = Vec::new();

        if strengths.len() > weaknesses.len() {
            recommendations.push("High-priority project for detailed technical analysis".to_string());
            recommendations.push("Consider for architectural pattern extraction".to_string());
        }

        if weaknesses.iter().any(|w| w.contains("technical quality")) {
            recommendations.push("Conduct deeper code review before pattern extraction".to_string());
        }

        if weaknesses.iter().any(|w| w.contains("sustainability")) {
            recommendations.push("Verify current maintenance status and community health".to_string());
        }

        if strengths.iter().any(|s| s.contains("relevant")) {
            recommendations.push("Prioritize for migration pattern analysis".to_string());
        }

        // Category-specific recommendations
        match project.category {
            ProjectCategory::IdeMigration => {
                recommendations.push("Analyze migration timeline and resource requirements".to_string());
                recommendations.push("Document performance improvements and challenges".to_string());
            }
            ProjectCategory::VscodeFork => {
                recommendations.push("Study extension ecosystem compatibility approach".to_string());
                recommendations.push("Analyze customization and branding strategies".to_string());
            }
            ProjectCategory::RustIde => {
                recommendations.push("Examine Rust-specific tooling integration".to_string());
                recommendations.push("Study native performance characteristics".to_string());
            }
            ProjectCategory::AiIde => {
                recommendations.push("Analyze AI integration patterns and user experience".to_string());
                recommendations.push("Study performance impact of AI features".to_string());
            }
            ProjectCategory::WasmApp => {
                recommendations.push("Benchmark WASM performance characteristics".to_string());
                recommendations.push("Study JavaScript interop patterns".to_string());
            }
            ProjectCategory::TechnicalPattern => {
                recommendations.push("Extract and document reusable patterns".to_string());
            }
        }

        Ok(recommendations)
    }

    /// Detect potential research biases
    async fn detect_bias_indicators(&self, project: &Project) -> Result<Vec<BiasIndicator>> {
        let mut bias_indicators = Vec::new();

        // Selection bias - GitHub-only sources
        if project.url.contains("github.com") {
            bias_indicators.push(BiasIndicator {
                bias_type: "selection_bias".to_string(),
                description: "Research limited to GitHub repositories may miss other platforms".to_string(),
                severity: "medium".to_string(),
                mitigation: "Include projects from GitLab, Bitbucket, and other platforms".to_string(),
            });
        }

        // Popularity bias - high star count
        if let Some(stars) = project.stars {
            if stars > 10000 {
                bias_indicators.push(BiasIndicator {
                    bias_type: "popularity_bias".to_string(),
                    description: "High star count may overshadow technical merit".to_string(),
                    severity: "low".to_string(),
                    mitigation: "Balance popularity metrics with technical quality assessment".to_string(),
                });
            }
        }

        // Recency bias - very new projects
        if let Some(metadata) = project.metadata.as_object() {
            if let Some(created_at) = metadata.get("created_at") {
                // Check if project is very new (less than 6 months)
                bias_indicators.push(BiasIndicator {
                    bias_type: "recency_bias".to_string(),
                    description: "New projects may lack maturity indicators".to_string(),
                    severity: "medium".to_string(),
                    mitigation: "Consider project maturity in evaluation criteria".to_string(),
                });
            }
        }

        // Language bias
        if let Some(metadata) = project.metadata.as_object() {
            if let Some(language) = metadata.get("language").and_then(|v| v.as_str()) {
                if language == "Rust" {
                    bias_indicators.push(BiasIndicator {
                        bias_type: "technology_bias".to_string(),
                        description: "Preference for Rust projects may overlook other valid approaches".to_string(),
                        severity: "low".to_string(),
                        mitigation: "Include diverse technology stacks in analysis".to_string(),
                    });
                }
            }
        }

        Ok(bias_indicators)
    }

    // Helper methods for specific assessments
    async fn assess_code_quality_indicators(&self, project: &Project, _indicators: &[String]) -> Result<f32> {
        let mut score = 0.0;

        if let Some(metadata) = project.metadata.as_object() {
            // Has tests
            if metadata.get("has_tests").and_then(|v| v.as_bool()).unwrap_or(false) {
                score += 8.0;
            }

            // Has CI/CD
            if metadata.get("has_github_actions").and_then(|v| v.as_bool()).unwrap_or(false) {
                score += 7.0;
            }

            // Good issue management
            if let Some(issues) = metadata.get("open_issues_count").and_then(|v| v.as_i64()) {
                if issues < 50 {
                    score += 5.0;
                }
            }

            // Regular commits
            if let Some(activity) = metadata.get("recent_activity").and_then(|v| v.as_object()) {
                if let Some(commits) = activity.get("commits_last_30_days").and_then(|v| v.as_i64()) {
                    if commits > 10 {
                        score += 5.0;
                    }
                }
            }
        }

        Ok(score)
    }

    async fn assess_architecture_patterns(&self, project: &Project, _patterns: &[String]) -> Result<f32> {
        let mut score = 0.0;

        if let Some(metadata) = project.metadata.as_object() {
            // Modular structure
            if metadata.contains_key("src") {
                score += 10.0;
            }

            // Documentation
            if metadata.contains_key("docs") {
                score += 10.0;
            }

            // Configuration management
            if metadata.contains_key("Cargo.toml") || metadata.contains_key("package.json") {
                score += 10.0;
            }
        }

        Ok(score)
    }

    async fn assess_community_activity(&self, project: &Project, criteria: &CommunityActivityCriteria) -> Result<f32> {
        let mut score = 0.0;

        if let Some(metadata) = project.metadata.as_object() {
            if let Some(activity) = metadata.get("recent_activity").and_then(|v| v.as_object()) {
                // Recent commits
                if let Some(commits) = activity.get("commits_last_30_days").and_then(|v| v.as_i64()) {
                    if commits >= criteria.min_recent_commits as i64 {
                        score += 20.0;
                    }
                }

                // Recent releases
                if activity.contains_key("latest_release") {
                    score += 10.0;
                }
            }

            // Contributors
            if let Some(contributors) = metadata.get("contributors_count").and_then(|v| v.as_i64()) {
                if contributors >= criteria.min_contributor_diversity as i64 {
                    score += 10.0;
                }
            }
        }

        Ok(score)
    }

    async fn assess_documentation_quality(&self, project: &Project, criteria: &DocumentationCriteria) -> Result<f32> {
        let mut score = 0.0;

        if let Some(metadata) = project.metadata.as_object() {
            // Has README
            if let Some(readme) = metadata.get("readme_content").and_then(|v| v.as_str()) {
                if readme.len() >= criteria.min_readme_length {
                    score += 20.0;
                }
            }

            // Has documentation
            if metadata.get("has_wiki").and_then(|v| v.as_bool()).unwrap_or(false) {
                score += 10.0;
            }

            // Has examples
            if metadata.contains_key("examples") {
                score += 10.0;
            }
        }

        Ok(score)
    }

    async fn assess_maintenance_indicators(&self, project: &Project, _indicators: &MaintenanceIndicators) -> Result<f32> {
        let mut score = 0.0;

        if let Some(metadata) = project.metadata.as_object() {
            // Regular releases
            if let Some(activity) = metadata.get("recent_activity").and_then(|v| v.as_object()) {
                if let Some(releases) = activity.get("recent_releases").and_then(|v| v.as_i64()) {
                    if releases > 0 {
                        score += 15.0;
                    }
                }
            }

            // Not archived
            if !metadata.get("archived").and_then(|v| v.as_bool()).unwrap_or(false) {
                score += 15.0;
            }

            // Active issues management
            if let Some(issues) = metadata.get("open_issues_count").and_then(|v| v.as_i64()) {
                if issues < 100 {
                    score += 10.0;
                }
            }

            // Recent updates
            if let Some(updated_at) = project.last_updated {
                let days_since_update = (chrono::Utc::now() - updated_at).num_days();
                if days_since_update < 90 {
                    score += 10.0;
                }
            }
        }

        Ok(score)
    }

    async fn assess_funding_indicators(&self, _project: &Project, _indicators: &[String]) -> Result<f32> {
        // This would require additional API calls to check for sponsorship, funding, etc.
        // For now, return a neutral score
        Ok(10.0)
    }

    async fn assess_technology_alignment(&self, project: &Project, alignment: &[String]) -> Result<f32> {
        let mut score = 0.0;

        if let Some(metadata) = project.metadata.as_object() {
            if let Some(language) = metadata.get("language").and_then(|v| v.as_str()) {
                if alignment.contains(&language.to_lowercase()) {
                    score += 30.0;
                }
            }

            // Check topics for technology alignment
            if let Some(topics) = metadata.get("topics").and_then(|v| v.as_array()) {
                for topic in topics {
                    if let Some(topic_str) = topic.as_str() {
                        if alignment.contains(&topic_str.to_lowercase()) {
                            score += 5.0;
                        }
                    }
                }
            }
        }

        Ok(score.min(30.0))
    }

    async fn assess_use_case_similarity(&self, project: &Project, use_cases: &[String]) -> Result<f32> {
        let mut score = 0.0;

        if let Some(description) = &project.description {
            let description_lower = description.to_lowercase();
            for use_case in use_cases {
                if description_lower.contains(&use_case.to_lowercase()) {
                    score += 5.0;
                }
            }
        }

        Ok(score.min(25.0))
    }

    async fn assess_architectural_relevance(&self, project: &Project, relevance: &[String]) -> Result<f32> {
        let mut score = 0.0;

        // Check README content for architectural patterns
        if let Some(metadata) = project.metadata.as_object() {
            if let Some(readme) = metadata.get("readme_content").and_then(|v| v.as_str()) {
                let readme_lower = readme.to_lowercase();
                for pattern in relevance {
                    if readme_lower.contains(&pattern.to_lowercase()) {
                        score += 5.0;
                    }
                }
            }
        }

        Ok(score.min(25.0))
    }

    async fn assess_migration_patterns(&self, project: &Project, patterns: &[String]) -> Result<f32> {
        let mut score = 0.0;

        // Check for migration-related keywords in description and README
        let text_sources = vec![
            project.description.as_deref().unwrap_or(""),
            project.metadata.get("readme_content").and_then(|v| v.as_str()).unwrap_or(""),
        ];

        for text in text_sources {
            let text_lower = text.to_lowercase();
            for pattern in patterns {
                if text_lower.contains(&pattern.to_lowercase()) {
                    score += 5.0;
                }
            }
        }

        Ok(score.min(20.0))
    }

    // Criteria definitions for each category
    fn ide_migration_criteria() -> EvaluationCriteria {
        EvaluationCriteria {
            technical_quality: TechnicalQualityCriteria {
                min_stars: 100,
                min_contributors: 5,
                required_languages: vec!["rust".to_string(), "cpp".to_string(), "go".to_string()],
                architecture_patterns: vec!["migration".to_string(), "performance".to_string()],
                code_quality_indicators: vec!["tests".to_string(), "ci".to_string()],
            },
            adoption_metrics: AdoptionMetricsCriteria {
                min_forks: 20,
                min_downloads: None,
                community_activity: CommunityActivityCriteria {
                    min_recent_commits: 10,
                    max_days_since_update: 90,
                    min_open_issues_ratio: 0.1,
                    min_contributor_diversity: 3,
                },
                documentation_quality: DocumentationCriteria {
                    has_readme: true,
                    has_api_docs: false,
                    has_examples: true,
                    min_readme_length: 1000,
                },
            },
            sustainability: SustainabilityCriteria {
                license_requirements: vec!["mit".to_string(), "apache".to_string(), "bsd".to_string()],
                maintenance_indicators: MaintenanceIndicators {
                    regular_releases: true,
                    responsive_issues: true,
                    active_maintainers: true,
                    security_updates: false,
                },
                funding_indicators: vec![],
            },
            relevance_to_kiro: RelevanceCriteria {
                technology_alignment: vec!["rust".to_string(), "wasm".to_string(), "electron".to_string()],
                use_case_similarity: vec!["ide".to_string(), "editor".to_string(), "migration".to_string()],
                architectural_relevance: vec!["extension".to_string(), "plugin".to_string(), "performance".to_string()],
                migration_patterns: vec!["electron".to_string(), "native".to_string(), "rewrite".to_string()],
            },
        }
    }

    fn vscode_fork_criteria() -> EvaluationCriteria {
        EvaluationCriteria {
            technical_quality: TechnicalQualityCriteria {
                min_stars: 500,
                min_contributors: 10,
                required_languages: vec!["typescript".to_string(), "javascript".to_string()],
                architecture_patterns: vec!["fork".to_string(), "customization".to_string()],
                code_quality_indicators: vec!["tests".to_string(), "ci".to_string()],
            },
            adoption_metrics: AdoptionMetricsCriteria {
                min_forks: 50,
                min_downloads: Some(1000),
                community_activity: CommunityActivityCriteria {
                    min_recent_commits: 20,
                    max_days_since_update: 30,
                    min_open_issues_ratio: 0.1,
                    min_contributor_diversity: 5,
                },
                documentation_quality: DocumentationCriteria {
                    has_readme: true,
                    has_api_docs: true,
                    has_examples: true,
                    min_readme_length: 2000,
                },
            },
            sustainability: SustainabilityCriteria {
                license_requirements: vec!["mit".to_string()],
                maintenance_indicators: MaintenanceIndicators {
                    regular_releases: true,
                    responsive_issues: true,
                    active_maintainers: true,
                    security_updates: true,
                },
                funding_indicators: vec!["sponsors".to_string()],
            },
            relevance_to_kiro: RelevanceCriteria {
                technology_alignment: vec!["vscode".to_string(), "electron".to_string(), "typescript".to_string()],
                use_case_similarity: vec!["ide".to_string(), "editor".to_string(), "development".to_string()],
                architectural_relevance: vec!["extension".to_string(), "marketplace".to_string(), "customization".to_string()],
                migration_patterns: vec!["fork".to_string(), "branding".to_string(), "features".to_string()],
            },
        }
    }

    fn rust_ide_criteria() -> EvaluationCriteria {
        EvaluationCriteria {
            technical_quality: TechnicalQualityCriteria {
                min_stars: 200,
                min_contributors: 3,
                required_languages: vec!["rust".to_string()],
                architecture_patterns: vec!["native".to_string(), "performance".to_string()],
                code_quality_indicators: vec!["tests".to_string(), "clippy".to_string()],
            },
            adoption_metrics: AdoptionMetricsCriteria {
                min_forks: 30,
                min_downloads: None,
                community_activity: CommunityActivityCriteria {
                    min_recent_commits: 15,
                    max_days_since_update: 60,
                    min_open_issues_ratio: 0.1,
                    min_contributor_diversity: 2,
                },
                documentation_quality: DocumentationCriteria {
                    has_readme: true,
                    has_api_docs: true,
                    has_examples: true,
                    min_readme_length: 1500,
                },
            },
            sustainability: SustainabilityCriteria {
                license_requirements: vec!["mit".to_string(), "apache".to_string()],
                maintenance_indicators: MaintenanceIndicators {
                    regular_releases: true,
                    responsive_issues: true,
                    active_maintainers: true,
                    security_updates: false,
                },
                funding_indicators: vec![],
            },
            relevance_to_kiro: RelevanceCriteria {
                technology_alignment: vec!["rust".to_string(), "native".to_string(), "performance".to_string()],
                use_case_similarity: vec!["ide".to_string(), "editor".to_string(), "development".to_string()],
                architectural_relevance: vec!["gui".to_string(), "native".to_string(), "cross-platform".to_string()],
                migration_patterns: vec!["native".to_string(), "performance".to_string(), "rust".to_string()],
            },
        }
    }

    fn ai_ide_criteria() -> EvaluationCriteria {
        EvaluationCriteria {
            technical_quality: TechnicalQualityCriteria {
                min_stars: 1000,
                min_contributors: 10,
                required_languages: vec!["typescript".to_string(), "python".to_string(), "rust".to_string()],
                architecture_patterns: vec!["ai".to_string(), "llm".to_string()],
                code_quality_indicators: vec!["tests".to_string(), "ci".to_string()],
            },
            adoption_metrics: AdoptionMetricsCriteria {
                min_forks: 100,
                min_downloads: Some(5000),
                community_activity: CommunityActivityCriteria {
                    min_recent_commits: 30,
                    max_days_since_update: 14,
                    min_open_issues_ratio: 0.1,
                    min_contributor_diversity: 8,
                },
                documentation_quality: DocumentationCriteria {
                    has_readme: true,
                    has_api_docs: true,
                    has_examples: true,
                    min_readme_length: 3000,
                },
            },
            sustainability: SustainabilityCriteria {
                license_requirements: vec!["mit".to_string(), "apache".to_string()],
                maintenance_indicators: MaintenanceIndicators {
                    regular_releases: true,
                    responsive_issues: true,
                    active_maintainers: true,
                    security_updates: true,
                },
                funding_indicators: vec!["funding".to_string(), "commercial".to_string()],
            },
            relevance_to_kiro: RelevanceCriteria {
                technology_alignment: vec!["ai".to_string(), "llm".to_string(), "gpt".to_string()],
                use_case_similarity: vec!["ide".to_string(), "ai".to_string(), "assistant".to_string()],
                architectural_relevance: vec!["integration".to_string(), "api".to_string(), "streaming".to_string()],
                migration_patterns: vec!["ai".to_string(), "integration".to_string(), "performance".to_string()],
            },
        }
    }

    fn wasm_app_criteria() -> EvaluationCriteria {
        EvaluationCriteria {
            technical_quality: TechnicalQualityCriteria {
                min_stars: 150,
                min_contributors: 3,
                required_languages: vec!["rust".to_string(), "javascript".to_string(), "typescript".to_string()],
                architecture_patterns: vec!["wasm".to_string(), "web".to_string()],
                code_quality_indicators: vec!["tests".to_string(), "wasm-pack".to_string()],
            },
            adoption_metrics: AdoptionMetricsCriteria {
                min_forks: 25,
                min_downloads: None,
                community_activity: CommunityActivityCriteria {
                    min_recent_commits: 10,
                    max_days_since_update: 90,
                    min_open_issues_ratio: 0.1,
                    min_contributor_diversity: 2,
                },
                documentation_quality: DocumentationCriteria {
                    has_readme: true,
                    has_api_docs: false,
                    has_examples: true,
                    min_readme_length: 1000,
                },
            },
            sustainability: SustainabilityCriteria {
                license_requirements: vec!["mit".to_string(), "apache".to_string()],
                maintenance_indicators: MaintenanceIndicators {
                    regular_releases: false,
                    responsive_issues: true,
                    active_maintainers: true,
                    security_updates: false,
                },
                funding_indicators: vec![],
            },
            relevance_to_kiro: RelevanceCriteria {
                technology_alignment: vec!["wasm".to_string(), "rust".to_string(), "web".to_string()],
                use_case_similarity: vec!["tools".to_string(), "editor".to_string(), "development".to_string()],
                architectural_relevance: vec!["interop".to_string(), "performance".to_string(), "browser".to_string()],
                migration_patterns: vec!["wasm".to_string(), "performance".to_string(), "web".to_string()],
            },
        }
    }

    fn technical_pattern_criteria() -> EvaluationCriteria {
        EvaluationCriteria {
            technical_quality: TechnicalQualityCriteria {
                min_stars: 50,
                min_contributors: 2,
                required_languages: vec!["rust".to_string(), "typescript".to_string(), "javascript".to_string()],
                architecture_patterns: vec!["pattern".to_string(), "architecture".to_string()],
                code_quality_indicators: vec!["documentation".to_string()],
            },
            adoption_metrics: AdoptionMetricsCriteria {
                min_forks: 10,
                min_downloads: None,
                community_activity: CommunityActivityCriteria {
                    min_recent_commits: 5,
                    max_days_since_update: 180,
                    min_open_issues_ratio: 0.1,
                    min_contributor_diversity: 1,
                },
                documentation_quality: DocumentationCriteria {
                    has_readme: true,
                    has_api_docs: false,
                    has_examples: true,
                    min_readme_length: 500,
                },
            },
            sustainability: SustainabilityCriteria {
                license_requirements: vec!["mit".to_string(), "apache".to_string(), "bsd".to_string()],
                maintenance_indicators: MaintenanceIndicators {
                    regular_releases: false,
                    responsive_issues: false,
                    active_maintainers: false,
                    security_updates: false,
                },
                funding_indicators: vec![],
            },
            relevance_to_kiro: RelevanceCriteria {
                technology_alignment: vec!["pattern".to_string(), "architecture".to_string(), "design".to_string()],
                use_case_similarity: vec!["development".to_string(), "tools".to_string(), "framework".to_string()],
                architectural_relevance: vec!["pattern".to_string(), "best-practices".to_string(), "design".to_string()],
                migration_patterns: vec!["pattern".to_string(), "refactor".to_string(), "architecture".to_string()],
            },
        }
    }
}