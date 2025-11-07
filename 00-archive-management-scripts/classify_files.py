#!/usr/bin/env python3
"""
File Classification Script
Classifies all Markdown and text files in the repository based on content.
Generates a table of contents with 10-word classifications.
"""

import os
import pathlib
import json
from typing import Dict, List, Tuple

def classify_file(file_path: str) -> str:
    """
    Classify a file based on its content.
    Returns a 10-word (or less) classification.
    """
    try:
        with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
            # Read first 500 characters for classification
            content = f.read(500).lower()

        # Determine classification based on keywords and patterns
        if 'readme' in os.path.basename(file_path).lower():
            return "Project documentation and setup instructions overview"
        elif 'cargo.toml' in file_path.lower() or 'rust' in content:
            return "Rust project configuration and dependency management"
        elif 'test' in file_path.lower() or 'test' in content:
            return "Testing infrastructure and test case definitions"
        elif 'api' in content or 'endpoint' in content:
            return "API documentation and endpoint specifications"
        elif 'architecture' in content or 'design' in content:
            return "Software architecture and system design documentation"
        elif 'migration' in content or 'upgrade' in content:
            return "Migration guides and upgrade procedures"
        elif 'performance' in content or 'optimization' in content:
            return "Performance analysis and optimization strategies"
        elif 'analysis' in content or 'research' in content:
            return "Research findings and analytical documentation"
        elif 'spec' in file_path.lower() or 'requirement' in content:
            return "Technical specifications and system requirements"
        elif 'implementation' in content or 'code' in content:
            return "Implementation details and code documentation"
        elif 'config' in file_path.lower() or 'configuration' in content:
            return "Configuration settings and setup parameters"
        elif 'security' in content or 'auth' in content:
            return "Security guidelines and authentication documentation"
        elif 'ide' in content or 'vscode' in content or 'editor' in content:
            return "IDE features and editor integration documentation"
        elif 'pipeline' in content or 'workflow' in content:
            return "CI/CD pipeline and workflow automation documentation"
        elif 'docker' in content or 'container' in content:
            return "Container configuration and deployment documentation"
        elif 'database' in content or 'sql' in content:
            return "Database schema and data management documentation"
        elif 'script' in file_path.lower() or '.sh' in file_path:
            return "Automation scripts and utility programs"
        elif 'ui' in content or 'interface' in content:
            return "User interface design and interaction patterns"
        elif 'wasm' in content or 'webassembly' in content:
            return "WebAssembly integration and compilation documentation"
        elif 'journal' in file_path.lower() or 'notes' in file_path.lower():
            return "Development journal and working notes"
        else:
            return "General documentation and project information"

    except Exception as e:
        return f"Unable to classify: {str(e)[:30]}"

def scan_repository(root_dir: str) -> Dict[str, List[Tuple[str, str]]]:
    """
    Scan repository for all markdown and text files.
    Returns a dictionary categorized by classification.
    """
    results = {}

    # Extensions to scan
    extensions = {'.md', '.txt', '.markdown'}

    # Directories to skip
    skip_dirs = {'.git', 'node_modules', 'target', 'build', '__pycache__', '.claude'}

    for root, dirs, files in os.walk(root_dir):
        # Filter out directories to skip
        dirs[:] = [d for d in dirs if d not in skip_dirs]

        for file in files:
            ext = pathlib.Path(file).suffix
            if ext in extensions:
                file_path = os.path.join(root, file)
                rel_path = os.path.relpath(file_path, root_dir)

                classification = classify_file(file_path)

                if classification not in results:
                    results[classification] = []
                results[classification].append(rel_path)

    return results

def generate_toc(classifications: Dict[str, List[str]], output_file: str):
    """
    Generate a table of contents markdown file.
    """
    with open(output_file, 'w', encoding='utf-8') as f:
        f.write("# Repository Table of Contents\n\n")
        f.write("Auto-generated classification of all markdown and text files.\n\n")
        f.write("---\n\n")

        # Sort classifications alphabetically
        for classification in sorted(classifications.keys()):
            files = classifications[classification]
            f.write(f"## {classification}\n\n")
            f.write(f"*{len(files)} file(s)*\n\n")

            for file_path in sorted(files):
                f.write(f"- `{file_path}`\n")
            f.write("\n")

        f.write("---\n\n")
        f.write(f"Total files classified: {sum(len(v) for v in classifications.values())}\n")
        f.write(f"Total categories: {len(classifications)}\n")

def main():
    # Get the repository root (current directory)
    repo_root = os.getcwd()

    print("Scanning repository for markdown and text files...")
    classifications = scan_repository(repo_root)

    print(f"Found {sum(len(v) for v in classifications.values())} files in {len(classifications)} categories")

    # Generate table of contents
    toc_file = os.path.join(repo_root, "TABLE_OF_CONTENTS.md")
    print(f"Generating table of contents: {toc_file}")
    generate_toc(classifications, toc_file)

    # Also save as JSON for programmatic use
    json_file = os.path.join(repo_root, "file_classifications.json")
    with open(json_file, 'w', encoding='utf-8') as f:
        json.dump(classifications, f, indent=2)

    print("Classification complete!")
    print(f"- Markdown TOC: {toc_file}")
    print(f"- JSON data: {json_file}")

if __name__ == "__main__":
    main()
