#!/usr/bin/env python3
"""
File filtering and cleanup script.
Preserves text files and JSON files with specific keywords.
"""

import os
import json
import pathlib
from typing import Dict, List, Tuple
from collections import defaultdict

def should_preserve_file(file_path: str) -> Tuple[bool, str]:
    """
    Determine if a file should be preserved.

    Returns:
        (should_preserve, reason)
    """
    ext = pathlib.Path(file_path).suffix.lower()

    # Rule 1: Preserve all .txt files
    if ext == '.txt':
        return (True, "txt_file")

    # Rule 2: Check JSON files for keywords
    if ext == '.json':
        try:
            with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                content = f.read().lower()

            # Check for keywords: rust, idiom, trun
            if 'rust' in content or 'idiom' in content or 'trun' in content:
                return (True, "json_with_keywords")
            else:
                return (False, "json_without_keywords")
        except Exception as e:
            # If we can't read the file, preserve it to be safe
            return (True, f"json_read_error: {str(e)[:50]}")

    # Rule 3: Preserve .md files (documentation)
    if ext == '.md':
        return (True, "markdown_file")

    # Rule 4: Preserve .py and .sh files (scripts)
    if ext in ['.py', '.sh']:
        return (True, "script_file")

    # Rule 5: Delete everything else
    return (False, f"other_type_{ext}")

def scan_and_analyze(root_dir: str) -> Dict:
    """
    Scan directory and build data structure with filtering decisions.
    """
    results = {
        'files': [],
        'stats': {
            'total': 0,
            'preserve': 0,
            'delete': 0
        },
        'by_reason': defaultdict(int),
        'by_extension': defaultdict(lambda: {'preserve': 0, 'delete': 0})
    }

    # Directories to skip
    skip_dirs = {'.git', '.claude', '__pycache__', 'node_modules', 'target'}

    for root, dirs, files in os.walk(root_dir):
        # Filter out directories to skip
        dirs[:] = [d for d in dirs if d not in skip_dirs]

        for file in files:
            file_path = os.path.join(root, file)
            rel_path = os.path.relpath(file_path, root_dir)

            preserve, reason = should_preserve_file(file_path)
            filter_value = 1 if preserve else 0

            # Get file size
            try:
                file_size = os.path.getsize(file_path)
            except:
                file_size = 0

            ext = pathlib.Path(file).suffix.lower()

            # Record the file
            results['files'].append({
                'path': rel_path,
                'filter': filter_value,
                'reason': reason,
                'size': file_size,
                'extension': ext
            })

            # Update stats
            results['stats']['total'] += 1
            if preserve:
                results['stats']['preserve'] += 1
                results['by_extension'][ext]['preserve'] += 1
            else:
                results['stats']['delete'] += 1
                results['by_extension'][ext]['delete'] += 1

            results['by_reason'][reason] += 1

    return results

def format_size(size_bytes: int) -> str:
    """Format bytes to human readable size."""
    for unit in ['B', 'KB', 'MB', 'GB']:
        if size_bytes < 1024.0:
            return f"{size_bytes:.1f}{unit}"
        size_bytes /= 1024.0
    return f"{size_bytes:.1f}TB"

def print_analysis(results: Dict):
    """Print comprehensive analysis."""
    print("\n" + "="*80)
    print("FILE FILTERING ANALYSIS")
    print("="*80)

    # Overall stats
    print(f"\nOVERALL STATISTICS:")
    print(f"  Total files scanned: {results['stats']['total']}")
    print(f"  Files to preserve (filter=1): {results['stats']['preserve']}")
    print(f"  Files to delete (filter=0): {results['stats']['delete']}")
    print(f"  Preservation rate: {results['stats']['preserve']/results['stats']['total']*100:.1f}%")

    # By reason
    print(f"\nBREAKDOWN BY REASON:")
    for reason, count in sorted(results['by_reason'].items(), key=lambda x: -x[1]):
        print(f"  {reason}: {count} files")

    # By extension
    print(f"\nBREAKDOWN BY FILE TYPE:")
    for ext, counts in sorted(results['by_extension'].items(), key=lambda x: -(x[1]['preserve'] + x[1]['delete'])):
        total = counts['preserve'] + counts['delete']
        print(f"  {ext if ext else '(no extension)'}: {total} total "
              f"(preserve: {counts['preserve']}, delete: {counts['delete']})")

    # Files marked for deletion
    print(f"\nFILES MARKED FOR DELETION (filter=0):")
    delete_files = [f for f in results['files'] if f['filter'] == 0]

    if delete_files:
        # Show first 20
        for i, f in enumerate(delete_files[:20]):
            size = format_size(f['size'])
            print(f"  [{i+1}] {f['path']} ({size}) - {f['reason']}")

        if len(delete_files) > 20:
            print(f"  ... and {len(delete_files) - 20} more files")

        # Calculate total size to delete
        total_size = sum(f['size'] for f in delete_files)
        print(f"\n  Total size to be deleted: {format_size(total_size)}")
    else:
        print("  No files marked for deletion")

    # Sample preserved files
    print(f"\nSAMPLE PRESERVED FILES (filter=1):")
    preserve_files = [f for f in results['files'] if f['filter'] == 1]
    for i, f in enumerate(preserve_files[:10]):
        size = format_size(f['size'])
        print(f"  [{i+1}] {f['path']} ({size}) - {f['reason']}")

    if len(preserve_files) > 10:
        print(f"  ... and {len(preserve_files) - 10} more files")

def save_results(results: Dict, output_file: str):
    """Save results to JSON file."""
    with open(output_file, 'w', encoding='utf-8') as f:
        json.dump(results, f, indent=2)
    print(f"\n✓ Full results saved to: {output_file}")

def delete_filtered_files(results: Dict, dry_run: bool = True):
    """Delete files marked with filter=0."""
    delete_files = [f for f in results['files'] if f['filter'] == 0]

    if not delete_files:
        print("\nNo files to delete.")
        return

    if dry_run:
        print(f"\n⚠️  DRY RUN: Would delete {len(delete_files)} files")
        print("Run with dry_run=False to actually delete files")
        return

    print(f"\n🗑️  Deleting {len(delete_files)} files...")
    deleted_count = 0
    error_count = 0

    for f in delete_files:
        full_path = os.path.join(os.getcwd(), f['path'])
        try:
            os.remove(full_path)
            deleted_count += 1
            if deleted_count % 10 == 0:
                print(f"  Deleted {deleted_count}/{len(delete_files)} files...")
        except Exception as e:
            error_count += 1
            print(f"  ✗ Error deleting {f['path']}: {e}")

    print(f"\n✓ Deletion complete: {deleted_count} deleted, {error_count} errors")

def main():
    repo_root = os.getcwd()

    print("Scanning repository...")
    results = scan_and_analyze(repo_root)

    # Print analysis
    print_analysis(results)

    # Save results
    output_file = os.path.join(repo_root, "file_filter_results.json")
    save_results(results, output_file)

    # Ask user before deleting
    print("\n" + "="*80)
    print("DELETION CONFIRMATION")
    print("="*80)
    print(f"Ready to delete {results['stats']['delete']} files marked with filter=0")
    print("This will be performed as a DRY RUN first.")

    # Perform dry run
    delete_filtered_files(results, dry_run=True)

if __name__ == "__main__":
    main()
