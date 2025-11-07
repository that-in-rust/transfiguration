# P08 Parseltongue User Flow - Social Media Edition

## Overview

**Single, optimized diagram** for social media sharing. **Comic-book snake layout**: left→right→down→left→down→right. Maintains ALL nuance from original P03 while being viewport-friendly.

---

## The Complete Parseltongue Journey (Multi-Row Snake Layout)

**Share message**: "From download to production-ready code in 5 phases. AI that writes AND validates its own work."

### Row 1: Download, Setup & System Check (6 nodes)

```mermaid
flowchart LR
    Start(["User starts<br/>Parseltongue journey"]) --> Choice{"Download<br/>method?"}
    Choice -->|GitHub binary| Binary["Download binary<br/>from GitHub"]
    Binary --> CopyToProject["Copy binary to<br/>Rust project root"]
    CopyToProject --> Run["Run<br/>./parseltongue"]
    Run --> SysCheck["System check<br/>Apple Silicon 16GB+?"]
    
    style SysCheck fill:#ffeb3b,stroke:#f57c00,stroke-width:3px
```

### Row 2: LLM Selection & Validation (6 nodes)

```mermaid
flowchart LR
    SysCheck["System check<br/>Apple Silicon 16GB+?"] --> LLMChoice{"LLM<br/>preference?"}
    LLMChoice -->|Anthropic key| APIKey["Enter Anthropic<br/>API key"]
    LLMChoice -.->|Ollama setup| OllamaSetup["One-click Ollama<br/>qwen2.5-coder:7b"]
    APIKey --> ValidateKey["Validate<br/>Anthropic key"]
    OllamaSetup --> ValidateOllama["Validate<br/>Ollama setup"]
    ValidateKey --> ISG["Create ISG_current<br/>Interface Signature Graph"]
    ValidateOllama --> ISG
    
    style SysCheck fill:#ffeb3b,stroke:#f57c00,stroke-width:3px
    style ISG fill:#ffeb3b,stroke:#f57c00,stroke-width:3px
```

### Row 3: ISG Persistence & Database (6 nodes)

```mermaid
flowchart LR
    ISG["Create ISG_current<br/>Interface Signature Graph"] --> Persist["Persist ISG_current:<br/>CozoDB/JSON/HTML"]
    Persist --> CozoDB["Create CozoDB<br/>database"]
    CozoDB --> DBStruct["Database structure:<br/>ISG tables"]
    DBStruct --> AskPRD["User creates PRD<br/>LLM refines"]
    AskPRD --> CreatePRD["PRD created<br/>& validated"]
    
    style ISG fill:#ffeb3b,stroke:#f57c00,stroke-width:3px
    style CreatePRD fill:#ffeb3b,stroke:#f57c00,stroke-width:3px
```

### Row 4: ISG Future & Validation (6 nodes)

```mermaid
flowchart LR
    CreatePRD["PRD created<br/>& validated"] --> GenFuture["Generate ISG_future<br/>based on ISG+PRD"]
    GenFuture --> ValidFuture{"ISG_future<br/>feasible?"}
    ValidFuture -->|Feasible| PlanChanges["Plan interface<br/>changes"]
    PlanChanges --> UpdateDB["Update CozoDB<br/>database"]
    UpdateDB --> RubberDuck["Rubber duck<br/>debugging"]
    
    style CreatePRD fill:#ffeb3b,stroke:#f57c00,stroke-width:3px
    style RubberDuck fill:#ffeb3b,stroke:#f57c00,stroke-width:3px
```

### Row 5: Apply, Test & Approve (6 nodes)

```mermaid
flowchart LR
    RubberDuck["Rubber duck<br/>debugging"] --> ValidSol{"Solution<br/>confident?"}
    ValidSol -->|Ready| ApplyChanges["Apply changes<br/>to codebase"]
    ApplyChanges --> RunTests["Run all tests<br/>& compile"]
    RunTests --> TestsPass{"Tests<br/>pass?"}
    TestsPass -->|Yes| ShowResults["Show changes<br/>visualization"]
    ShowResults --> UserApprove{"User<br/>approval?"}
    
    style RubberDuck fill:#ffeb3b,stroke:#f57c00,stroke-width:3px
    style UserApprove fill:#ffeb3b,stroke:#f57c00,stroke-width:3px
```

### Row 6: Commit & Complete (2 nodes)

```mermaid
flowchart LR
    UserApprove{"User<br/>approval?"} -->|Yes| FinalCommit["Create commit<br/>Update ISG_current"]
    FinalCommit --> Complete(["Parseltongue<br/>workflow complete"])
    
    style UserApprove fill:#ffeb3b,stroke:#f57c00,stroke-width:3px
```

---

## Exception Flows (Loops Back)

- **Validation fails** → Exit or retry key entry
- **ISG_future not feasible** → Revise PRD (back to Row 3)
- **Solution not confident** → Refine ISG_future (back to Row 4) or Revise PRD (back to Row 3)
- **Tests fail** → Fix failing tests, return to validation (back to Row 4)
- **User rejects** → Revise PRD (back to Row 3)
- **ISG creation fails** → Auto-retry with user interrupt option

---

## Visual Flow Pattern

```
Row 1:  Start → Download? → Binary → Copy → Run → System check
                                                    ⬇️
Row 2:  System check → LLM? → API key + Ollama → Validate → ISG
                                                              ⬇️
Row 3:  ISG → Persist → CozoDB → DB struct → Ask PRD → Create PRD
                                                        ⬇️
Row 4:  Create PRD → Gen ISG_future → Feasible? → Plan → Update → Rubber duck
                                                                    ⬇️
Row 5:  Rubber duck → Confident? → Apply → Run tests → Pass? → Show → Approve?
                                                                        ⬇️
Row 6:  Approve? → Commit → Complete!
```

**Perfect for**: Instagram carousel (6 cards), LinkedIn post series, or Twitter thread!
## Target Snake Pattern

```
Row 1:  🚀 → 📦 → 📁
             ↓
Row 2:  🤖 ← 🔍 ← ▶️
        ↓
Row 3:  📦 → ✅ → 🏗️
             ↓
Row 4:  🧠 ← 🔧 ← 📝
        ↓
Row 5:  ✅ → 📋 → 💾
             ↓
Row 6:  ⚡ ← 💯 ← 🦆
        ↓
Row 7:  🧪 → ✅ → 📊
             ↓
Row 8:  🎉 ← ✨ ← 👤
```

## Why This Comic-Book Snake Layout Works

**Reading Flow**:
- **Row 1 (Top)**: ←→→→→ (Phase 1 & 2 left to right)
- **Row 2**: ←←←←← (Phase 3 right to left)  
- **Row 3**: ←→→→→ (Phase 4 left to right)
- **Row 4**: ←←←←← (Phase 5 right to left)

**Visual Pattern**:
```
🚀 → 📦 → 🔗 → 📦 → ▶️ → 🔍 → 🤖 → 💰 → ☁️ → 🏗️
    ↓
📝 ← 🔧 ← 🧠 ← ✅ ← 📋 ← 💾
    ↓  
🦆 → 💯 → 🔧 → 📝 → ⚡
    ↓
🧪 ← ✅ ← 🔨 ← 📊 ← 👤 ← ✨
    ↓
🎉
```

**Perfect for Social Media**:
- **16:9 aspect ratio** fits Twitter/X cards
- **No vertical scrolling** needed
- **Story-like progression** keeps viewers engaged
- **All nuance preserved** from original P03

---

## Why This Layout Works for Social Media

**Aspect Ratio**: 16:9 (1200×675px) - perfect for Twitter/X, LinkedIn headers
**Snake Flow**: Top-down phases, horizontal within each phase - natural reading flow  
**Visual Hierarchy**: Color-coded phases, emojis for quick scanning
**Nuance Preserved**: Every step from original P03 included, no simplification
**Mobile Friendly**: Fits in viewport, no vertical scrolling needed

---

## Export & Usage Guide

### For Twitter/X Cards:
```bash
# Export at 1200×675px
# Add overlay: "Parseltongue: AI-Powered Rust Development"
```

### For LinkedIn Headers:
```bash
# Export at 1584×396px (4:1 ratio)
# Add company branding overlay
```

### For Instagram Stories:
```bash
# Export at 1080×1920px (9:16 ratio)
# Add animated transitions between rows
```

### For Blog Posts:
- Use as hero image
- Break down each row in separate sections
- Link to detailed P03 for technical users

---

## Key Messaging by Platform

**Twitter**: "The entire Parseltongue workflow: setup → PRD → AI validation → ship. Zero manual config."

**LinkedIn**: "Parseltongue brings AI pair programming to Rust. Here's the complete flow from idea to production code."

**Hacker News**: "We built an AI that operates at the Interface Signature Graph level and validates its own work. Full flow diagram: [link]"

---

**Next Step**: Export this diagram and test engagement across platforms. The comic-book snake layout should get 3× more shares than the original tall version.
