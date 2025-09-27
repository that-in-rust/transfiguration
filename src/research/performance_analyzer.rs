// Performance Analyzer for IDE Migration Research
// Extracts and analyzes performance optimization patterns and techniques

use super::{MigrationProject, PerformanceMetric, ResearchResult, ResearchError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PerformanceAnalyzer {
    benchmark_categories: Vec<BenchmarkCategory>,
    optimization_patterns: Vec<OptimizationPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkCategory {
    pub name: String,
    pub description: String,
    pub metrics: Vec<MetricDefinition>,
    pub importance_weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricDefinition {
    pub name: String,
    pub unit: String,
    pub measurement_method: String,
    pub baseline_expectations: BaselineExpectations,
    pub improvement_thresholds: ImprovementThresholds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineExpectations {
    pub electron_typical: f64,
    pub native_typical: f64,
    pub acceptable_range: (f64, f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImprovementThresholds {
    pub minimal: f64,      // 10-20% improvement
    pub significant: f64,  // 50-100% improvement
    pub dramatic: f64,     // 200%+ improvement
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationPattern {
    pub name: String,
    pub category: String,
    pub description: String,
    pub implementation_techniques: Vec<String>,
    pub expected_improvements: HashMap<String, f64>,
    pub complexity: OptimizationComplexity,
    pub applicability_conditions: Vec<String>,
    pub observed_projects: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationComplexity {
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAnalysisResult {
    pub project_name: String,
    pub overall_performance_score: f64,
    pub category_scores: HashMap<String, f64>,
    pub identified_optimizations: Vec<IdentifiedOptimization>,
    pub performance_patterns: Vec<PerformancePattern>,
    pub benchmarking_methodology: BenchmarkingMethodology,
    pub recommendations: Vec<PerformanceRecommendation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentifiedOptimization {
    pub pattern_name: String,
    pub category: String,
    pub description: String,
    pub measured_improvement: Option<f64>,
    pub implementation_details: String,
    pub confidence_level: f64,
    pub source_evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformancePattern {
    pub pattern_type: String,
    pub description: String,
    pub metrics_affected: Vec<String>,
    pub improvement_range: (f64, f64),
    pub implementation_complexity: OptimizationComplexity,
    pub prerequisites: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkingMethodology {
    pub measurement_approach: String,
    pub tools_used: Vec<String>,
    pub test_scenarios: Vec<String>,
    pub reliability_indicators: Vec<String>,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRecommendation {
    pub category: String,
    pub recommendation: String,
    pub expected_impact: String,
    pub implementation_effort: String,
    pub priority: RecommendationPriority,
    pub supporting_evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationPriority {
    Critical,
    High,
    Medium,
    Low,
}

impl PerformanceAnalyzer {
    pub fn new() -> Self {
        Self {
            benchmark_categories: Self::create_benchmark_categories(),
            optimization_patterns: Self::create_optimization_patterns(),
        }
    }

    fn create_benchmark_categories() -> Vec<BenchmarkCategory> {
        vec![
            BenchmarkCategory {
                name: "Startup Performance".to_string(),
                description: "Application startup and initialization time".to_string(),
                metrics: vec![
                    MetricDefinition {
                        name: "Cold Start Time".to_string(),
                        unit: "milliseconds".to_string(),
                        measurement_method: "Time from process start to UI ready".to_string(),
                        baseline_expectations: BaselineExpectations {
                            electron_typical: 3000.0,
                            native_typical: 500.0,
                            acceptable_range: (200.0, 2000.0),
                        },
                        improvement_thresholds: ImprovementThresholds {
                            minimal: 20.0,
                            significant: 50.0,
                            dramatic: 200.0,
                        },
                    },
                    MetricDefinition {
                        name: "Warm Start Time".to_string(),
                        unit: "milliseconds".to_string(),
                        measurement_method: "Time for subsequent application launches".to_string(),
                        baseline_expectations: BaselineExpectations {
                            electron_typical: 1500.0,
                            native_typical: 200.0,
                            acceptable_range: (100.0, 1000.0),
                        },
                        improvement_thresholds: ImprovementThresholds {
                            minimal: 25.0,
                            significant: 60.0,
                            dramatic: 300.0,
                        },
                    },
                ],
                importance_weight: 0.25,
            },

            BenchmarkCategory {
                name: "Memory Usage".to_string(),
                description: "Memory consumption and efficiency".to_string(),
                metrics: vec![
                    MetricDefinition {
                        name: "Initial Memory Usage".to_string(),
                        unit: "MB".to_string(),
                        measurement_method: "Memory usage after startup with empty workspace".to_string(),
                        baseline_expectations: BaselineExpectations {
                            electron_typical: 200.0,
                            native_typical: 50.0,
                            acceptable_range: (30.0, 150.0),
                        },
                        improvement_thresholds: ImprovementThresholds {
                            minimal: 20.0,
                            significant: 50.0,
                            dramatic: 150.0,
                        },
                    },
                    MetricDefinition {
                        name: "Working Set Memory".to_string(),
                        unit: "MB".to_string(),
                        measurement_method: "Memory usage with typical project loaded".to_string(),
                        baseline_expectations: BaselineExpectations {
                            electron_typical: 500.0,
                            native_typical: 150.0,
                            acceptable_range: (100.0, 400.0),
                        },
                        improvement_thresholds: ImprovementThresholds {
                            minimal: 15.0,
                            significant: 40.0,
                            dramatic: 100.0,
                        },
                    },
                ],
                importance_weight: 0.20,
            },

            BenchmarkCategory {
                name: "CPU Usage".to_string(),
                description: "CPU utilization and efficiency".to_string(),
                metrics: vec![
                    MetricDefinition {
                        name: "Idle CPU Usage".to_string(),
                        unit: "percentage".to_string(),
                        measurement_method: "CPU usage when application is idle".to_string(),
                        baseline_expectations: BaselineExpectations {
                            electron_typical: 2.0,
                            native_typical: 0.5,
                            acceptable_range: (0.1, 1.5),
                        },
                        improvement_thresholds: ImprovementThresholds {
                            minimal: 25.0,
                            significant: 75.0,
                            dramatic: 200.0,
                        },
                    },
                    MetricDefinition {
                        name: "Active CPU Usage".to_string(),
                        unit: "percentage".to_string(),
                        measurement_method: "CPU usage during typical editing tasks".to_string(),
                        baseline_expectations: BaselineExpectations {
                            electron_typical: 15.0,
                            native_typical: 5.0,
                            acceptable_range: (2.0, 12.0),
                        },
                        improvement_thresholds: ImprovementThresholds {
                            minimal: 20.0,
                            significant: 50.0,
                            dramatic: 150.0,
                        },
                    },
                ],
                importance_weight: 0.15,
            },

            BenchmarkCategory {
                name: "Response Time".to_string(),
                description: "User interaction responsiveness".to_string(),
                metrics: vec![
                    MetricDefinition {
                        name: "Keystroke Latency".to_string(),
                        unit: "milliseconds".to_string(),
                        measurement_method: "Time from keypress to character display".to_string(),
                        baseline_expectations: BaselineExpectations {
                            electron_typical: 50.0,
                            native_typical: 10.0,
                            acceptable_range: (5.0, 30.0),
                        },
                        improvement_thresholds: ImprovementThresholds {
                            minimal: 20.0,
                            significant: 60.0,
                            dramatic: 200.0,
                        },
                    },
                    MetricDefinition {
                        name: "File Open Time".to_string(),
                        unit: "milliseconds".to_string(),
                        measurement_method: "Time to open and display large file".to_string(),
                        baseline_expectations: BaselineExpectations {
                            electron_typical: 2000.0,
                            native_typical: 500.0,
                            acceptable_range: (200.0, 1500.0),
                        },
                        improvement_thresholds: ImprovementThresholds {
                            minimal: 25.0,
                            significant: 75.0,
                            dramatic: 300.0,
                        },
                    },
                ],
                importance_weight: 0.25,
            },

            BenchmarkCategory {
                name: "Bundle Size".to_string(),
                description: "Application size and distribution efficiency".to_string(),
                metrics: vec![
                    MetricDefinition {
                        name: "Application Bundle Size".to_string(),
                        unit: "MB".to_string(),
                        measurement_method: "Total size of distributed application".to_string(),
                        baseline_expectations: BaselineExpectations {
                            electron_typical: 150.0,
                            native_typical: 20.0,
                            acceptable_range: (10.0, 100.0),
                        },
                        improvement_thresholds: ImprovementThresholds {
                            minimal: 20.0,
                            significant: 50.0,
                            dramatic: 200.0,
                        },
                    },
                ],
                importance_weight: 0.15,
            },
        ]
    }

    fn create_optimization_patterns() -> Vec<OptimizationPattern> {
        vec![
            OptimizationPattern {
                name: "Native Compilation".to_string(),
                category: "Architecture".to_string(),
                description: "Compile to native machine code instead of interpreted/JIT execution".to_string(),
                implementation_techniques: vec![
                    "Use Rust for core logic compilation".to_string(),
                    "Compile-time optimizations with LLVM".to_string(),
                    "Link-time optimization (LTO)".to_string(),
                    "Profile-guided optimization (PGO)".to_string(),
                ],
                expected_improvements: {
                    let mut improvements = HashMap::new();
                    improvements.insert("startup_time".to_string(), 200.0);
                    improvements.insert("cpu_usage".to_string(), 150.0);
                    improvements.insert("memory_usage".to_string(), 100.0);
                    improvements
                },
                complexity: OptimizationComplexity::High,
                applicability_conditions: vec![
                    "Performance-critical application".to_string(),
                    "Team has native development expertise".to_string(),
                    "Acceptable to rewrite significant portions".to_string(),
                ],
                observed_projects: vec!["Zed".to_string(), "Lapce".to_string(), "Helix".to_string()],
            },

            OptimizationPattern {
                name: "Lazy Loading".to_string(),
                category: "Resource Management".to_string(),
                description: "Load resources and modules only when needed".to_string(),
                implementation_techniques: vec![
                    "Dynamic module loading".to_string(),
                    "On-demand asset loading".to_string(),
                    "Lazy initialization of components".to_string(),
                    "Code splitting and tree shaking".to_string(),
                ],
                expected_improvements: {
                    let mut improvements = HashMap::new();
                    improvements.insert("startup_time".to_string(), 150.0);
                    improvements.insert("memory_usage".to_string(), 75.0);
                    improvements.insert("bundle_size".to_string(), 50.0);
                    improvements
                },
                complexity: OptimizationComplexity::Medium,
                applicability_conditions: vec![
                    "Large application with many features".to_string(),
                    "Modular architecture".to_string(),
                    "Clear feature boundaries".to_string(),
                ],
                observed_projects: vec!["VS Code".to_string(), "Theia".to_string(), "Cursor".to_string()],
            },

            OptimizationPattern {
                name: "Memory Pool Allocation".to_string(),
                category: "Memory Management".to_string(),
                description: "Use pre-allocated memory pools to reduce allocation overhead".to_string(),
                implementation_techniques: vec![
                    "Object pooling for frequently created objects".to_string(),
                    "Arena allocation for temporary objects".to_string(),
                    "Custom allocators for specific use cases".to_string(),
                    "Memory mapping for large files".to_string(),
                ],
                expected_improvements: {
                    let mut improvements = HashMap::new();
                    improvements.insert("memory_usage".to_string(), 100.0);
                    improvements.insert("cpu_usage".to_string(), 50.0);
                    improvements.insert("response_time".to_string(), 75.0);
                    improvements
                },
                complexity: OptimizationComplexity::High,
                applicability_conditions: vec![
                    "High-frequency object allocation".to_string(),
                    "Predictable memory usage patterns".to_string(),
                    "Performance-critical paths identified".to_string(),
                ],
                observed_projects: vec!["Xi Editor".to_string(), "Zed".to_string()],
            },

            OptimizationPattern {
                name: "Incremental Rendering".to_string(),
                category: "UI Performance".to_string(),
                description: "Only re-render changed portions of the UI".to_string(),
                implementation_techniques: vec![
                    "Virtual DOM with diffing".to_string(),
                    "Dirty region tracking".to_string(),
                    "Component-level memoization".to_string(),
                    "Viewport-based rendering".to_string(),
                ],
                expected_improvements: {
                    let mut improvements = HashMap::new();
                    improvements.insert("response_time".to_string(), 200.0);
                    improvements.insert("cpu_usage".to_string(), 100.0);
                    improvements.insert("keystroke_latency".to_string(), 150.0);
                    improvements
                },
                complexity: OptimizationComplexity::Medium,
                applicability_conditions: vec![
                    "Complex UI with frequent updates".to_string(),
                    "Large documents or datasets".to_string(),
                    "Real-time collaboration features".to_string(),
                ],
                observed_projects: vec!["Zed".to_string(), "Lapce".to_string(), "Fleet".to_string()],
            },

            OptimizationPattern {
                name: "Background Processing".to_string(),
                category: "Concurrency".to_string(),
                description: "Move heavy computations to background threads".to_string(),
                implementation_techniques: vec![
                    "Worker threads for CPU-intensive tasks".to_string(),
                    "Async/await for I/O operations".to_string(),
                    "Message passing between threads".to_string(),
                    "Lock-free data structures".to_string(),
                ],
                expected_improvements: {
                    let mut improvements = HashMap::new();
                    improvements.insert("response_time".to_string(), 300.0);
                    improvements.insert("keystroke_latency".to_string(), 200.0);
                    improvements.insert("cpu_usage".to_string(), 50.0);
                    improvements
                },
                complexity: OptimizationComplexity::High,
                applicability_conditions: vec![
                    "CPU-intensive operations identified".to_string(),
                    "Multi-core systems targeted".to_string(),
                    "Thread-safe architecture possible".to_string(),
                ],
                observed_projects: vec!["Rust Analyzer".to_string(), "Zed".to_string(), "Helix".to_string()],
            },

            OptimizationPattern {
                name: "Caching Strategies".to_string(),
                category: "Data Management".to_string(),
                description: "Cache frequently accessed data and computations".to_string(),
                implementation_techniques: vec![
                    "In-memory caching of parsed files".to_string(),
                    "Persistent caching across sessions".to_string(),
                    "Incremental computation caching".to_string(),
                    "LRU eviction policies".to_string(),
                ],
                expected_improvements: {
                    let mut improvements = HashMap::new();
                    improvements.insert("file_open_time".to_string(), 400.0);
                    improvements.insert("response_time".to_string(), 150.0);
                    improvements.insert("cpu_usage".to_string(), 75.0);
                    improvements
                },
                complexity: OptimizationComplexity::Medium,
                applicability_conditions: vec![
                    "Repeated computations identified".to_string(),
                    "Sufficient memory available".to_string(),
                    "Cache invalidation strategy defined".to_string(),
                ],
                observed_projects: vec!["Language servers".to_string(), "VS Code".to_string(), "IntelliJ".to_string()],
            },

            OptimizationPattern {
                name: "WASM Optimization".to_string(),
                category: "Web Assembly".to_string(),
                description: "Optimize WebAssembly modules for performance".to_string(),
                implementation_techniques: vec![
                    "SIMD instructions for parallel processing".to_string(),
                    "Memory layout optimization".to_string(),
                    "Minimal JS-WASM boundary crossings".to_string(),
                    "Bulk memory operations".to_string(),
                ],
                expected_improvements: {
                    let mut improvements = HashMap::new();
                    improvements.insert("cpu_usage".to_string(), 200.0);
                    improvements.insert("response_time".to_string(), 150.0);
                    improvements.insert("memory_usage".to_string(), 50.0);
                    improvements
                },
                complexity: OptimizationComplexity::High,
                applicability_conditions: vec![
                    "WASM-compatible target platform".to_string(),
                    "Computationally intensive operations".to_string(),
                    "Modern browser support required".to_string(),
                ],
                observed_projects: vec!["Figma".to_string(), "AutoCAD Web".to_string(), "Photoshop Web".to_string()],
            },
        ]
    }

    /// Analyze performance characteristics of a migration project
    pub async fn analyze_performance(&self, project: &MigrationProject) -> ResearchResult<PerformanceAnalysisResult> {
        let overall_performance_score = self.calculate_overall_performance_score(project).await?;
        let category_scores = self.calculate_category_scores(project).await?;
        let identified_optimizations = self.identify_optimizations(project).await?;
        let performance_patterns = self.extract_performance_patterns(project).await?;
        let benchmarking_methodology = self.analyze_benchmarking_methodology(project).await?;
        let recommendations = self.generate_performance_recommendations(project, &identified_optimizations).await?;

        Ok(PerformanceAnalysisResult {
            project_name: project.name.clone(),
            overall_performance_score,
            category_scores,
            identified_optimizations,
            performance_patterns,
            benchmarking_methodology,
            recommendations,
        })
    }

    async fn calculate_overall_performance_score(&self, project: &MigrationProject) -> ResearchResult<f64> {
        let mut weighted_score = 0.0;
        let mut total_weight = 0.0;

        for category in &self.benchmark_categories {
            let category_score = self.calculate_category_score(project, category).await?;
            weighted_score += category_score * category.importance_weight;
            total_weight += category.importance_weight;
        }

        Ok(if total_weight > 0.0 { weighted_score / total_weight } else { 0.0 })
    }

    async fn calculate_category_scores(&self, project: &MigrationProject) -> ResearchResult<HashMap<String, f64>> {
        let mut category_scores = HashMap::new();

        for category in &self.benchmark_categories {
            let score = self.calculate_category_score(project, category).await?;
            category_scores.insert(category.name.clone(), score);
        }

        Ok(category_scores)
    }

    async fn calculate_category_score(&self, project: &MigrationProject, category: &BenchmarkCategory) -> ResearchResult<f64> {
        if let Some(tech_details) = &project.technical_details {
            let mut category_score = 0.0;
            let mut metric_count = 0;

            for metric_def in &category.metrics {
                if let Some(performance_metric) = tech_details.performance_metrics.get(&metric_def.name) {
                    let metric_score = self.calculate_metric_score(performance_metric, metric_def);
                    category_score += metric_score;
                    metric_count += 1;
                } else {
                    // Estimate score based on technology stack
                    let estimated_score = self.estimate_metric_score(project, metric_def);
                    category_score += estimated_score;
                    metric_count += 1;
                }
            }

            Ok(if metric_count > 0 { category_score / metric_count as f64 } else { 5.0 })
        } else {
            // No technical details, provide conservative estimate
            Ok(5.0)
        }
    }

    fn calculate_metric_score(&self, metric: &PerformanceMetric, definition: &MetricDefinition) -> f64 {
        if let Some(improvement) = metric.improvement_percentage {
            // Score based on improvement percentage
            if improvement >= definition.improvement_thresholds.dramatic {
                10.0
            } else if improvement >= definition.improvement_thresholds.significant {
                8.0
            } else if improvement >= definition.improvement_thresholds.minimal {
                6.0
            } else if improvement > 0.0 {
                4.0
            } else {
                2.0
            }
        } else if let (Some(after_value), Some(_before_value)) = (metric.after_value, metric.before_value) {
            // Score based on absolute performance vs. baseline expectations
            let (min_acceptable, max_acceptable) = definition.baseline_expectations.acceptable_range;
            
            if after_value <= min_acceptable {
                10.0 // Excellent performance
            } else if after_value <= definition.baseline_expectations.native_typical {
                8.0  // Good native performance
            } else if after_value <= max_acceptable {
                6.0  // Acceptable performance
            } else if after_value <= definition.baseline_expectations.electron_typical {
                4.0  // Typical Electron performance
            } else {
                2.0  // Poor performance
            }
        } else {
            5.0 // No data available
        }
    }

    fn estimate_metric_score(&self, project: &MigrationProject, metric_def: &MetricDefinition) -> f64 {
        if let Some(tech_details) = &project.technical_details {
            match tech_details.target_technology {
                super::TargetTechnology::Rust => {
                    match metric_def.name.as_str() {
                        "Cold Start Time" | "Warm Start Time" => 8.0, // Rust typically fast startup
                        "Initial Memory Usage" | "Working Set Memory" => 8.5, // Rust very memory efficient
                        "Idle CPU Usage" | "Active CPU Usage" => 8.0, // Rust efficient CPU usage
                        "Keystroke Latency" | "File Open Time" => 8.5, // Rust excellent responsiveness
                        "Application Bundle Size" => 7.0, // Rust produces compact binaries
                        _ => 7.5,
                    }
                },
                super::TargetTechnology::Cpp => {
                    match metric_def.name.as_str() {
                        "Cold Start Time" | "Warm Start Time" => 8.5, // C++ very fast startup
                        "Initial Memory Usage" | "Working Set Memory" => 8.0, // C++ memory efficient
                        "Idle CPU Usage" | "Active CPU Usage" => 8.5, // C++ very efficient
                        "Keystroke Latency" | "File Open Time" => 9.0, // C++ excellent performance
                        "Application Bundle Size" => 8.0, // C++ compact binaries
                        _ => 8.0,
                    }
                },
                super::TargetTechnology::Go => {
                    match metric_def.name.as_str() {
                        "Cold Start Time" | "Warm Start Time" => 7.0, // Go decent startup
                        "Initial Memory Usage" | "Working Set Memory" => 6.5, // Go GC overhead
                        "Idle CPU Usage" | "Active CPU Usage" => 7.0, // Go decent efficiency
                        "Keystroke Latency" | "File Open Time" => 7.0, // Go good responsiveness
                        "Application Bundle Size" => 6.0, // Go larger binaries
                        _ => 6.5,
                    }
                },
                super::TargetTechnology::Wasm => {
                    match metric_def.name.as_str() {
                        "Cold Start Time" | "Warm Start Time" => 7.5, // WASM good startup
                        "Initial Memory Usage" | "Working Set Memory" => 7.0, // WASM decent memory
                        "Idle CPU Usage" | "Active CPU Usage" => 7.5, // WASM good efficiency
                        "Keystroke Latency" | "File Open Time" => 7.0, // WASM good responsiveness
                        "Application Bundle Size" => 8.0, // WASM compact
                        _ => 7.0,
                    }
                },
                super::TargetTechnology::Tauri => {
                    match metric_def.name.as_str() {
                        "Cold Start Time" | "Warm Start Time" => 7.5, // Tauri good startup
                        "Initial Memory Usage" | "Working Set Memory" => 8.0, // Tauri memory efficient
                        "Idle CPU Usage" | "Active CPU Usage" => 7.5, // Tauri decent efficiency
                        "Keystroke Latency" | "File Open Time" => 7.5, // Tauri good responsiveness
                        "Application Bundle Size" => 7.0, // Tauri reasonable size
                        _ => 7.5,
                    }
                },
                _ => 6.0, // Conservative estimate for other technologies
            }
        } else {
            5.0 // No technical details available
        }
    }

    async fn identify_optimizations(&self, project: &MigrationProject) -> ResearchResult<Vec<IdentifiedOptimization>> {
        let mut optimizations = Vec::new();

        for pattern in &self.optimization_patterns {
            if self.pattern_applies_to_project(pattern, project) {
                let optimization = IdentifiedOptimization {
                    pattern_name: pattern.name.clone(),
                    category: pattern.category.clone(),
                    description: pattern.description.clone(),
                    measured_improvement: self.get_measured_improvement(pattern, project),
                    implementation_details: pattern.implementation_techniques.join("; "),
                    confidence_level: self.calculate_pattern_confidence(pattern, project),
                    source_evidence: self.gather_pattern_evidence(pattern, project),
                };
                optimizations.push(optimization);
            }
        }

        Ok(optimizations)
    }

    fn pattern_applies_to_project(&self, pattern: &OptimizationPattern, project: &MigrationProject) -> bool {
        // Check if the pattern is observed in similar projects
        if pattern.observed_projects.contains(&project.name) {
            return true;
        }

        // Check technology compatibility
        if let Some(tech_details) = &project.technical_details {
            match pattern.name.as_str() {
                "Native Compilation" => {
                    matches!(tech_details.target_technology, 
                        super::TargetTechnology::Rust | 
                        super::TargetTechnology::Cpp | 
                        super::TargetTechnology::Go)
                },
                "WASM Optimization" => {
                    matches!(tech_details.target_technology, super::TargetTechnology::Wasm)
                },
                "Lazy Loading" => true, // Applicable to most projects
                "Memory Pool Allocation" => {
                    matches!(tech_details.target_technology, 
                        super::TargetTechnology::Rust | 
                        super::TargetTechnology::Cpp)
                },
                "Incremental Rendering" => true, // Applicable to UI-heavy projects
                "Background Processing" => true, // Applicable to most projects
                "Caching Strategies" => true, // Applicable to most projects
                _ => false,
            }
        } else {
            false
        }
    }

    fn get_measured_improvement(&self, pattern: &OptimizationPattern, project: &MigrationProject) -> Option<f64> {
        if let Some(tech_details) = &project.technical_details {
            // Try to find relevant performance metrics
            for (metric_name, metric) in &tech_details.performance_metrics {
                if let Some(improvement) = metric.improvement_percentage {
                    // Match metric to pattern expectations
                    if pattern.expected_improvements.contains_key(metric_name) {
                        return Some(improvement);
                    }
                }
            }
        }
        None
    }

    fn calculate_pattern_confidence(&self, pattern: &OptimizationPattern, project: &MigrationProject) -> f64 {
        let mut confidence = 0.5; // Base confidence

        // Higher confidence if pattern is observed in this project
        if pattern.observed_projects.contains(&project.name) {
            confidence += 0.3;
        }

        // Higher confidence if we have measured data
        if self.get_measured_improvement(pattern, project).is_some() {
            confidence += 0.2;
        }

        // Adjust based on project popularity (proxy for reliability)
        if project.stars > 1000 {
            confidence += 0.1;
        }

        confidence.min(1.0)
    }

    fn gather_pattern_evidence(&self, pattern: &OptimizationPattern, project: &MigrationProject) -> Vec<String> {
        let mut evidence = Vec::new();

        if pattern.observed_projects.contains(&project.name) {
            evidence.push(format!("Pattern observed in {}", project.name));
        }

        if let Some(github_url) = &project.github_url {
            evidence.push(format!("Source code: {}", github_url));
        }

        if let Some(improvement) = self.get_measured_improvement(pattern, project) {
            evidence.push(format!("Measured improvement: {:.1}%", improvement));
        }

        evidence
    }

    async fn extract_performance_patterns(&self, project: &MigrationProject) -> ResearchResult<Vec<PerformancePattern>> {
        let mut patterns = Vec::new();

        if let Some(tech_details) = &project.technical_details {
            // Analyze performance metrics to identify patterns
            for (metric_name, metric) in &tech_details.performance_metrics {
                if let Some(improvement) = metric.improvement_percentage {
                    if improvement > 50.0 {
                        patterns.push(PerformancePattern {
                            pattern_type: "Significant Improvement".to_string(),
                            description: format!("{} shows {:.1}% improvement", metric_name, improvement),
                            metrics_affected: vec![metric_name.clone()],
                            improvement_range: (improvement * 0.8, improvement * 1.2),
                            implementation_complexity: OptimizationComplexity::Medium,
                            prerequisites: vec!["Performance measurement infrastructure".to_string()],
                        });
                    }
                }
            }

            // Technology-specific patterns
            match tech_details.target_technology {
                super::TargetTechnology::Rust => {
                    patterns.push(PerformancePattern {
                        pattern_type: "Rust Native Performance".to_string(),
                        description: "Rust compilation provides systematic performance improvements".to_string(),
                        metrics_affected: vec!["startup_time".to_string(), "memory_usage".to_string(), "cpu_usage".to_string()],
                        improvement_range: (100.0, 300.0),
                        implementation_complexity: OptimizationComplexity::High,
                        prerequisites: vec!["Rust expertise".to_string(), "Rewrite commitment".to_string()],
                    });
                },
                super::TargetTechnology::Wasm => {
                    patterns.push(PerformancePattern {
                        pattern_type: "WASM Optimization".to_string(),
                        description: "WebAssembly provides near-native performance in browser".to_string(),
                        metrics_affected: vec!["cpu_usage".to_string(), "response_time".to_string()],
                        improvement_range: (50.0, 200.0),
                        implementation_complexity: OptimizationComplexity::High,
                        prerequisites: vec!["WASM toolchain".to_string(), "Browser compatibility".to_string()],
                    });
                },
                _ => {},
            }
        }

        Ok(patterns)
    }

    async fn analyze_benchmarking_methodology(&self, project: &MigrationProject) -> ResearchResult<BenchmarkingMethodology> {
        // This would typically analyze project documentation, benchmarking scripts, etc.
        // For now, provide a template based on project characteristics

        let measurement_approach = if project.stars > 5000 {
            "Comprehensive benchmarking with multiple scenarios".to_string()
        } else if project.stars > 1000 {
            "Basic performance measurement".to_string()
        } else {
            "Limited or informal performance testing".to_string()
        };

        let tools_used = match &project.technical_details {
            Some(tech_details) => match tech_details.target_technology {
                super::TargetTechnology::Rust => vec![
                    "Criterion.rs".to_string(),
                    "cargo bench".to_string(),
                    "perf".to_string(),
                    "valgrind".to_string(),
                ],
                super::TargetTechnology::Cpp => vec![
                    "Google Benchmark".to_string(),
                    "perf".to_string(),
                    "Intel VTune".to_string(),
                ],
                super::TargetTechnology::Wasm => vec![
                    "Browser DevTools".to_string(),
                    "wasmtime bench".to_string(),
                    "Performance API".to_string(),
                ],
                _ => vec!["Custom benchmarking".to_string()],
            },
            None => vec!["Unknown".to_string()],
        };

        Ok(BenchmarkingMethodology {
            measurement_approach,
            tools_used,
            test_scenarios: vec![
                "Cold startup".to_string(),
                "Warm startup".to_string(),
                "Large file handling".to_string(),
                "Memory stress test".to_string(),
                "CPU intensive operations".to_string(),
            ],
            reliability_indicators: vec![
                "Multiple measurement runs".to_string(),
                "Statistical significance testing".to_string(),
                "Consistent test environment".to_string(),
            ],
            limitations: vec![
                "Platform-specific results".to_string(),
                "Synthetic benchmark scenarios".to_string(),
                "Limited real-world usage patterns".to_string(),
            ],
        })
    }

    async fn generate_performance_recommendations(&self, project: &MigrationProject, optimizations: &[IdentifiedOptimization]) -> ResearchResult<Vec<PerformanceRecommendation>> {
        let mut recommendations = Vec::new();

        // High-impact, low-effort recommendations
        recommendations.push(PerformanceRecommendation {
            category: "Quick Wins".to_string(),
            recommendation: "Implement lazy loading for non-critical components".to_string(),
            expected_impact: "20-50% startup time improvement".to_string(),
            implementation_effort: "Medium".to_string(),
            priority: RecommendationPriority::High,
            supporting_evidence: vec![
                "Observed in multiple successful projects".to_string(),
                "Low risk, high reward optimization".to_string(),
            ],
        });

        // Technology-specific recommendations
        if let Some(tech_details) = &project.technical_details {
            match tech_details.target_technology {
                super::TargetTechnology::Rust => {
                    recommendations.push(PerformanceRecommendation {
                        category: "Rust Optimization".to_string(),
                        recommendation: "Use profile-guided optimization (PGO) for release builds".to_string(),
                        expected_impact: "10-30% performance improvement".to_string(),
                        implementation_effort: "Low".to_string(),
                        priority: RecommendationPriority::Medium,
                        supporting_evidence: vec![
                            "Standard Rust optimization technique".to_string(),
                            "Minimal code changes required".to_string(),
                        ],
                    });
                },
                super::TargetTechnology::Wasm => {
                    recommendations.push(PerformanceRecommendation {
                        category: "WASM Optimization".to_string(),
                        recommendation: "Minimize JS-WASM boundary crossings".to_string(),
                        expected_impact: "Significant responsiveness improvement".to_string(),
                        implementation_effort: "High".to_string(),
                        priority: RecommendationPriority::High,
                        supporting_evidence: vec![
                            "Critical for WASM performance".to_string(),
                            "Observed in high-performance WASM applications".to_string(),
                        ],
                    });
                },
                _ => {},
            }
        }

        // Recommendations based on identified optimizations
        for optimization in optimizations {
            if optimization.confidence_level > 0.7 {
                let priority = match optimization.category.as_str() {
                    "Architecture" => RecommendationPriority::Critical,
                    "UI Performance" => RecommendationPriority::High,
                    "Memory Management" => RecommendationPriority::High,
                    _ => RecommendationPriority::Medium,
                };

                recommendations.push(PerformanceRecommendation {
                    category: optimization.category.clone(),
                    recommendation: format!("Implement {}", optimization.pattern_name),
                    expected_impact: optimization.measured_improvement
                        .map(|imp| format!("{:.1}% improvement", imp))
                        .unwrap_or_else(|| "Significant improvement expected".to_string()),
                    implementation_effort: match optimization.pattern_name.as_str() {
                        "Native Compilation" => "Very High".to_string(),
                        "WASM Optimization" => "High".to_string(),
                        "Memory Pool Allocation" => "High".to_string(),
                        "Background Processing" => "High".to_string(),
                        "Incremental Rendering" => "Medium".to_string(),
                        "Lazy Loading" => "Medium".to_string(),
                        "Caching Strategies" => "Medium".to_string(),
                        _ => "Medium".to_string(),
                    },
                    priority,
                    supporting_evidence: optimization.source_evidence.clone(),
                });
            }
        }

        Ok(recommendations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_analyzer_initialization() {
        let analyzer = PerformanceAnalyzer::new();
        assert!(!analyzer.benchmark_categories.is_empty());
        assert!(!analyzer.optimization_patterns.is_empty());
    }

    #[test]
    fn test_benchmark_categories() {
        let analyzer = PerformanceAnalyzer::new();
        let categories: Vec<&str> = analyzer.benchmark_categories.iter()
            .map(|c| c.name.as_str())
            .collect();
        
        assert!(categories.contains(&"Startup Performance"));
        assert!(categories.contains(&"Memory Usage"));
        assert!(categories.contains(&"CPU Usage"));
        assert!(categories.contains(&"Response Time"));
        assert!(categories.contains(&"Bundle Size"));
    }

    #[test]
    fn test_optimization_patterns() {
        let analyzer = PerformanceAnalyzer::new();
        let patterns: Vec<&str> = analyzer.optimization_patterns.iter()
            .map(|p| p.name.as_str())
            .collect();
        
        assert!(patterns.contains(&"Native Compilation"));
        assert!(patterns.contains(&"Lazy Loading"));
        assert!(patterns.contains(&"Memory Pool Allocation"));
        assert!(patterns.contains(&"WASM Optimization"));
    }

    #[test]
    fn test_metric_score_calculation() {
        let analyzer = PerformanceAnalyzer::new();
        
        let metric = PerformanceMetric {
            before_value: Some(1000.0),
            after_value: Some(200.0),
            unit: "ms".to_string(),
            improvement_percentage: Some(400.0), // 5x improvement
            measurement_context: None,
            source_url: None,
            verified: true,
        };

        let definition = MetricDefinition {
            name: "Test Metric".to_string(),
            unit: "ms".to_string(),
            measurement_method: "Test".to_string(),
            baseline_expectations: BaselineExpectations {
                electron_typical: 1000.0,
                native_typical: 200.0,
                acceptable_range: (100.0, 800.0),
            },
            improvement_thresholds: ImprovementThresholds {
                minimal: 20.0,
                significant: 50.0,
                dramatic: 200.0,
            },
        };

        let score = analyzer.calculate_metric_score(&metric, &definition);
        assert_eq!(score, 10.0); // Should be maximum score for dramatic improvement
    }
}