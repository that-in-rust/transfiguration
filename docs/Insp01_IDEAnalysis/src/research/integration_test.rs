// Integration Test for IDE Migration Research System
// Demonstrates complete research workflow from data collection to analysis

#[cfg(test)]
mod tests {
    use super::super::{
        data_collector::DataCollector,
        migration_analyzer::MigrationAnalyzer,
        performance_analyzer::PerformanceAnalyzer,
        analysis_report::AnalysisReportGenerator,
    };

    #[tokio::test]
    async fn test_complete_research_workflow() {
        // Step 1: Initialize data collector and collect project data
        let mut data_collector = DataCollector::new();
        let projects = data_collector.collect_migration_project_data().await
            .expect("Failed to collect project data");

        assert!(!projects.is_empty(), "Should collect at least some projects");
        println!("Collected {} projects for analysis", projects.len());

        // Verify we have different categories of projects
        let categories: std::collections::HashSet<_> = projects.iter()
            .map(|p| &p.category)
            .collect();
        assert!(categories.len() > 1, "Should have multiple project categories");

        // Step 2: Perform migration analysis on collected projects
        let migration_analyzer = MigrationAnalyzer::new();
        let mut migration_analyses = Vec::new();

        for project in &projects {
            let analysis = migration_analyzer.analyze_migration_project(project).await
                .expect("Failed to analyze migration project");
            migration_analyses.push(analysis);
        }

        assert_eq!(migration_analyses.len(), projects.len(), 
                  "Should have analysis for each project");

        // Step 3: Perform performance analysis on collected projects
        let performance_analyzer = PerformanceAnalyzer::new();
        let mut performance_analyses = Vec::new();

        for project in &projects {
            let analysis = performance_analyzer.analyze_performance(project).await
                .expect("Failed to analyze performance");
            performance_analyses.push(analysis);
        }

        assert_eq!(performance_analyses.len(), projects.len(),
                  "Should have performance analysis for each project");

        // Step 4: Generate comprehensive analysis report
        let report_generator = AnalysisReportGenerator::new(
            projects.clone(),
            migration_analyses.clone(),
            performance_analyses.clone(),
        );

        let comprehensive_report = report_generator.generate_comprehensive_report()
            .expect("Failed to generate comprehensive report");

        // Verify report structure and content
        assert_eq!(comprehensive_report.metadata.projects_analyzed, projects.len() as i32);
        assert!(!comprehensive_report.executive_summary.key_findings.is_empty());
        assert!(!comprehensive_report.executive_summary.strategic_recommendations.is_empty());

        // Verify project analysis section
        assert_eq!(comprehensive_report.project_analysis.total_projects, projects.len() as i32);
        assert!(!comprehensive_report.project_analysis.category_breakdown.is_empty());
        assert!(!comprehensive_report.project_analysis.top_performing_projects.is_empty());

        // Step 5: Validate specific research findings
        validate_research_findings(&projects, &migration_analyses, &performance_analyses);

        println!("✅ Complete research workflow test passed");
        println!("📊 Generated comprehensive report with {} sections", 8);
        println!("🎯 Analyzed {} projects across {} categories", 
                projects.len(), 
                comprehensive_report.project_analysis.category_breakdown.len());
    }

    fn validate_research_findings(
        projects: &[super::super::MigrationProject],
        migration_analyses: &[super::super::migration_analyzer::MigrationAnalysisResult],
        performance_analyses: &[super::super::performance_analyzer::PerformanceAnalysisResult],
    ) {
        // Validate that we have key projects in our analysis
        let project_names: Vec<&str> = projects.iter().map(|p| p.name.as_str()).collect();
        
        // Should include major Rust IDEs
        assert!(project_names.contains(&"Zed"), "Should include Zed editor");
        assert!(project_names.contains(&"Lapce"), "Should include Lapce editor");
        assert!(project_names.contains(&"Helix"), "Should include Helix editor");
        
        // Should include migration frameworks
        assert!(project_names.contains(&"Tauri"), "Should include Tauri framework");
        
        // Should include VS Code forks
        assert!(project_names.contains(&"VSCodium"), "Should include VSCodium");

        // Validate migration analysis results
        for analysis in migration_analyses {
            assert!(analysis.technical_approach_score >= 0.0 && analysis.technical_approach_score <= 10.0,
                   "Technical approach score should be in valid range");
            assert!(!analysis.success_indicators.is_empty(), 
                   "Should identify success indicators");
            assert!(!analysis.lessons_learned.is_empty(),
                   "Should extract lessons learned");
        }

        // Validate performance analysis results
        for analysis in performance_analyses {
            assert!(analysis.overall_performance_score >= 0.0 && analysis.overall_performance_score <= 10.0,
                   "Performance score should be in valid range");
            assert!(!analysis.category_scores.is_empty(),
                   "Should have category scores");
            assert!(!analysis.recommendations.is_empty(),
                   "Should have performance recommendations");
        }

        // Validate that Rust projects show good performance characteristics
        let rust_projects: Vec<_> = projects.iter()
            .filter(|p| {
                if let Some(tech_details) = &p.technical_details {
                    matches!(tech_details.target_technology, super::super::TargetTechnology::Rust)
                } else {
                    false
                }
            })
            .collect();

        assert!(!rust_projects.is_empty(), "Should have Rust-based projects");

        // Validate that performance improvements are documented
        let projects_with_improvements: Vec<_> = projects.iter()
            .filter(|p| {
                if let Some(tech_details) = &p.technical_details {
                    tech_details.performance_metrics.values()
                        .any(|metric| metric.improvement_percentage.unwrap_or(0.0) > 0.0)
                } else {
                    false
                }
            })
            .collect();

        assert!(!projects_with_improvements.is_empty(), 
               "Should have projects with documented performance improvements");

        println!("✅ Research findings validation passed");
        println!("🦀 Found {} Rust-based projects", rust_projects.len());
        println!("📈 Found {} projects with performance improvements", 
                projects_with_improvements.len());
    }

    #[tokio::test]
    async fn test_kiro_applicability_assessment() {
        let mut data_collector = DataCollector::new();
        let projects = data_collector.collect_migration_project_data().await
            .expect("Failed to collect project data");

        // Find projects with high Kiro applicability
        let highly_applicable: Vec<_> = projects.iter()
            .filter(|p| {
                if let Some(analysis) = &p.analysis {
                    matches!(analysis.applicability_to_kiro, super::super::ApplicabilityLevel::High)
                } else {
                    false
                }
            })
            .collect();

        assert!(!highly_applicable.is_empty(), 
               "Should identify projects with high Kiro applicability");

        // VS Code forks should be highly applicable
        let vscode_forks: Vec<_> = projects.iter()
            .filter(|p| matches!(p.category, super::super::ProjectCategory::VsCodeFork))
            .collect();

        for fork in vscode_forks {
            if let Some(analysis) = &fork.analysis {
                assert!(matches!(analysis.applicability_to_kiro, 
                               super::super::ApplicabilityLevel::High | 
                               super::super::ApplicabilityLevel::Medium),
                       "VS Code forks should have medium to high applicability");
            }
        }

        println!("✅ Kiro applicability assessment test passed");
        println!("🎯 Found {} highly applicable projects", highly_applicable.len());
    }

    #[tokio::test]
    async fn test_performance_pattern_extraction() {
        let performance_analyzer = PerformanceAnalyzer::new();
        
        // Test with a sample Rust project
        let rust_project = create_sample_rust_project();
        let analysis = performance_analyzer.analyze_performance(&rust_project).await
            .expect("Failed to analyze performance");

        // Should identify relevant optimization patterns
        assert!(!analysis.identified_optimizations.is_empty(),
               "Should identify optimization patterns");

        // Should have performance recommendations
        assert!(!analysis.recommendations.is_empty(),
               "Should have performance recommendations");

        // Should have benchmarking methodology
        assert!(!analysis.benchmarking_methodology.tools_used.is_empty(),
               "Should suggest benchmarking tools");

        println!("✅ Performance pattern extraction test passed");
        println!("🔧 Identified {} optimization patterns", analysis.identified_optimizations.len());
        println!("💡 Generated {} recommendations", analysis.recommendations.len());
    }

    #[tokio::test]
    async fn test_migration_strategy_analysis() {
        let migration_analyzer = MigrationAnalyzer::new();
        
        // Test with different migration approaches
        let gradual_project = create_sample_gradual_migration_project();
        let rewrite_project = create_sample_rewrite_project();

        let gradual_analysis = migration_analyzer.analyze_migration_project(&gradual_project).await
            .expect("Failed to analyze gradual migration");

        let rewrite_analysis = migration_analyzer.analyze_migration_project(&rewrite_project).await
            .expect("Failed to analyze rewrite project");

        // Gradual migration should generally have lower complexity
        assert!(matches!(gradual_analysis.migration_complexity, 
                        super::super::migration_analyzer::MigrationComplexity::Low |
                        super::super::migration_analyzer::MigrationComplexity::Medium),
               "Gradual migration should have lower complexity");

        // Complete rewrite should have higher complexity
        assert!(matches!(rewrite_analysis.migration_complexity,
                        super::super::migration_analyzer::MigrationComplexity::High |
                        super::super::migration_analyzer::MigrationComplexity::VeryHigh),
               "Complete rewrite should have higher complexity");

        println!("✅ Migration strategy analysis test passed");
        println!("📊 Gradual migration complexity: {:?}", gradual_analysis.migration_complexity);
        println!("📊 Rewrite migration complexity: {:?}", rewrite_analysis.migration_complexity);
    }

    // Helper functions to create sample projects for testing

    fn create_sample_rust_project() -> super::super::MigrationProject {
        use std::collections::HashMap;
        
        let mut performance_metrics = HashMap::new();
        performance_metrics.insert("startup_time".to_string(), super::super::PerformanceMetric {
            before_value: Some(3000.0),
            after_value: Some(500.0),
            unit: "milliseconds".to_string(),
            improvement_percentage: Some(500.0), // 6x improvement
            measurement_context: Some("Test data".to_string()),
            source_url: None,
            verified: false,
        });

        super::super::MigrationProject {
            id: None,
            name: "Test Rust IDE".to_string(),
            url: "https://example.com".to_string(),
            github_url: Some("https://github.com/test/rust-ide".to_string()),
            category: super::super::ProjectCategory::RustIde,
            status: super::super::ProjectStatus::Active,
            last_updated: None,
            stars: 5000,
            contributors: 50,
            technical_details: Some(super::super::TechnicalDetails {
                source_technology: super::super::SourceTechnology::Electron,
                target_technology: super::super::TargetTechnology::Rust,
                migration_approach: super::super::MigrationApproach::CompleteRewrite,
                architecture_patterns: vec!["Native Compilation".to_string()],
                performance_metrics,
                migration_timeline_months: Some(24),
                team_size: Some(10),
            }),
            analysis: Some(super::super::ProjectAnalysis {
                strengths: vec!["High performance".to_string()],
                weaknesses: vec!["Learning curve".to_string()],
                lessons_learned: vec!["Rust provides excellent performance".to_string()],
                applicability_to_kiro: super::super::ApplicabilityLevel::High,
                confidence_level: super::super::ConfidenceLevel::High,
                technical_score: 9,
                adoption_score: 8,
                sustainability_score: 8,
                relevance_score: 9,
                overall_score: 8.5,
                analysis_notes: Some("Test project analysis".to_string()),
            }),
        }
    }

    fn create_sample_gradual_migration_project() -> super::super::MigrationProject {
        super::super::MigrationProject {
            id: None,
            name: "Gradual Migration Test".to_string(),
            url: "https://example.com".to_string(),
            github_url: None,
            category: super::super::ProjectCategory::IdeMigration,
            status: super::super::ProjectStatus::Active,
            last_updated: None,
            stars: 2000,
            contributors: 15,
            technical_details: Some(super::super::TechnicalDetails {
                source_technology: super::super::SourceTechnology::Electron,
                target_technology: super::super::TargetTechnology::Tauri,
                migration_approach: super::super::MigrationApproach::Gradual,
                architecture_patterns: vec!["Incremental Migration".to_string()],
                performance_metrics: std::collections::HashMap::new(),
                migration_timeline_months: Some(12),
                team_size: Some(5),
            }),
            analysis: None,
        }
    }

    fn create_sample_rewrite_project() -> super::super::MigrationProject {
        super::super::MigrationProject {
            id: None,
            name: "Complete Rewrite Test".to_string(),
            url: "https://example.com".to_string(),
            github_url: None,
            category: super::super::ProjectCategory::RustIde,
            status: super::super::ProjectStatus::Active,
            last_updated: None,
            stars: 8000,
            contributors: 25,
            technical_details: Some(super::super::TechnicalDetails {
                source_technology: super::super::SourceTechnology::Native,
                target_technology: super::super::TargetTechnology::Rust,
                migration_approach: super::super::MigrationApproach::CompleteRewrite,
                architecture_patterns: vec!["Ground-Up Rewrite".to_string()],
                performance_metrics: std::collections::HashMap::new(),
                migration_timeline_months: Some(36),
                team_size: Some(20),
            }),
            analysis: None,
        }
    }
}