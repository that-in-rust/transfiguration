# Static Analysis Phase 2 - Research Data Management System

A comprehensive PostgreSQL-based research data management system for systematic analysis of IDE transfiguration projects, competitive landscape analysis, and strategic pattern identification.

## Features

### 🔍 Project Discovery & Cataloging
- **Automated GitHub API Integration**: Discover repositories using advanced search queries
- **Web Scraping Tools**: Extract projects from awesome lists and documentation sites
- **Keyword-Based Search**: Intelligent filtering for relevant project identification
- **Metadata Extraction**: Comprehensive project data standardization

### 📊 Systematic Project Evaluation
- **Standardized Evaluation Criteria**: Objective comparison across multiple dimensions
- **Scoring Algorithms**: Technical quality, adoption, and sustainability metrics
- **Confidence Level Calculation**: Reliability assessment for research findings
- **Bias Detection**: Automated identification and mitigation of research biases

### ✅ Research Data Validation & Quality Assurance
- **Multi-Source Verification**: Cross-validation of claims and findings
- **Peer Review Workflow**: Structured documentation and review processes
- **Completeness Validation**: Coverage assessment and gap identification
- **Accuracy Cross-Validation**: Source reliability scoring and verification

## Architecture

### Database Schema (PostgreSQL)
- **Projects**: Main research subjects with comprehensive metadata
- **Technical Analyses**: Detailed technical evaluations and insights
- **Research Sources**: Attribution and verification tracking
- **Research Findings**: Synthesized insights and patterns
- **Competitive Analyses**: Market positioning and differentiation data
- **Validation Results**: Quality assurance and bias detection results

### Core Components
- **Discovery Engine**: Automated project identification and cataloging
- **Project Evaluator**: Systematic evaluation using standardized criteria
- **Validation Framework**: Comprehensive data quality assurance
- **Bias Detector**: Research bias identification and mitigation
- **Source Verifier**: URL and content verification system

## Quick Start

### Prerequisites
- Rust 1.75+
- PostgreSQL 15+
- Docker & Docker Compose (optional)

### Using Docker Compose (Recommended)

```bash
# Clone the repository
git clone <repository-url>
cd static-analysis-phase2

# Set environment variables
export GITHUB_TOKEN=your_github_token_here

# Start the system
docker-compose up -d

# View logs
docker-compose logs -f research_app
```

### Manual Setup

```bash
# Install dependencies
cargo build --release

# Set up PostgreSQL database
createdb research_db
export DATABASE_URL=postgresql://localhost/research_db

# Optional: Set GitHub token for enhanced discovery
export GITHUB_TOKEN=your_github_token_here

# Run the application
cargo run
```

## Usage

### Running Comprehensive Discovery

The system automatically discovers projects across all categories:

- **IDE Migration Projects**: Electron to native migrations
- **VS Code Forks**: Significant VS Code customizations
- **Rust IDEs**: Native Rust development environments
- **AI IDEs**: AI-powered development tools
- **WASM Applications**: WebAssembly-based development tools

### Database Operations

```rust
use static_analysis_phase2::{Database, models::*};

// Connect to database
let db = Database::new("postgresql://localhost/research_db").await?;

// Search projects
let projects = db.search_projects("rust ide", Some(ProjectCategory::RustIde)).await?;

// Get research progress
let progress = db.get_research_progress().await?;

// Get statistics
let stats = db.get_statistics().await?;
```

### Validation Framework

```rust
use static_analysis_phase2::validation::ValidationFramework;

let validator = ValidationFramework::new();

// Validate entity data
let results = validator.validate_entity("project", entity_id, &data).await?;

// Cross-validate findings
let cross_validation = validator.cross_validate_findings(&findings, &sources).await?;
```

## Research Methodology

### Systematic Project Analysis Framework
1. **Project Identification**: Comprehensive search and cataloging
2. **Standardized Evaluation**: Consistent criteria across all projects
3. **Technical Deep-Dive**: Architecture, performance, and implementation analysis
4. **Outcome Assessment**: Success metrics, challenges, and lessons learned
5. **Pattern Recognition**: Cross-project pattern identification and synthesis

### Data Collection Standards
- **Source Verification**: All claims backed by verifiable sources
- **Bias Mitigation**: Multiple perspectives and objective evaluation criteria
- **Temporal Context**: Understanding projects within their historical context
- **Quantitative Metrics**: Performance data, adoption metrics, and timeline data
- **Qualitative Insights**: User feedback, developer experiences, and community sentiment

### Quality Assurance Framework
- **Completeness Validation**: Comprehensive coverage of research scope
- **Accuracy Verification**: Cross-validation of technical claims and performance data
- **Bias Detection**: Systematic identification and mitigation of research bias
- **Source Reliability**: Assessment of source credibility and verification status

## Configuration

### Environment Variables
- `DATABASE_URL`: PostgreSQL connection string
- `GITHUB_TOKEN`: GitHub API token for enhanced discovery (optional)
- `RUST_LOG`: Logging level (info, debug, warn, error)

### Research Categories Configuration
Each category has specific evaluation criteria:

- **IDE Migration**: Focus on migration strategies and performance improvements
- **VS Code Fork**: Emphasis on customization patterns and extension compatibility
- **Rust IDE**: Native performance and Rust-specific tooling integration
- **AI IDE**: AI integration patterns and user experience design
- **WASM App**: WebAssembly performance and JavaScript interop patterns

## Data Export & Sharing

### PostgreSQL Dump Export
```bash
# Export research data
pg_dump research_db > research_data_backup.sql

# Import on another system
psql research_db < research_data_backup.sql
```

### JSON Export (for sharing)
```bash
# Export projects to JSON
psql research_db -c "COPY (SELECT row_to_json(projects) FROM projects) TO '/tmp/projects.json'"
```

## API Reference

### Database Operations
- `create_project(project)`: Add new project to database
- `get_project(id)`: Retrieve project by ID
- `search_projects(query, category)`: Full-text search with filtering
- `update_project_status(id, status)`: Update project analysis status
- `get_research_progress()`: Get completion status by category

### Discovery Operations
- `discover_projects(category, keywords)`: Discover projects for specific category
- `run_comprehensive_discovery()`: Discover across all categories
- `discover_ide_migrations()`: Find IDE migration projects
- `discover_vscode_forks()`: Find VS Code forks
- `discover_rust_ides()`: Find Rust-based IDEs

### Validation Operations
- `validate_entity(type, id, data)`: Comprehensive entity validation
- `validate_research_coverage(category, target, projects)`: Coverage assessment
- `cross_validate_findings(findings, sources)`: Multi-source verification
- `detect_bias(type, id, data)`: Bias detection and analysis

## Contributing

1. Fork the repository
2. Create a feature branch
3. Implement changes with tests
4. Run validation: `cargo test`
5. Submit pull request

### Development Setup
```bash
# Install development dependencies
cargo install sqlx-cli

# Run database migrations
sqlx migrate run

# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run
```

## Research Output

The system generates structured research data suitable for:
- **Strategic Decision Making**: Evidence-based architecture recommendations
- **Risk Assessment**: Comprehensive understanding of potential challenges
- **Pattern Recognition**: Identification of successful approaches and anti-patterns
- **Implementation Planning**: Realistic timelines and resource requirements

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built following Design101 TDD-First Architecture Principles
- Implements Shreyas Doshi's product thinking methodology
- Uses PostgreSQL for superior full-text search and JSON handling
- Designed for Kiro IDE transfiguration research requirements