// Performance Optimization Patterns Extraction System
// Extracts and analyzes proven optimization techniques from migration projects

use super::{MigrationProject, ResearchResult, ResearchError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationPatternExtractor {
    pub pattern_library: Vec<OptimizationPattern>,
    pub technique_catalog: Vec<OptimizationTechnique>,
    pub benchmarking_methodologies: Vec<BenchmarkingMethodology>,
    pub success_metrics: Vec<SuccessMetric>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationPattern {
    pub pattern_id: String,
    pub name: String,
    pub category: OptimizationCategory,
    pub description: String,
    pub implementation_techniques: Vec<String>,
    pub expected_improvements: HashMap<String, ImprovementRange>,
    pub complexity_level: ComplexityLevel,
    pub prerequisites: Vec<String>,
    pub observed_projects: Vec<String>,
    pub success_rate: f64,
    pub kiro_applicability: ApplicabilityLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationCategory {
    StartupPerformance,
    MemoryManagement,
    CpuEfficiency,
    IoOptimization,
    RenderingPerformance,
    ConcurrencyPatterns,
    CachingStrategies,
    NetworkOptimization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImprovementRange {
    pub min_improvement: f64,
    pub typical_improvement: f64,
    pub max_improvement: f64,
    pub unit: String,
    pub measurement_context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplexityLevel {
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApplicabilityLevel {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationTechnique {
    pub technique_id: String,
    pub name: String,
    pub description: String,
    pub implementation_steps: Vec<ImplementationStep>,
    pub code_examples: Vec<CodeExample>,
    pub performance_impact: PerformanceImpact,
    pub trade_offs: TradeOffs,
    pub validation_methods: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationStep {
    pub step_number: i32,
    pub description: String,
    pub technical_details: String,
    pub estimated_effort: String,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExample {
    pub language: String,
    pub description: String,
    pub code_snippet: String,
    pub performance_notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceImpact {
    pub primary_metrics: Vec<String>,
    pub typical_improvement: f64,
    pub measurement_method: String,
    pub validation_projects: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeOffs {
    pub benefits: Vec<String>,
    pub costs: Vec<String>,
    pub risks: Vec<String>,
    pub mitigation_strategies: Vec<String>,
}

impl OptimizationPatternExtractor {
    pub fn new() -> Self {
        Self {
            pattern_library: Self::build_pattern_library(),
            technique_catalog: Self::build_technique_catalog(),
            benchmarking_methodologies: Self::build_benchmarking_methodologies(),
            success_metrics: Self::build_success_metrics(),
        }
    }

    fn build_pattern_library() -> Vec<OptimizationPattern> {
        vec![
            // Native Compilation Pattern
            OptimizationPattern {
                pattern_id: "native_compilation".to_string(),
                name: "Native Compilation Optimization".to_string(),
                category: OptimizationCategory::CpuEfficiency,
                description: "Compile application logic to native machine code for maximum performance".to_string(),
                implementation_techniques: vec![
                    "Rust compilation with LLVM optimizations".to_string(),
                    "Link-time optimization (LTO)".to_string(),
                    "Profile-guided optimization (PGO)".to_string(),
                    "Target-specific CPU optimizations".to_string(),
                ],
                expected_improvements: HashMap::from([
                    ("cpu_performance".to_string(), ImprovementRange {
                        min_improvement: 200.0,
                        typical_improvement: 500.0,
                        max_improvement: 1000.0,
                        unit: "percentage".to_string(),
                        measurement_context: "vs interpreted/JIT execution".to_string(),
                    }),
                    ("startup_time".to_string(), ImprovementRange {
                        min_improvement: 300.0,
                        typical_improvement: 600.0,
                        max_improvement: 2000.0,
                        unit: "percentage".to_string(),
                        measurement_context: "vs Electron startup".to_string(),
                    }),
                ]),
                complexity_level: ComplexityLevel::High,
                prerequisites: vec![
                    "Rust development expertise".to_string(),
                    "Performance profiling capabilities".to_string(),
                    "Comprehensive test suite".to_string(),
                ],
                observed_projects: vec!["Zed".to_string(), "Lapce".to_string(), "Helix".to_string()],
                success_rate: 0.85,
                kiro_applicability: ApplicabilityLevel::High,
            },
        ]
    }
}    
        // Memory Pool Allocation Pattern
            OptimizationPattern {
                pattern_id: "memory_pooling".to_string(),
                name: "Memory Pool Allocation".to_string(),
                category: OptimizationCategory::MemoryManagement,
                description: "Use pre-allocated memory pools to reduce allocation overhead and improve cache locality".to_string(),
                implementation_techniques: vec![
                    "Object pooling for frequently created objects".to_string(),
                    "Arena allocation for temporary objects".to_string(),
                    "Custom allocators for specific use cases".to_string(),
                    "Memory mapping for large files".to_string(),
                ],
                expected_improvements: HashMap::from([
                    ("memory_allocation_speed".to_string(), ImprovementRange {
                        min_improvement: 300.0,
                        typical_improvement: 800.0,
                        max_improvement: 2000.0,
                        unit: "percentage".to_string(),
                        measurement_context: "allocation/deallocation cycles".to_string(),
                    }),
                    ("memory_fragmentation".to_string(), ImprovementRange {
                        min_improvement: 50.0,
                        typical_improvement: 80.0,
                        max_improvement: 95.0,
                        unit: "percentage_reduction".to_string(),
                        measurement_context: "heap fragmentation".to_string(),
                    }),
                ]),
                complexity_level: ComplexityLevel::High,
                prerequisites: vec![
                    "Memory usage profiling".to_string(),
                    "Understanding of allocation patterns".to_string(),
                    "Custom allocator implementation skills".to_string(),
                ],
                observed_projects: vec!["Xi Editor".to_string(), "Zed".to_string()],
                success_rate: 0.75,
                kiro_applicability: ApplicabilityLevel::High,
            },

            // Incremental Rendering Pattern
            OptimizationPattern {
                pattern_id: "incremental_rendering".to_string(),
                name: "Incremental Rendering Optimization".to_string(),
                category: OptimizationCategory::RenderingPerformance,
                description: "Only re-render changed portions of the UI to minimize rendering overhead".to_string(),
                implementation_techniques: vec![
                    "Dirty region tracking".to_string(),
                    "Virtual DOM with efficient diffing".to_string(),
                    "Component-level memoization".to_string(),
                    "Viewport-based rendering".to_string(),
                ],
                expected_improvements: HashMap::from([
                    ("rendering_performance".to_string(), ImprovementRange {
                        min_improvement: 200.0,
                        typical_improvement: 500.0,
                        max_improvement: 1500.0,
                        unit: "percentage".to_string(),
                        measurement_context: "frame rendering time".to_string(),
                    }),
                    ("keystroke_latency".to_string(), ImprovementRange {
                        min_improvement: 300.0,
                        typical_improvement: 800.0,
                        max_improvement: 2000.0,
                        unit: "percentage".to_string(),
                        measurement_context: "keypress to display".to_string(),
                    }),
                ]),
                complexity_level: ComplexityLevel::Medium,
                prerequisites: vec![
                    "UI architecture understanding".to_string(),
                    "Rendering pipeline knowledge".to_string(),
                    "Performance measurement tools".to_string(),
                ],
                observed_projects: vec!["Zed".to_string(), "Lapce".to_string(), "Fleet".to_string()],
                success_rate: 0.90,
                kiro_applicability: ApplicabilityLevel::High,
            },

            // Lazy Loading Pattern
            OptimizationPattern {
                pattern_id: "lazy_loading".to_string(),
                name: "Lazy Loading and Code Splitting".to_string(),
                category: OptimizationCategory::StartupPerformance,
                description: "Load resources and modules only when needed to improve startup time".to_string(),
                implementation_techniques: vec![
                    "Dynamic module loading".to_string(),
                    "On-demand asset loading".to_string(),
                    "Lazy initialization of components".to_string(),
                    "Code splitting and tree shaking".to_string(),
                ],
                expected_improvements: HashMap::from([
                    ("startup_time".to_string(), ImprovementRange {
                        min_improvement: 150.0,
                        typical_improvement: 300.0,
                        max_improvement: 800.0,
                        unit: "percentage".to_string(),
                        measurement_context: "cold startup time".to_string(),
                    }),
                    ("initial_memory_usage".to_string(), ImprovementRange {
                        min_improvement: 30.0,
                        typical_improvement: 60.0,
                        max_improvement: 80.0,
                        unit: "percentage_reduction".to_string(),
                        measurement_context: "initial memory footprint".to_string(),
                    }),
                ]),
                complexity_level: ComplexityLevel::Medium,
                prerequisites: vec![
                    "Modular architecture".to_string(),
                    "Dependency analysis capabilities".to_string(),
                    "Build system optimization".to_string(),
                ],
                observed_projects: vec!["VS Code".to_string(), "Theia".to_string(), "Cursor".to_string()],
                success_rate: 0.95,
                kiro_applicability: ApplicabilityLevel::High,
            },

            // Background Processing Pattern
            OptimizationPattern {
                pattern_id: "background_processing".to_string(),
                name: "Background Processing and Concurrency".to_string(),
                category: OptimizationCategory::ConcurrencyPatterns,
                description: "Move heavy computations to background threads to maintain UI responsiveness".to_string(),
                implementation_techniques: vec![
                    "Worker threads for CPU-intensive tasks".to_string(),
                    "Async/await for I/O operations".to_string(),
                    "Message passing between threads".to_string(),
                    "Lock-free data structures".to_string(),
                ],
                expected_improvements: HashMap::from([
                    ("ui_responsiveness".to_string(), ImprovementRange {
                        min_improvement: 500.0,
                        typical_improvement: 1000.0,
                        max_improvement: 5000.0,
                        unit: "percentage".to_string(),
                        measurement_context: "UI thread blocking reduction".to_string(),
                    }),
                    ("throughput".to_string(), ImprovementRange {
                        min_improvement: 200.0,
                        typical_improvement: 400.0,
                        max_improvement: 800.0,
                        unit: "percentage".to_string(),
                        measurement_context: "overall system throughput".to_string(),
                    }),
                ]),
                complexity_level: ComplexityLevel::High,
                prerequisites: vec![
                    "Concurrency programming expertise".to_string(),
                    "Thread-safe architecture design".to_string(),
                    "Performance profiling tools".to_string(),
                ],
                observed_projects: vec!["Rust Analyzer".to_string(), "Zed".to_string(), "Helix".to_string()],
                success_rate: 0.80,
                kiro_applicability: ApplicabilityLevel::High,
            },

            // Caching Strategies Pattern
            OptimizationPattern {
                pattern_id: "intelligent_caching".to_string(),
                name: "Intelligent Caching Strategies".to_string(),
                category: OptimizationCategory::CachingStrategies,
                description: "Cache frequently accessed data and computations with intelligent eviction policies".to_string(),
                implementation_techniques: vec![
                    "In-memory caching of parsed files".to_string(),
                    "Persistent caching across sessions".to_string(),
                    "Incremental computation caching".to_string(),
                    "LRU and adaptive eviction policies".to_string(),
                ],
                expected_improvements: HashMap::from([
                    ("file_open_time".to_string(), ImprovementRange {
                        min_improvement: 300.0,
                        typical_improvement: 800.0,
                        max_improvement: 2000.0,
                        unit: "percentage".to_string(),
                        measurement_context: "subsequent file access".to_string(),
                    }),
                    ("computation_time".to_string(), ImprovementRange {
                        min_improvement: 500.0,
                        typical_improvement: 1500.0,
                        max_improvement: 5000.0,
                        unit: "percentage".to_string(),
                        measurement_context: "repeated computations".to_string(),
                    }),
                ]),
                complexity_level: ComplexityLevel::Medium,
                prerequisites: vec![
                    "Cache invalidation strategy".to_string(),
                    "Memory management understanding".to_string(),
                    "Access pattern analysis".to_string(),
                ],
                observed_projects: vec!["Language servers".to_string(), "VS Code".to_string(), "IntelliJ".to_string()],
                success_rate: 0.85,
                kiro_applicability: ApplicabilityLevel::High,
            },

            // WASM Optimization Pattern
            OptimizationPattern {
                pattern_id: "wasm_optimization".to_string(),
                name: "WebAssembly Performance Optimization".to_string(),
                category: OptimizationCategory::CpuEfficiency,
                description: "Optimize WebAssembly modules for maximum performance in browser environments".to_string(),
                implementation_techniques: vec![
                    "SIMD instructions for parallel processing".to_string(),
                    "Memory layout optimization".to_string(),
                    "Minimal JS-WASM boundary crossings".to_string(),
                    "Bulk memory operations".to_string(),
                ],
                expected_improvements: HashMap::from([
                    ("computation_speed".to_string(), ImprovementRange {
                        min_improvement: 200.0,
                        typical_improvement: 500.0,
                        max_improvement: 1000.0,
                        unit: "percentage".to_string(),
                        measurement_context: "vs JavaScript implementation".to_string(),
                    }),
                    ("memory_efficiency".to_string(), ImprovementRange {
                        min_improvement: 30.0,
                        typical_improvement: 60.0,
                        max_improvement: 80.0,
                        unit: "percentage_reduction".to_string(),
                        measurement_context: "memory usage vs JS".to_string(),
                    }),
                ]),
                complexity_level: ComplexityLevel::High,
                prerequisites: vec![
                    "WASM toolchain expertise".to_string(),
                    "Browser performance profiling".to_string(),
                    "Low-level optimization knowledge".to_string(),
                ],
                observed_projects: vec!["Figma".to_string(), "AutoCAD Web".to_string(), "Photoshop Web".to_string()],
                success_rate: 0.70,
                kiro_applicability: ApplicabilityLevel::High,
            },
        ]
    }

    fn build_technique_catalog() -> Vec<OptimizationTechnique> {
        vec![
            OptimizationTechnique {
                technique_id: "rust_lto".to_string(),
                name: "Rust Link-Time Optimization".to_string(),
                description: "Enable LTO for cross-crate optimizations and dead code elimination".to_string(),
                implementation_steps: vec![
                    ImplementationStep {
                        step_number: 1,
                        description: "Configure Cargo.toml for LTO".to_string(),
                        technical_details: "Add lto = true to [profile.release] section".to_string(),
                        estimated_effort: "1 hour".to_string(),
                        dependencies: vec!["Rust toolchain".to_string()],
                    },
                    ImplementationStep {
                        step_number: 2,
                        description: "Measure performance impact".to_string(),
                        technical_details: "Benchmark before/after with criterion.rs".to_string(),
                        estimated_effort: "4 hours".to_string(),
                        dependencies: vec!["Benchmarking framework".to_string()],
                    },
                ],
                code_examples: vec![
                    CodeExample {
                        language: "toml".to_string(),
                        description: "Cargo.toml configuration for LTO".to_string(),
                        code_snippet: r#"[profile.release]
lto = true
codegen-units = 1
panic = "abort""#.to_string(),
                        performance_notes: "Typically provides 10-30% performance improvement".to_string(),
                    },
                ],
                performance_impact: PerformanceImpact {
                    primary_metrics: vec!["execution_speed".to_string(), "binary_size".to_string()],
                    typical_improvement: 20.0,
                    measurement_method: "Benchmark suite comparison".to_string(),
                    validation_projects: vec!["Zed".to_string(), "Lapce".to_string()],
                },
                trade_offs: TradeOffs {
                    benefits: vec![
                        "Improved runtime performance".to_string(),
                        "Smaller binary size".to_string(),
                        "Better optimization across crate boundaries".to_string(),
                    ],
                    costs: vec![
                        "Longer compilation time".to_string(),
                        "Increased memory usage during compilation".to_string(),
                    ],
                    risks: vec![
                        "Potential compilation failures with complex dependencies".to_string(),
                    ],
                    mitigation_strategies: vec![
                        "Use LTO only for release builds".to_string(),
                        "Incremental compilation for development".to_string(),
                    ],
                },
                validation_methods: vec![
                    "Performance benchmarking".to_string(),
                    "Binary size measurement".to_string(),
                    "Compilation time tracking".to_string(),
                ],
            },
        ]
    }

    fn build_benchmarking_methodologies() -> Vec<BenchmarkingMethodology> {
        vec![
            BenchmarkingMethodology {
                methodology_id: "startup_benchmarking".to_string(),
                name: "Startup Performance Benchmarking".to_string(),
                description: "Comprehensive methodology for measuring application startup performance".to_string(),
                measurement_phases: vec![
                    MeasurementPhase {
                        phase_name: "Cold Start".to_string(),
                        description: "First application launch after system restart".to_string(),
                        measurement_points: vec![
                            "Process creation time".to_string(),
                            "Initial memory allocation".to_string(),
                            "UI first paint".to_string(),
                            "Application ready state".to_string(),
                        ],
                        tools_required: vec!["System profiler".to_string(), "Custom timing hooks".to_string()],
                    },
                    MeasurementPhase {
                        phase_name: "Warm Start".to_string(),
                        description: "Subsequent application launches with OS caching".to_string(),
                        measurement_points: vec![
                            "Process creation time".to_string(),
                            "Cache utilization metrics".to_string(),
                            "UI ready time".to_string(),
                        ],
                        tools_required: vec!["Performance monitoring".to_string()],
                    },
                ],
                success_criteria: vec![
                    SuccessCriterion {
                        metric_name: "Cold start time".to_string(),
                        target_value: "< 500ms".to_string(),
                        measurement_method: "Average of 10 measurements".to_string(),
                        baseline_comparison: "vs Electron baseline".to_string(),
                    },
                ],
                validation_requirements: vec![
                    "Multiple platform testing".to_string(),
                    "Various hardware configurations".to_string(),
                    "Statistical significance validation".to_string(),
                ],
            },
        ]
    }

    fn build_success_metrics() -> Vec<SuccessMetric> {
        vec![
            SuccessMetric {
                metric_id: "startup_performance".to_string(),
                name: "Startup Performance".to_string(),
                description: "Time from application launch to ready state".to_string(),
                measurement_unit: "milliseconds".to_string(),
                target_improvement: 300.0, // 3x improvement
                baseline_value: Some(2500.0), // Typical Electron startup
                measurement_frequency: "Daily during development".to_string(),
                validation_method: "Automated benchmarking suite".to_string(),
                kiro_importance: ImportanceLevel::Critical,
            },
            SuccessMetric {
                metric_id: "memory_efficiency".to_string(),
                name: "Memory Usage Efficiency".to_string(),
                description: "Initial and working set memory consumption".to_string(),
                measurement_unit: "MB".to_string(),
                target_improvement: 200.0, // 2x improvement (50% reduction)
                baseline_value: Some(200.0), // Typical Electron memory
                measurement_frequency: "Daily during development".to_string(),
                validation_method: "Memory profiling tools".to_string(),
                kiro_importance: ImportanceLevel::High,
            },
        ]
    }

    pub fn extract_patterns_from_project(&self, project: &MigrationProject) -> ResearchResult<Vec<ExtractedPattern>> {
        let mut extracted_patterns = Vec::new();

        // Analyze project for applicable patterns
        for pattern in &self.pattern_library {
            if self.pattern_applies_to_project(pattern, project) {
                let extracted = ExtractedPattern {
                    pattern: pattern.clone(),
                    evidence: self.gather_evidence(pattern, project),
                    confidence_level: self.calculate_confidence(pattern, project),
                    implementation_notes: self.generate_implementation_notes(pattern, project),
                    kiro_adaptation: self.suggest_kiro_adaptation(pattern, project),
                };
                extracted_patterns.push(extracted);
            }
        }

        Ok(extracted_patterns)
    }

    fn pattern_applies_to_project(&self, pattern: &OptimizationPattern, project: &MigrationProject) -> bool {
        // Check if pattern is observed in this project
        if pattern.observed_projects.contains(&project.name) {
            return true;
        }

        // Check technology compatibility
        if let Some(tech_details) = &project.technical_details {
            match pattern.pattern_id.as_str() {
                "native_compilation" => {
                    matches!(tech_details.target_technology, 
                        super::TargetTechnology::Rust | 
                        super::TargetTechnology::Cpp)
                },
                "wasm_optimization" => {
                    matches!(tech_details.target_technology, super::TargetTechnology::Wasm)
                },
                "memory_pooling" => {
                    matches!(tech_details.target_technology, 
                        super::TargetTechnology::Rust | 
                        super::TargetTechnology::Cpp)
                },
                _ => true, // Most patterns are generally applicable
            }
        } else {
            false
        }
    }

    fn gather_evidence(&self, pattern: &OptimizationPattern, project: &MigrationProject) -> Vec<String> {
        let mut evidence = Vec::new();

        if pattern.observed_projects.contains(&project.name) {
            evidence.push(format!("Pattern directly observed in {}", project.name));
        }

        if let Some(tech_details) = &project.technical_details {
            for (metric_name, metric) in &tech_details.performance_metrics {
                if let Some(improvement) = metric.improvement_percentage {
                    if let Some(expected_range) = pattern.expected_improvements.get(metric_name) {
                        if improvement >= expected_range.min_improvement {
                            evidence.push(format!("{}: {:.1}% improvement (expected: {:.1}%+)", 
                                                metric_name, improvement, expected_range.min_improvement));
                        }
                    }
                }
            }
        }

        evidence
    }

    fn calculate_confidence(&self, pattern: &OptimizationPattern, project: &MigrationProject) -> f64 {
        let mut confidence = 0.5; // Base confidence

        if pattern.observed_projects.contains(&project.name) {
            confidence += 0.3;
        }

        if project.stars > 1000 {
            confidence += 0.1; // Popular projects are more reliable
        }

        if let Some(tech_details) = &project.technical_details {
            if !tech_details.performance_metrics.is_empty() {
                confidence += 0.1; // Has performance data
            }
        }

        confidence.min(1.0)
    }

    fn generate_implementation_notes(&self, pattern: &OptimizationPattern, project: &MigrationProject) -> Vec<String> {
        let mut notes = Vec::new();

        notes.push(format!("Complexity level: {:?}", pattern.complexity_level));
        notes.push(format!("Success rate in similar projects: {:.1}%", pattern.success_rate * 100.0));

        if let Some(tech_details) = &project.technical_details {
            notes.push(format!("Target technology: {:?}", tech_details.target_technology));
            if let Some(timeline) = tech_details.migration_timeline_months {
                notes.push(format!("Implementation timeline: {} months", timeline));
            }
        }

        notes
    }

    fn suggest_kiro_adaptation(&self, pattern: &OptimizationPattern, _project: &MigrationProject) -> Vec<String> {
        let mut adaptations = Vec::new();

        match pattern.pattern_id.as_str() {
            "native_compilation" => {
                adaptations.extend(vec![
                    "Focus on AI processing performance optimization".to_string(),
                    "Prioritize extension system performance".to_string(),
                    "Optimize large file handling capabilities".to_string(),
                ]);
            },
            "incremental_rendering" => {
                adaptations.extend(vec![
                    "Implement for code editor rendering".to_string(),
                    "Optimize AI chat interface updates".to_string(),
                    "Enhance file tree rendering performance".to_string(),
                ]);
            },
            "lazy_loading" => {
                adaptations.extend(vec![
                    "Lazy load extension modules".to_string(),
                    "On-demand AI model loading".to_string(),
                    "Progressive UI component initialization".to_string(),
                ]);
            },
            _ => {
                adaptations.push("Apply pattern with Kiro-specific considerations".to_string());
            },
        }

        adaptations
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkingMethodology {
    pub methodology_id: String,
    pub name: String,
    pub description: String,
    pub measurement_phases: Vec<MeasurementPhase>,
    pub success_criteria: Vec<SuccessCriterion>,
    pub validation_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasurementPhase {
    pub phase_name: String,
    pub description: String,
    pub measurement_points: Vec<String>,
    pub tools_required: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessCriterion {
    pub metric_name: String,
    pub target_value: String,
    pub measurement_method: String,
    pub baseline_comparison: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessMetric {
    pub metric_id: String,
    pub name: String,
    pub description: String,
    pub measurement_unit: String,
    pub target_improvement: f64,
    pub baseline_value: Option<f64>,
    pub measurement_frequency: String,
    pub validation_method: String,
    pub kiro_importance: ImportanceLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImportanceLevel {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedPattern {
    pub pattern: OptimizationPattern,
    pub evidence: Vec<String>,
    pub confidence_level: f64,
    pub implementation_notes: Vec<String>,
    pub kiro_adaptation: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_extractor_initialization() {
        let extractor = OptimizationPatternExtractor::new();
        assert!(!extractor.pattern_library.is_empty());
        assert!(!extractor.technique_catalog.is_empty());
    }

    #[test]
    fn test_pattern_extraction() {
        let extractor = OptimizationPatternExtractor::new();
        
        // Create a sample Rust project
        let project = create_sample_rust_project();
        let patterns = extractor.extract_patterns_from_project(&project).unwrap();
        
        assert!(!patterns.is_empty());
        
        // Should find native compilation pattern for Rust projects
        let has_native_compilation = patterns.iter()
            .any(|p| p.pattern.pattern_id == "native_compilation");
        assert!(has_native_compilation);
    }

    fn create_sample_rust_project() -> MigrationProject {
        use std::collections::HashMap;
        
        MigrationProject {
            id: None,
            name: "Test Rust IDE".to_string(),
            url: "https://example.com".to_string(),
            github_url: None,
            category: super::ProjectCategory::RustIde,
            status: super::ProjectStatus::Active,
            last_updated: None,
            stars: 5000,
            contributors: 50,
            technical_details: Some(super::TechnicalDetails {
                source_technology: super::SourceTechnology::Electron,
                target_technology: super::TargetTechnology::Rust,
                migration_approach: super::MigrationApproach::CompleteRewrite,
                architecture_patterns: vec![],
                performance_metrics: HashMap::new(),
                migration_timeline_months: None,
                team_size: None,
            }),
            analysis: None,
        }
    }
}