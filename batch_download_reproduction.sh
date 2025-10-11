#!/bin/bash
# Batch Download Reproduction Script
# Created: 2025-10-11T02:57:20Z
# Purpose: Reproduce all successful downloads from static analysis session
# Total Expected: 11 files, ~2.2GB

set -e

echo "🚀 Starting batch download reproduction..."
echo "📅 Original session: 2025-10-10T19:07:16Z to 2025-10-11T02:57:20Z"
echo "📊 Expected: 11 files, ~2.2GB total"
echo ""

# Ensure we're in the downloads directory
cd downloads-for-analysis

# Create directories if they don't exist
mkdir -p {ides,editors,chat-apps,web-frameworks,extracted,analysis-results}

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
echo "📝 PHASE 2: Editors & Development Tools"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# VS Code (113MB) - Industry Standard
echo "📦 Downloading VS Code..."
curl -L -o editors/vscode.deb "https://code.visualstudio.com/sha/download?build=stable&os=linux-deb-x64"
verify_download "editors/vscode.deb" "100M"

# Atom Editor (145MB) - Classic Electron
echo "📦 Downloading Atom..."
curl -L -o editors/atom.deb "https://github.com/atom/atom/releases/download/v1.60.0/atom-amd64.deb"
verify_download "editors/atom.deb" "130M"

# Sublime Text (15MB) - Native C++
echo "📦 Downloading Sublime Text..."
curl -L -o editors/sublime-text.deb "https://download.sublimetext.com/sublime-text_build-4180_amd64.deb"
verify_download "editors/sublime-text.deb" "10M"

# GitHub CLI (18MB) - Dynamic URL
echo "📦 Downloading GitHub CLI (dynamic URL)..."
GITHUB_CLI_URL=$(curl -s "https://api.github.com/repos/cli/cli/releases/latest" | grep browser_download_url | grep linux_amd64.deb | cut -d '"' -f 4)
if [ -n "$GITHUB_CLI_URL" ]; then
    curl -L -o editors/github-cli.deb "$GITHUB_CLI_URL"
    verify_download "editors/github-cli.deb" "15M"
else
    echo "❌ Could not determine GitHub CLI URL"
fi

echo ""
echo "🔧 PHASE 3: IDEs & Complex Tools"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# IntelliJ IDEA Community (849MB) - Major IDE
echo "📦 Downloading IntelliJ IDEA Community (large download - ~849MB)..."
curl -L -o ides/intellij-idea-community.tar.gz "https://download.jetbrains.com/idea/ideaIC-2024.2.4.tar.gz"
verify_download "ides/intellij-idea-community.tar.gz" "800M"

# PyCharm Community (640MB) - Python IDE
echo "📦 Downloading PyCharm Community (large download - ~640MB)..."
curl -L -o ides/pycharm-community.tar.gz "https://download.jetbrains.com/python/pycharm-community-2024.2.4.tar.gz"
verify_download "ides/pycharm-community.tar.gz" "600M"

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
echo "📁 Files by Category:"
for dir in chat-apps editors ides web-frameworks; do
    if [ -d "$dir" ]; then
        file_count=$(find "$dir" -type f | wc -l)
        if [ "$file_count" -gt 0 ]; then
            total_size=$(du -sh "$dir" 2>/dev/null | cut -f1 || echo "0")
            echo "   $dir: $file_count files ($total_size)"
            find "$dir" -type f -name "*.deb" -o -name "*.tar.gz" -o -name "*.tar.xz" | while read -r file; do
                file_size=$(du -sh "$file" 2>/dev/null | cut -f1 || echo "?")
                echo "     - $(basename "$file") ($file_size)"
            done
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
echo "✅ Batch download reproduction complete!"
echo "📋 Next steps:"
echo "   1. Extract packages: dpkg-deb -x <file.deb> extracted/<tool>/"
echo "   2. Run analysis: cd Insp01Kiro && ./analyze_kiro.sh"
echo "   3. Review logs: cat ../DOWNLOAD_SESSION_LOG.md"
echo ""
echo "📝 All download methods logged in ../DOWNLOAD_REPRODUCTION_GUIDE.md"
