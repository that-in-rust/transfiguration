#!/usr/bin/env python3
"""
Filename Shortening Script - TRANSFIGURATION ONLY
Keeps last 30 characters of filename + 8-char hash at end
Format: last30chars-hash.ext

SAFETY: This script will ONLY work on files inside transfiguration/ directory
"""

import hashlib
import json
import os
import sys
from pathlib import Path
from typing import Dict, Tuple


def is_safe_directory(directory: Path) -> bool:
    """Ensure we're only working inside transfiguration directory."""
    try:
        # Get absolute path and check if 'transfiguration' is in the path
        abs_path = directory.resolve()
        return 'transfiguration' in str(abs_path).lower()
    except:
        return False


def generate_hash(text: str, length: int = 8) -> str:
    """Generate a short hash from text."""
    return hashlib.sha256(text.encode()).hexdigest()[:length]


def shorten_filename(original_filename: str, max_chars: int = 30) -> Tuple[str, str]:
    """
    Shorten a filename to last N characters + hash.

    Args:
        original_filename: Original filename with extension
        max_chars: Maximum characters to keep from original name

    Returns:
        Tuple of (shortened_filename, hash_used)
    """
    # Split name and extension
    path = Path(original_filename)
    name = path.stem
    ext = path.suffix

    # If name is already short enough, return as-is
    if len(name) <= max_chars:
        return original_filename, None

    # Generate hash from full original filename
    file_hash = generate_hash(original_filename)

    # Get last N characters of name
    shortened_name = name[-max_chars:] if len(name) > max_chars else name

    # Create new filename: last30chars-hash.ext
    new_filename = f"{shortened_name}-{file_hash}{ext}"

    return new_filename, file_hash


def process_directory(
    directory: Path,
    mapping_file: Path,
    dry_run: bool = True
) -> Dict[str, dict]:
    """
    Process all files in a directory and create mapping.

    Args:
        directory: Directory to process
        mapping_file: Path to JSON mapping file
        dry_run: If True, don't rename files, just show what would happen

    Returns:
        Dictionary mapping new names to original metadata
    """
    # SAFETY CHECK
    if not is_safe_directory(directory):
        print("ERROR: This script only works on transfiguration directory!")
        print(f"You tried to run it on: {directory}")
        sys.exit(1)

    mapping = {}

    if mapping_file.exists():
        with open(mapping_file, 'r') as f:
            mapping = json.load(f)

    print(f"Processing directory: {directory}")
    print(f"Dry run: {dry_run}\n")

    files_processed = 0
    files_renamed = 0

    for file_path in directory.rglob('*'):
        if file_path.is_file() and file_path != mapping_file:
            original_name = file_path.name
            new_name, file_hash = shorten_filename(original_name)

            files_processed += 1

            if new_name != original_name:
                files_renamed += 1
                new_path = file_path.parent / new_name
                rel_path = file_path.relative_to(directory.parent)

                print(f"{'[DRY RUN] ' if dry_run else ''}Renaming:")
                print(f"  From: {original_name}")
                print(f"  To:   {new_name}")
                print(f"  Location: {file_path.parent}")
                print()

                # Add to mapping
                mapping[new_name] = {
                    "original": original_name,
                    "hash": file_hash,
                    "location": str(rel_path.parent),
                    "archived_date": "2025-11-07"
                }

                if not dry_run:
                    file_path.rename(new_path)

    print(f"\nSummary:")
    print(f"  Files processed: {files_processed}")
    print(f"  Files renamed: {files_renamed}")
    print(f"  Mapping entries: {len(mapping)}")

    # Save mapping
    if not dry_run:
        with open(mapping_file, 'w') as f:
            json.dump(mapping, f, indent=2, sort_keys=True)
        print(f"\nMapping saved to: {mapping_file}")

    return mapping


def lookup_original(mapping_file: Path, short_name: str) -> dict:
    """Look up original filename from shortened name."""
    if not mapping_file.exists():
        return None

    with open(mapping_file, 'r') as f:
        mapping = json.load(f)

    return mapping.get(short_name)


def main():
    if len(sys.argv) < 2:
        print("Usage:")
        print("  Shorten single file:  python shorten_filename.py <filename>")
        print("  Process directory:    python shorten_filename.py <directory> [--execute]")
        print("  Lookup original:      python shorten_filename.py --lookup <short_name>")
        sys.exit(1)

    arg = sys.argv[1]

    # Lookup mode
    if arg == "--lookup":
        if len(sys.argv) < 3:
            print("Error: Provide short filename to lookup")
            sys.exit(1)

        mapping_file = Path(__file__).parent.parent / "FILE-MAPPING.json"
        result = lookup_original(mapping_file, sys.argv[2])

        if result:
            print(json.dumps(result, indent=2))
        else:
            print(f"No mapping found for: {sys.argv[2]}")
        return

    # Directory processing mode
    if os.path.isdir(arg):
        directory = Path(arg)

        # SAFETY CHECK
        if not is_safe_directory(directory):
            print("=" * 60)
            print("ERROR: SAFETY CHECK FAILED")
            print("This script only works on transfiguration directory!")
            print(f"You tried to run it on: {directory}")
            print("=" * 60)
            sys.exit(1)

        mapping_file = directory.parent / "FILE-MAPPING.json"
        dry_run = "--execute" not in sys.argv

        if dry_run:
            print("=" * 60)
            print("DRY RUN MODE - No files will be renamed")
            print("Add --execute flag to actually rename files")
            print("=" * 60)
            print()

        process_directory(directory, mapping_file, dry_run)
        return

    # Single file mode
    original = arg
    shortened, file_hash = shorten_filename(original)

    print(f"Original:  {original}")
    print(f"Shortened: {shortened}")
    if file_hash:
        print(f"Hash:      {file_hash}")


if __name__ == "__main__":
    main()
