# Downloads Directory

This folder contains downloaded development tools for static analysis.

## 📚 Documentation (Located in Parent Directory)

- **../DOWNLOAD_SESSION_LOG.md** - Complete download session log
- **../DOWNLOAD_REPRODUCTION_GUIDE.md** - Detailed reproduction instructions  
- **../batch_download_reproduction.sh** - Executable batch download script

## 🔒 Git Protection

This entire `downloads-for-analysis/` folder is gitignored to prevent large binaries from being committed.

## 🚀 Quick Start

From the transfiguration root directory:
```bash
# Run the batch download script
./batch_download_reproduction.sh

# Or follow manual instructions
cat DOWNLOAD_REPRODUCTION_GUIDE.md
```

## 📊 Expected Contents After Download

```
downloads-for-analysis/
├── chat-apps/          # Discord, Slack, Telegram
├── editors/            # VS Code, Atom, Sublime Text, GitHub CLI  
├── ides/               # IntelliJ IDEA, PyCharm, Hyper Terminal
├── web-frameworks/     # Firefox
├── extracted/          # Extracted package contents
└── analysis-results/   # Analysis outputs
```

**Total Expected Size**: ~2.2GB across 11 tools
## 🎯 COMPLETE TOOL INVENTORY - 2025-10-11

### Directory Structure:
```
downloads-for-analysis/
├── ai-tools/                    # AI-powered development tools
│   ├── @anthropic-ai/claude-code (90M)
│   ├── @randomlabs/slatecli     (177M)
│   ├── Cursor.app -> /Applications/
│   ├── Windsurf.app -> /Applications/
│   └── Warp.app -> /Applications/
├── editors/                     # Code editors
│   ├── vscode.deb              (113M)
│   ├── atom.deb                (145M)
│   ├── zed-linux.tar.gz        (145M)
│   └── lapce-linux.tar.gz      (23M)
├── ides/                        # Integrated development environments
│   ├── intellij-idea-community.tar.gz (849M)
│   └── pycharm-community.tar.gz       (640M)
├── github-repos/                # Source code repositories
│   ├── claude-code/            (37M)
│   ├── openai-codex/           (119M)
│   └── ast-grep/               (12M)
├── cursor_1.7.44_amd64.deb     (103M)
├── cursor_1.7.44_arm64.deb     (100M)
└── cli-tools/                   # Command-line tools
    ├── ast-grep-darwin.zip     (6.9M)
    └── bun-linux.zip           (36M)
```

### Priority Tools for Analysis:
1. **AI-Powered Tools**: Cursor, Windsurf, Claude Code, SlateCLI
2. **Rust-Based Tools**: Zed, Lapce, AST-Grep
3. **JetBrains IDEs**: IntelliJ IDEA, PyCharm
4. **Traditional Editors**: VS Code, Atom, Sublime Text
