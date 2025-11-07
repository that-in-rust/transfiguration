# Transfiguration: Learning Archive & Knowledge Repository

**Turning software complexity into actually useful insights since Tuesday**

## Purpose

Transfiguration is a curated archive where we preserve context, decisions, and learnings from all projects. It's not just a dumping ground—it's a **knowledge base that prevents information loss and enables learning from the past**.

The biggest problem solved: **never losing valuable context and insights again.**

## What's Inside

This repository contains wisdom of what we did right and what we did wrong, organized by:

- **Repository-specific learnings**: Per-project archives with experiments, decisions, and evolution
- **Cross-repository research**: Analysis and patterns that span multiple projects
- **Shared learnings**: Successful patterns, anti-patterns, and decision frameworks
- **Archived projects**: Completed or paused projects with full context preserved

## Structure

```
transfiguration/
├── 00-archive-management-scripts/    # Tools for managing the archive
├── repository-level-learnings/        # Per-project knowledge
├── cross-repository-research/         # Research spanning projects
├── cross-repository-learnings/        # Patterns and frameworks
├── archived-completed-projects/       # Finished/paused projects
└── FILE-MAPPING.json                  # Maps shortened filenames to originals
```

## How to Use

### Before Starting New Work
1. Check `repository-level-learnings/<project-name>/` for past context
2. Review `cross-repository-learnings/` for applicable patterns
3. Check `archived-completed-projects/` to avoid repeating past mistakes

### When Archiving Content
1. Follow the guidelines in `00-archive-management-scripts/`
2. Use descriptive folder names (minimum 3 words)
3. Update ARCHIVE-LOG.md with what you moved and why

### Finding Original Filenames
Many files have been shortened for readability. To find the original name:
```bash
python3 00-archive-management-scripts/shorten_filename.py --lookup "shortened-filename.md"
```

## File Naming Convention

Files with names longer than 30 characters are automatically shortened:
- Format: `last30chars-hash.ext`
- Example: `comprehensive_fork_catalog-18f4c856.md`
- All mappings stored in `FILE-MAPPING.json`

## Philosophy

**Every folder name should tell you what's inside.** No generic "experiments" or "research"—each folder is specific and meaningful. More folders = better organization.

The essence: Whatever gets created should be learnable. This archive is our institutional memory.

## Meta

- See `ARCHIVE-LOG.md` for timeline of what was archived when
- All `.claude/` directories are preserved except this repo's own
- File classifications can evolve—add new folders as patterns emerge
