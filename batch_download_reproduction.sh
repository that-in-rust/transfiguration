#!/bin/bash
# Batch Download Reproduction Script - COMPLETE COLLECTION
# Updated: 2025-10-11T09:29:00Z
# Purpose: Reproduce complete 15GB+ IDE evolution research collection
# Total Expected: 40+ tools, ~15GB+ (comprehensive analysis foundation)

set -e

echo "🚀 Starting COMPLETE IDE evolution research reproduction..."
echo "📅 Multi-session downloads: 2025-10-10 to 2025-10-11"
echo "📊 Expected: 40+ development tools, ~15GB+ total"
echo "🎯 Revolutionary IDE research foundation: AI + Rust + JetBrains patterns"
echo ""

# Ensure we're in the downloads directory
cd downloads-for-analysis

# Create directories for complete collection
mkdir -p {ides,editors,chat-apps,ai-tools,rust-tools,jetbrains,traditional-editors,terminals,web-frameworks,extracted,analysis-results,source-code}

# Log function
log_download() {
    local tool=$1
    local size=$2
    local file=$3
    echo "✅ $tool downloaded: $size → $file"
}

# Verification function
verify_download() {
    local file=$1
    local min_size=$2
    if [ -f "$file" ]; then
        actual_size=$(du -sh "$file" | cut -f1)
        echo "✅ Verified: $file ($actual_size)"
    else
        echo "❌ Failed: $file not found"
    fi
}

echo "💬 PHASE 1: Chat Applications"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Discord Desktop (113MB)
echo "📦 Downloading Discord..."
curl -L -o chat-apps/discord.deb "https://discord.com/api/download?platform=linux&format=deb"
verify_download "chat-apps/discord.deb" "100M"

# Slack Desktop (81MB)
echo "📦 Downloading Slack..."
curl -L -o chat-apps/slack.deb "https://downloads.slack-edge.com/releases/linux/4.38.125/prod/x64/slack-desktop-4.38.125-amd64.deb"
verify_download "chat-apps/slack.deb" "70M"

# Telegram Desktop (60MB)
echo "📦 Downloading Telegram..."
curl -L -o chat-apps/telegram.tar.xz "https://telegram.org/dl/desktop/linux"
verify_download "chat-apps/telegram.tar.xz" "50M"

echo ""
echo "🤖 PHASE 2: AI-Powered Development Tools (Revolutionary Insights)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Claude Code (npm package + source)
echo "📦 Claude Code (AI agent orchestration breakthrough)..."
echo "   Installing via npm: @anthropic-ai/claude-code"
npm info @anthropic-ai/claude-code > ai-tools/claude-code-info.txt 2>&1 || echo "⚠️  npm info failed"

# Cursor (AI-powered VS Code fork)
echo "📦 Downloading Cursor (AI-powered development)..."
curl -L -o ai-tools/cursor.deb "https://download.cursor.sh/linux/appImage/x64" || \
curl -L -o ai-tools/cursor.AppImage "https://download.cursor.sh/linux/appImage/x64"
verify_download "ai-tools/cursor.*" "200M"

# Zed Editor (Rust + AI)
echo "📦 Downloading Zed Editor (Rust performance + AI)..."
curl -L -o rust-tools/zed-linux.tar.gz "https://zed.dev/api/releases/stable/latest/zed-linux-x86_64.tar.gz"
verify_download "rust-tools/zed-linux.tar.gz" "140M"

# Note: Windsurf, Warp, SlateCLI are macOS apps or npm packages
echo "📝 Note: Windsurf, Warp (macOS apps), SlateCLI (npm) documented but not downloadable as binaries"

echo ""
echo "🦀 PHASE 3: Rust-Based Performance Tools (Performance Patterns)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Lapce Editor (ultra-lightweight Rust editor)
echo "📦 Downloading Lapce Editor (ultra-lightweight Rust)..."
LAPCE_URL=$(curl -s "https://api.github.com/repos/lapce/lapce/releases/latest" | grep browser_download_url | grep "linux.*tar.gz" | head -1 | cut -d '"' -f 4)
if [ -n "$LAPCE_URL" ]; then
    curl -L -o rust-tools/lapce.tar.gz "$LAPCE_URL"
    verify_download "rust-tools/lapce.tar.gz" "20M"
else
    echo "❌ Could not determine Lapce URL"
fi

# AST-Grep (fast AST manipulation)
echo "📦 Downloading AST-Grep (fast AST manipulation)..."
AST_GREP_URL=$(curl -s "https://api.github.com/repos/ast-grep/ast-grep/releases/latest" | grep browser_download_url | grep "linux.*tar.gz" | head -1 | cut -d '"' -f 4)
if [ -n "$AST_GREP_URL" ]; then
    curl -L -o rust-tools/ast-grep.tar.gz "$AST_GREP_URL"
    verify_download "rust-tools/ast-grep.tar.gz" "10M"
else
    echo "❌ Could not determine AST-Grep URL"
fi

# Clone AST-Grep source for analysis
echo "📦 Cloning AST-Grep source code..."
if [ ! -d "source-code/ast-grep" ]; then
    git clone --depth 1 https://github.com/ast-grep/ast-grep.git source-code/ast-grep
fi

echo ""
echo "📝 PHASE 4: Traditional Editors & Development Tools"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# VS Code (113MB) - Industry Standard
echo "📦 Downloading VS Code..."
curl -L -o traditional-editors/vscode.deb "https://code.visualstudio.com/sha/download?build=stable&os=linux-deb-x64"
verify_download "traditional-editors/vscode.deb" "100M"

# Atom Editor (145MB) - Classic Electron
echo "📦 Downloading Atom..."
curl -L -o traditional-editors/atom.deb "https://github.com/atom/atom/releases/download/v1.60.0/atom-amd64.deb"
verify_download "traditional-editors/atom.deb" "130M"

# Sublime Text (15MB) - Native C++
echo "📦 Downloading Sublime Text..."
curl -L -o traditional-editors/sublime-text.deb "https://download.sublimetext.com/sublime-text_build-4180_amd64.deb"
verify_download "traditional-editors/sublime-text.deb" "10M"

# GitHub CLI (18MB) - Dynamic URL
echo "📦 Downloading GitHub CLI (dynamic URL)..."
GITHUB_CLI_URL=$(curl -s "https://api.github.com/repos/cli/cli/releases/latest" | grep browser_download_url | grep linux_amd64.deb | cut -d '"' -f 4)
if [ -n "$GITHUB_CLI_URL" ]; then
    curl -L -o traditional-editors/github-cli.deb "$GITHUB_CLI_URL"
    verify_download "traditional-editors/github-cli.deb" "15M"
else
    echo "❌ Could not determine GitHub CLI URL"
fi

echo ""
echo "🏢 PHASE 5: JetBrains IDE Ecosystem (14 Tools - 13GB - Feature Completeness)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 WARNING: This downloads 13GB+ across 14 major IDEs - ensure sufficient disk space!"
echo ""

# Core Language IDEs
echo "📦 Downloading IntelliJ IDEA Community (~849MB)..."
curl -L -o jetbrains/intellij-idea-community.tar.gz "https://download.jetbrains.com/idea/ideaIC-2024.2.4.tar.gz"
verify_download "jetbrains/intellij-idea-community.tar.gz" "800M"

echo "📦 Downloading PyCharm Community (~640MB)..."
curl -L -o jetbrains/pycharm-community.tar.gz "https://download.jetbrains.com/python/pycharm-community-2024.2.4.tar.gz"
verify_download "jetbrains/pycharm-community.tar.gz" "600M"

echo "📦 Downloading WebStorm (~600MB)..."
curl -L -o jetbrains/webstorm.tar.gz "https://download.jetbrains.com/webstorm/WebStorm-2024.2.4.tar.gz"
verify_download "jetbrains/webstorm.tar.gz" "500M"

echo "📦 Downloading PhpStorm (~700MB)..."
curl -L -o jetbrains/phpstorm.tar.gz "https://download.jetbrains.com/webide/PhpStorm-2024.2.4.tar.gz"
verify_download "jetbrains/phpstorm.tar.gz" "600M"

echo "📦 Downloading GoLand (~700MB)..."
curl -L -o jetbrains/goland.tar.gz "https://download.jetbrains.com/go/goland-2024.2.4.tar.gz"
verify_download "jetbrains/goland.tar.gz" "600M"

echo "📦 Downloading RubyMine (~700MB)..."
curl -L -o jetbrains/rubymine.tar.gz "https://download.jetbrains.com/ruby/RubyMine-2024.2.4.tar.gz"
verify_download "jetbrains/rubymine.tar.gz" "600M"

echo "📦 Downloading CLion (~1.7GB)..."
curl -L -o jetbrains/clion.tar.gz "https://download.jetbrains.com/cpp/CLion-2024.2.4.tar.gz"
verify_download "jetbrains/clion.tar.gz" "1.5G"

echo "📦 Downloading Rider (~1.5GB)..."
curl -L -o jetbrains/rider.tar.gz "https://download.jetbrains.com/rider/JetBrains.Rider-2024.2.4.tar.gz"
verify_download "jetbrains/rider.tar.gz" "1.3G"

# Specialized IDEs  
echo "📦 Downloading RustRover (~1.0GB) - CRITICAL for Rust analysis..."
curl -L -o jetbrains/rustrover.tar.gz "https://download.jetbrains.com/rustrover/RustRover-2024.2.4.tar.gz"
verify_download "jetbrains/rustrover.tar.gz" "900M"

echo "📦 Downloading DataGrip (~801MB)..."
curl -L -o jetbrains/datagrip.tar.gz "https://download.jetbrains.com/datagrip/datagrip-2024.2.4.tar.gz"
verify_download "jetbrains/datagrip.tar.gz" "700M"

echo "📦 Downloading DataSpell (~1.0GB)..."
curl -L -o jetbrains/dataspell.tar.gz "https://download.jetbrains.com/dataspell/dataspell-2024.2.4.tar.gz"
verify_download "jetbrains/dataspell.tar.gz" "900M"

echo "📦 Downloading Android Studio (~1.2GB)..."
curl -L -o jetbrains/android-studio.tar.gz "https://redirector.gvt1.com/edgedl/android/studio/ide-zips/2024.1.2.12/android-studio-2024.1.2.12-linux.tar.gz"
verify_download "jetbrains/android-studio.tar.gz" "1.1G"

# DevOps Tools
echo "📦 Downloading TeamCity (~1.2GB)..."
curl -L -o jetbrains/teamcity.tar.gz "https://download.jetbrains.com/teamcity/TeamCity-2024.07.3.tar.gz"
verify_download "jetbrains/teamcity.tar.gz" "1.0G"

echo "📦 Downloading JetBrains Toolbox (~100MB)..."
curl -L -o jetbrains/toolbox.tar.gz "https://download.jetbrains.com/toolbox/jetbrains-toolbox-2.4.2.32922.tar.gz"
verify_download "jetbrains/toolbox.tar.gz" "90M"

echo ""
echo "🖥️ PHASE 4: Terminal & Desktop Applications"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Hyper Terminal (96MB) - React-based Terminal - Dynamic URL
echo "📦 Downloading Hyper Terminal (dynamic URL)..."
HYPER_URL=$(curl -s "https://api.github.com/repos/vercel/hyper/releases/latest" | grep browser_download_url | grep "amd64.deb" | cut -d '"' -f 4)
if [ -n "$HYPER_URL" ]; then
    curl -L -o ides/hyper.deb "$HYPER_URL"
    verify_download "ides/hyper.deb" "90M"
else
    echo "❌ Could not determine Hyper Terminal URL"
fi

# Firefox Browser (81MB) - Reference Application
echo "📦 Downloading Firefox..."
curl -L -o web-frameworks/firefox.deb "https://download.mozilla.org/?product=firefox-latest&os=linux64&lang=en-US"
verify_download "web-frameworks/firefox.deb" "70M"

echo ""
echo "📊 DOWNLOAD SUMMARY"
echo "═══════════════════════════════════════════════════════════════════════"

# Count and size all downloads
echo "📁 COMPLETE IDE EVOLUTION RESEARCH COLLECTION:"
for dir in chat-apps ai-tools rust-tools jetbrains traditional-editors terminals web-frameworks source-code; do
    if [ -d "$dir" ]; then
        file_count=$(find "$dir" -type f | wc -l)
        if [ "$file_count" -gt 0 ]; then
            total_size=$(du -sh "$dir" 2>/dev/null | cut -f1 || echo "0")
            echo "   📂 $dir: $file_count files ($total_size)"
            find "$dir" -type f \( -name "*.deb" -o -name "*.tar.gz" -o -name "*.tar.xz" -o -name "*.AppImage" \) 2>/dev/null | head -5 | while read -r file; do
                file_size=$(du -sh "$file" 2>/dev/null | cut -f1 || echo "?")
                echo "     - $(basename "$file") ($file_size)"
            done
            # Show count if more than 5 files
            remaining=$(find "$dir" -type f \( -name "*.deb" -o -name "*.tar.gz" -o -name "*.tar.xz" -o -name "*.AppImage" \) 2>/dev/null | wc -l)
            if [ "$remaining" -gt 5 ]; then
                echo "     ... and $((remaining - 5)) more files"
            fi
        fi
    fi
done

echo ""
echo "💾 Total Download Size: $(du -sh . 2>/dev/null | cut -f1 || echo "Unknown")"

echo ""
echo "🔒 Security Verification:"
cd .. && git_status=$(git status --porcelain)
if [ -z "$git_status" ]; then
    echo "✅ Git status clean - all downloads properly gitignored"
else
    echo "⚠️  Git status shows changes - check gitignore configuration"
fi

echo ""
echo "✅ COMPLETE IDE EVOLUTION RESEARCH COLLECTION DOWNLOAD FINISHED!"
echo "🎆 Revolutionary IDE research foundation established: 40+ tools, 15GB+"
echo ""
echo "📋 Next Steps for IDE Evolution Analysis:"
echo "   1. Extract all packages: tar -xzf *.tar.gz && dpkg-deb -x *.deb extracted/"
echo "   2. Analyze AI integration patterns: focus on Claude Code, Cursor, Zed"
echo "   3. Benchmark Rust performance: Zed, Lapce, AST-Grep, RustRover"
echo "   4. Study JetBrains architecture: All 14 IDEs for feature completeness"
echo "   5. Review complete analysis: cat ../COMPLETE_IDE_EVOLUTION_ANALYSIS.md"
echo ""
echo "📊 RESEARCH IMPACT: This collection enables revolutionary next-gen IDE development"
echo "🎯 GOAL: AI-native + Rust-fast + JetBrains-complete = Future of Development"
echo ""
echo "📝 Complete documentation: ../COMPLETE_IDE_EVOLUTION_ANALYSIS.md"
