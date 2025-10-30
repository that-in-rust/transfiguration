# Claude Standard Operating Procedures (SOP)

This document outlines all shell commands, scripts, and procedures used with Claude in this repository.

## Environment Setup

### Claude Code Z.AI Environment Setup
**File**: `claude_code_zai_env.sh`

**Purpose**: Automated setup of Claude Code with Z.AI API configuration

**Key Commands Used**:
```bash
# Run the environment setup script
./claude_code_zai_env.sh
```

**What the script does**:
1. Installs Node.js (v22) via nvm if not present
2. Installs Claude Code globally via npm
3. Configures Claude Code with Z.AI API endpoint
4. Sets up API key configuration
5. Creates necessary configuration directories

**Key Configuration**:
- API Base URL: `https://api.z.ai/api/anthropic`
- API Management: `https://z.ai/manage-apikey/apikey-list`
- Timeout: 3000000ms
- Package: `@anthropic-ai/claude-code`

## Development Commands

### Rust Development
**Primary Project Configuration**: `./Cargo.toml`

**Common Commands**:
```bash
# Build the project
cargo build

# Build with optimizations
cargo build --release

# Run tests
cargo test

# Run the main application
cargo run

# Run with specific arguments
cargo run -- --args

# Check code without building
cargo check

# Format code
cargo fmt

# Run clippy lints
cargo clippy

# Clean build artifacts
cargo clean

# Generate documentation
cargo doc

# Install specific tools
cargo install --path .

# Run specific examples
cargo run --example example_name

# Run tests with backtrace
RUST_BACKTRACE=1 cargo test

# Test specific module
cargo test module_name

# Build release and run
./target/release/app_name
```

### Node.js/npm Commands
**Usage**: Claude Code installation and management

```bash
# Install Claude Code globally
npm install -g @anthropic-ai/claude-code

# Check Node.js version
node -v

# Check npm version
npm -v

# Install nvm (Node Version Manager)
curl -s https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash

# Load nvm environment
. "$HOME/.nvm/nvm.sh"

# Install specific Node.js version
nvm install 22

# Use specific Node.js version
nvm use 22
```

### Python Commands
**Usage**: Various analysis scripts and tools

```bash
# Install packages
pip install package_name

# Install from requirements
pip install -r requirements.txt

# Run Python scripts
python3 script.py

# Install with specific version
python3 -m pip install package==version

# Run modules
python3 -m module_name
```

### Git Commands
**Repository Management**:

```bash
# Check status
git status

# Add files
git add .
git add file_name

# Commit changes
git commit -m "commit message"

# Push to remote
git push

# Pull changes
git pull

# Create new branch
git checkout -b branch_name

# Switch branches
git checkout branch_name

# Check git history
git log --oneline
git log --oneline -20

# Search git history
git log --grep="keyword" --oneline

# Check git version
git --version

# Clean up files
git clean -fd

# Reset repository
git reset --hard

# Remove files
git rm file_name

# Add remote
git remote add origin remote_url

# Check ignored files
git check-ignore file_name

# Filter repository (history rewrite)
git filter-repo --filter-filter
```

## Project-Specific Scripts

### IDE Analysis Scripts (Archived)
**Location**: `./zzArchive202510/tempPOC_archive/Insp09_ActiveWorkspaces_StrategicInitiatives/Insp01_IDEAnalysis/`

**Main Analysis Script**: `analyze_kiro.sh`
**Purpose**: Comprehensive IDE analysis pipeline

**Library Scripts**:
- `lib/api_surface_analyzer.sh`
- `lib/baseline_comparison.sh`
- `lib/behavioral_pattern_analyzer.sh`
- `lib/configuration_analyzer.sh`
- `lib/error_handling.sh`
- `lib/file_discovery.sh`
- `lib/output_management.sh`
- `lib/phase2_handoff.sh`
- `lib/result_aggregation.sh`
- `lib/typescript_parser.sh`
- `lib/ui_structure_analyzer.sh`
- `lib/validation_framework.sh`

**Test Scripts**:
- `test_analysis_pipeline.sh`
- `test_behavioral_analyzer.sh`
- `test_configuration_analyzer.sh`
- `test_minimal_behavioral.sh`
- `test_syntax.sh`
- `test_ui_analyzer.sh`

**Integration Script**: `integrate_analysis_results.sh`

### Tools Management
**Location**: `./zzArchive202510/tempPOC_archive/Insp09_ActiveWorkspaces_StrategicInitiatives/Insp00_Tools/`

**Batch Download Script**: `batch_download_reproduction.sh`
**Purpose**: Download and manage development tools collection

## System Commands

### File System Operations
```bash
# List files
ls -la

# Find files
find . -name "*.sh" -type f
find . -name "Cargo.toml"

# Create directories
mkdir -p directory_name

# Move/copy files
mv source destination
cp source destination

# Remove files/directories
rm file_name
rm -rf directory_name

# Change permissions
chmod +x script.sh

# Show file tree
tree directory_name

# Check disk usage
du -sh directory_name
```

### Process Management
```bash
# Run commands in background
command &

# Kill background processes
kill process_id

# Check running processes
ps aux

# System information
system_profiler SPSoftwareDataType
```

### Network Operations
```bash
# Download files
curl -O url
wget url

# Test connectivity
curl -I url

# Clone repositories
git clone repository_url
```

## Archive Management

### Archive Structure
**Main Archive Directory**: `./zzArchive202510/tempPOC_archive/`

**Contents**:
- IDE Analysis tools and scripts
- Development tools collection
- Historical analysis data
- Test files and validation scripts

### Archive Access
```bash
# Navigate to archive
cd zzArchive202510/tempPOC_archive/

# List archive contents
ls -la

# Extract compressed archives (if any)
unzip archive.zip
tar -xzf archive.tar.gz
```

## Claude Code Specific Operations

### Configuration Files
- **Settings**: `$HOME/.claude/settings.json`
- **Onboarding**: `$HOME/.claude.json`

### Environment Variables
- `ANTHROPIC_AUTH_TOKEN`: API key for authentication
- `ANTHROPIC_BASE_URL`: API base URL (Z.AI)
- `API_TIMEOUT_MS`: Request timeout (3000000ms)
- `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`: Disable telemetry

### Claude Code Commands
```bash
# Start Claude Code
claude

# Check version
claude --version

# Get help
claude --help

# Run with specific working directory
claude --working-directory path/to/project

# Use specific model
claude --model model_name

# Run in quiet mode
claude --quiet

# Disable colors
claude --no-color
```

## Testing Commands

### Rust Tests
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run tests in release mode
cargo test --release

# Run tests with specific features
cargo test --features "feature_name"

# Run tests with backtrace
RUST_BACKTRACE=1 cargo test

# Run tests with full backtrace
RUST_BACKTRACE=full cargo test
```

### Integration Tests
```bash
# Run specific test binaries
./target/release/test_binary

# Run with test data
./test_script.sh test_data/

# Run analysis pipeline
./test_analysis_pipeline.sh
```

## Build and Deployment

### Release Build
```bash
# Build release version
cargo build --release

# Run release binary
./target/release/app_name

# Create distribution
tar -czf app_name.tar.gz target/release/

# Install system-wide
cargo install --path .
```

### Documentation
```bash
# Generate documentation
cargo doc

# View documentation
cargo doc --open

# Generate documentation for specific crate
cargo doc -p crate_name
```

## Troubleshooting

### Common Issues
1. **Node.js version**: Ensure Node.js >= 18
2. **Rust toolchain**: Keep rustup updated
3. **API keys**: Verify Z.AI API key is valid
4. **Permissions**: Ensure scripts have execute permissions (`chmod +x`)

### Debug Commands
```bash
# Check Rust version
rustc --version
cargo --version

# Check Node.js version
node -v
npm -v

# Verify Claude Code installation
which claude
claude --version

# Check API connectivity
curl -I https://api.z.ai/api/anthropic

# Test script syntax
bash -n script.sh
```

## Notes

- This SOP is a living document and should be updated as new procedures are introduced
- All archived scripts are preserved for reference but may not be actively used
- Environment setup script (`claude_code_zai_env.sh`) should be run for new development environments
- Always test scripts in non-production environments before deployment