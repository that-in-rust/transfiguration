#!/usr/bin/env python3
"""
Flatten docs directory - move all nested files to root and delete subdirectories.
"""

import os
import shutil
from pathlib import Path

def flatten_directory(root_dir):
    """
    Move all files from subdirectories to root, then delete empty directories.
    """
    root_path = Path(root_dir)
    moved_files = []

    print(f"Flattening directory: {root_dir}")

    # Find all files in subdirectories
    all_files = []
    for file_path in root_path.rglob('*'):
        if file_path.is_file():
            # Skip files already at root
            if file_path.parent != root_path:
                all_files.append(file_path)

    print(f"Found {len(all_files)} files in subdirectories to flatten")

    # Move each file to root with path-based naming
    for file_path in all_files:
        # Get relative path from docs root
        rel_path = file_path.relative_to(root_path)

        # Create new flat filename by replacing / with __
        flat_name = str(rel_path).replace('/', '__')

        # New destination
        dest_path = root_path / flat_name

        # Handle conflicts (shouldn't happen with path prefixing)
        if dest_path.exists():
            base, ext = os.path.splitext(flat_name)
            counter = 1
            while dest_path.exists():
                flat_name = f"{base}_{counter}{ext}"
                dest_path = root_path / flat_name
                counter += 1

        # Move file
        try:
            shutil.move(str(file_path), str(dest_path))
            moved_files.append((str(rel_path), flat_name))
            if len(moved_files) % 50 == 0:
                print(f"  Moved {len(moved_files)} files...")
        except Exception as e:
            print(f"  Error moving {file_path}: {e}")

    print(f"\nMoved {len(moved_files)} files to root")

    # Show sample of moved files
    print("\nSample of flattened files:")
    for orig, new in moved_files[:10]:
        print(f"  {orig} → {new}")

    # Now delete all empty directories
    print(f"\nDeleting empty subdirectories...")
    deleted_dirs = []

    # Sort by depth (deepest first) to delete from bottom up
    all_dirs = sorted([d for d in root_path.rglob('*') if d.is_dir()],
                     key=lambda p: len(p.parts), reverse=True)

    for dir_path in all_dirs:
        try:
            # Only delete if empty
            if not any(dir_path.iterdir()):
                dir_path.rmdir()
                deleted_dirs.append(str(dir_path.relative_to(root_path)))
        except Exception as e:
            print(f"  Could not delete {dir_path}: {e}")

    print(f"Deleted {len(deleted_dirs)} empty directories")

    return moved_files, deleted_dirs

def main():
    docs_dir = os.path.join(os.getcwd(), 'docs')

    if not os.path.exists(docs_dir):
        print(f"Error: {docs_dir} does not exist")
        return

    moved, deleted = flatten_directory(docs_dir)

    print("\n" + "="*80)
    print("FLATTENING COMPLETE")
    print("="*80)
    print(f"Files moved: {len(moved)}")
    print(f"Directories deleted: {len(deleted)}")
    print(f"\nAll files are now in: {docs_dir}")

if __name__ == "__main__":
    main()
