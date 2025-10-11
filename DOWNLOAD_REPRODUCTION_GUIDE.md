# Download Reproduction Guide
**Created**: 2025-10-11T02:57:20Z  
**Purpose**: Exact commands to reproduce all successful downloads  
**Total Downloads**: 11 tools (2.2GB)

## 🔧 Prerequisites Setup
```bash
# Ensure you're in the correct transfiguration directory
cd /path/to/your/transfiguration/

# Navigate to downloads directory (will be created if needed)
cd downloads-for-analysis

# Create required subdirectories
mkdir -p {ides,editors,chat-apps,web-frameworks,extracted,analysis-results}

# Verify gitignore protection (should show downloads-for-analysis/ in gitignore)
grep -A 10 "Downloads for static analysis" ../.gitignore
```

## ✅ Successful Download Commands

### 💬 Chat Applications (254MB)

#### Discord Desktop (113MB)
```bash
curl -L -o chat-apps/discord.deb "https://discord.com/api/download?platform=linux&format=deb"
# Verify: du -sh chat-apps/discord.deb
# Expected: ~113MB
```

#### Slack Desktop (81MB) 
```bash
curl -L -o chat-apps/slack.deb "https://downloads.slack-edge.com/releases/linux/4.38.125/prod/x64/slack-desktop-4.38.125-amd64.deb"
# Verify: du -sh chat-apps/slack.deb
# Expected: ~81MB
```

#### Telegram Desktop (60MB)
```bash
curl -L -o chat-apps/telegram.tar.xz "https://telegram.org/dl/desktop/linux"
# Verify: du -sh chat-apps/telegram.tar.xz  
# Expected: ~60MB
# Note: This is a tar.xz, not .deb
```

### 📝 Editors & Development Tools (1.8GB)

#### VS Code (113MB) - Industry Standard
```bash
curl -L -o editors/vscode.deb "https://code.visualstudio.com/sha/download?build=stable&os=linux-deb-x64"
# Verify: du -sh editors/vscode.deb
# Expected: ~113MB
# File format: code_1.94.2-1728494015_amd64.deb
```

#### IntelliJ IDEA Community (849MB) - Major IDE
```bash
curl -L -o ides/intellij-idea-community.tar.gz "https://download.jetbrains.com/idea/ideaIC-2024.2.4.tar.gz"
# Verify: du -sh ides/intellij-idea-community.tar.gz
# Expected: ~849MB
# Note: This is a tar.gz, not .deb
```

#### PyCharm Community (640MB) - Python IDE
```bash
curl -L -o ides/pycharm-community.tar.gz "https://download.jetbrains.com/python/pycharm-community-2024.2.4.tar.gz"
# Verify: du -sh ides/pycharm-community.tar.gz
# Expected: ~640MB  
# Note: This is a tar.gz, not .deb
```

#### Atom Editor (145MB) - Classic Electron
```bash
curl -L -o editors/atom.deb "https://github.com/atom/atom/releases/download/v1.60.0/atom-amd64.deb"
# Verify: du -sh editors/atom.deb
# Expected: ~145MB
# Note: Project discontinued but valuable for analysis
```

#### Sublime Text (15MB) - Native C++
```bash
curl -L -o editors/sublime-text.deb "https://download.sublimetext.com/sublime-text_build-4180_amd64.deb"
# Verify: du -sh editors/sublime-text.deb
# Expected: ~15MB
```

#### GitHub CLI (18MB) - Modern CLI + AI
```bash
# Dynamic URL approach (recommended)
GITHUB_CLI_URL=$(curl -s "https://api.github.com/repos/cli/cli/releases/latest" | grep browser_download_url | grep linux_amd64.deb | cut -d '"' -f 4)
curl -L -o editors/github-cli.deb "$GITHUB_CLI_URL"
# Verify: du -sh editors/github-cli.deb
# Expected: ~18MB

# Alternative static URL (may become outdated):
# curl -L -o editors/github-cli.deb "https://github.com/cli/cli/releases/download/v2.58.0/gh_2.58.0_linux_amd64.deb"
```

### 🖥️ Terminal & Desktop Applications (177MB)

#### Hyper Terminal (96MB) - React-based Terminal
```bash
# Dynamic URL approach (recommended)
HYPER_URL=$(curl -s "https://api.github.com/repos/vercel/hyper/releases/latest" | grep browser_download_url | grep "amd64.deb" | cut -d '"' -f 4)
curl -L -o ides/hyper.deb "$HYPER_URL"
# Verify: du -sh ides/hyper.deb
# Expected: ~96MB

# Alternative static URL (may become outdated):
# curl -L -o ides/hyper.deb "https://github.com/vercel/hyper/releases/download/v3.4.1/hyper_3.4.1_amd64.deb"
```

#### Firefox Browser (81MB) - Reference Application
```bash
curl -L -o web-frameworks/firefox.deb "https://download.mozilla.org/?product=firefox-latest&os=linux64&lang=en-US"
# Verify: du -sh web-frameworks/firefox.deb
# Expected: ~81MB
# Note: This downloads as .tar.bz2 but we save as .deb for consistency
```

## 🔍 Verification Commands

### Check All Downloads
```bash
# List all downloaded packages
find . -name "*.deb" -o -name "*.tar.gz" -o -name "*.tar.xz" | sort

# Show sizes of all downloads  
find . -name "*.deb" -o -name "*.tar.gz" -o -name "*.tar.xz" -exec du -sh {} \; | sort -hr

# Total download size
du -sh .
```

### Verify Git Protection
```bash
# Confirm downloads are gitignored (should show clean status)
cd .. && git status

# Should show: "nothing to commit, working tree clean"
# (Downloads folder and all .deb/.tar.gz files are gitignored)
```

## ❌ Failed Download Attempts (For Reference)

### Claude Desktop
```bash
# Attempted URLs (all failed):
# https://claude.ai/download/linux (returns HTML redirect)
# https://storage.googleapis.com/anthropic-assets/claude-desktop-linux-x64.deb (404)
# Status: No direct .deb download available
```

### Warp Terminal  
```bash
# Attempted methods:
# GitHub API search - no releases found
# Direct URLs - return HTML redirects
# Status: Uses proprietary distribution system
```

### Cursor Editor
```bash
# Attempted URLs:
# https://downloader.cursor.sh/linux/appImage/x64 (DNS resolution failed)
# Status: Likely uses AppImage or different distribution
```

### SlateCLI
```bash
# Issue: Ambiguous tool name
# Multiple projects named "slate" exist
# Need specific repository identification
```

### Alacritty/WezTerm/Neovim
```bash
# These projects don't distribute .deb packages
# They use:
# - AppImage (Alacritty)  
# - Distribution repos (WezTerm)
# - AppImage/Flatpak (Neovim)
```

## 🔄 Batch Download Script

The repository includes `batch_download_reproduction.sh` in the root directory.
Or create your own `batch_download.sh`:
```bash
#!/bin/bash
set -e

echo "🚀 Starting batch download reproduction..."

# Chat Apps
curl -L -o chat-apps/discord.deb "https://discord.com/api/download?platform=linux&format=deb"
curl -L -o chat-apps/slack.deb "https://downloads.slack-edge.com/releases/linux/4.38.125/prod/x64/slack-desktop-4.38.125-amd64.deb"  
curl -L -o chat-apps/telegram.tar.xz "https://telegram.org/dl/desktop/linux"

# Editors
curl -L -o editors/vscode.deb "https://code.visualstudio.com/sha/download?build=stable&os=linux-deb-x64"
curl -L -o editors/atom.deb "https://github.com/atom/atom/releases/download/v1.60.0/atom-amd64.deb"
curl -L -o editors/sublime-text.deb "https://download.sublimetext.com/sublime-text_build-4180_amd64.deb"

# Dynamic downloads
GITHUB_CLI_URL=$(curl -s "https://api.github.com/repos/cli/cli/releases/latest" | grep browser_download_url | grep linux_amd64.deb | cut -d '"' -f 4)
curl -L -o editors/github-cli.deb "$GITHUB_CLI_URL"

HYPER_URL=$(curl -s "https://api.github.com/repos/vercel/hyper/releases/latest" | grep browser_download_url | grep "amd64.deb" | cut -d '"' -f 4)  
curl -L -o ides/hyper.deb "$HYPER_URL"

# IDEs
curl -L -o ides/intellij-idea-community.tar.gz "https://download.jetbrains.com/idea/ideaIC-2024.2.4.tar.gz"
curl -L -o ides/pycharm-community.tar.gz "https://download.jetbrains.com/python/pycharm-community-2024.2.4.tar.gz"

# Browser
curl -L -o web-frameworks/firefox.deb "https://download.mozilla.org/?product=firefox-latest&os=linux64&lang=en-US"

echo "✅ Batch download complete!"
echo "📊 Total size: $(du -sh . | cut -f1)"
```

## 📊 Expected Results
- **11 total files** downloaded  
- **2.2GB total size**
- **Mix of .deb, .tar.gz, .tar.xz formats**
- **All files gitignored** (not committed to repo)
- **Comprehensive coverage** of different architectures:
  - Electron apps (Discord, Slack, VS Code, Atom, Hyper)
  - JVM apps (IntelliJ, PyCharm)  
  - Native apps (Sublime Text, Firefox)
  - CLI tools (GitHub CLI)
  - Chat applications (3 major platforms)

## 🔬 Next Steps for Analysis
```bash
# From the transfiguration root directory:
cd downloads-for-analysis

# Extract packages for analysis
dpkg-deb -x editors/vscode.deb extracted/vscode/
dpkg-deb -x chat-apps/discord.deb extracted/discord/
tar -xzf ides/intellij-idea-community.tar.gz -C extracted/
# ... etc for all packages

# Run static analysis
cd ../Insp01Kiro && ./analyze_kiro.sh
```

---
**Note**: This guide ensures 100% reproducibility of our successful downloads for the Campfire-on-Rust static analysis research project.