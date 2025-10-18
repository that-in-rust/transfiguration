#!/usr/bin/env python3
"""
Parseltongue Component Builder
Automated build system using the build manifest JSON
"""

import json
import subprocess
import sys
import os
from pathlib import Path
from typing import Dict, List, Any

class ComponentBuilder:
    def __init__(self, manifest_path: str = "build-manifest.json"):
        with open(manifest_path, 'r') as f:
            self.manifest = json.load(f)

        self.base_path = Path(self.manifest["config"]["basePath"])
        self.output_dir = Path(self.manifest["config"]["outputDir"])

    def run_command(self, command: str, cwd: Path = None, description: str = "") -> bool:
        """Run a shell command and return success status"""
        try:
            print(f"🔨 {description}")
            print(f"   Command: {command}")

            result = subprocess.run(
                command.split(),
                cwd=cwd,
                capture_output=True,
                text=True,
                check=True
            )

            if result.stdout:
                print(f"   Output: {result.stdout.strip()}")
            return True

        except subprocess.CalledProcessError as e:
            print(f"❌ Failed: {description}")
            print(f"   Error: {e.stderr}")
            return False
        except Exception as e:
            print(f"❌ Error: {description}")
            print(f"   Exception: {e}")
            return False

    def build_component(self, component_name: str) -> bool:
        """Build a single component"""
        if component_name not in self.manifest["components"]:
            print(f"❌ Unknown component: {component_name}")
            return False

        component = self.manifest["components"][component_name]
        print(f"\n🏗️  Building component: {component_name}")
        print(f"   Description: {component['description']}")

        # Create crate structure
        crate_path = self.base_path / component_name
        if not self.run_command(
            f"cargo new {component_name} --lib",
            cwd=self.base_path,
            description=f"Create crate structure for {component_name}"
        ):
            return False

        # Add dependencies
        if "add-dependencies" in [step["name"] for step in component["buildSteps"]]:
            deps = []
            # Extract dependencies from features or build steps
            for step in component["buildSteps"]:
                if step["name"] == "add-dependencies":
                    # Parse cargo add commands to extract crate names
                    cmd_parts = step["command"].replace("cd "+component_name+" && ", "").split()
                    if "add" in cmd_parts:
                        deps.extend(cmd_parts[2:])  # Everything after "cargo add"

            if deps:
                dep_cmd = f"cd {component_name} && cargo add {' '.join(deps)}"
                if not self.run_command(
                    dep_cmd,
                    cwd=self.base_path,
                    description=f"Add dependencies for {component_name}"
                ):
                    return False

        # Run build steps
        for step in component["buildSteps"]:
            if step["name"] not in ["create-crate-structure", "add-dependencies"]:
                # For now, just run the command if it exists
                if "command" in step:
                    cwd = self.base_path / component_name if "cd" in step["command"] else self.base_path
                    if not self.run_command(
                        step["command"],
                        cwd=cwd,
                        description=step.get("description", step["name"])
                    ):
                        return False

        # Run tests
        for test_step in component["testSteps"]:
            if "command" in test_step:
                cwd = self.base_path / component_name if "cd" in test_step["command"] else self.base_path
                if not self.run_command(
                    test_step["command"],
                    cwd=cwd,
                    description=test_step.get("description", test_step["name"])
                ):
                    return False

        print(f"✅ Successfully built component: {component_name}")
        return True

    def build_all_components(self) -> bool:
        """Build all components in dependency order"""
        print("🚀 Starting full build process...")

        for component_name in self.manifest["buildOrder"]:
            if not self.build_component(component_name):
                print(f"❌ Failed to build {component_name}")
                return False

        print("🎉 All components built successfully!")
        return True

    def list_variants(self, component_name: str = None):
        """List available variants for components"""
        if component_name:
            if component_name in self.manifest["components"]:
                component = self.manifest["components"][component_name]
                print(f"\n🔧 Variants for {component_name}:")
                for variant_name, variant_desc in component.get("variants", {}).items():
                    print(f"   • {variant_name}: {variant_desc}")
            else:
                print(f"❌ Unknown component: {component_name}")
        else:
            print("\n🔧 Available component variants:")
            for comp_name, component in self.manifest["components"].items():
                if "variants" in component:
                    print(f"\n📦 {comp_name}:")
                    for variant_name, variant_desc in component["variants"].items():
                        print(f"   • {variant_name}: {variant_desc}")

    def show_iteration_strategies(self):
        """Show available iteration strategies"""
        strategies = self.manifest.get("iterationStrategies", {})

        print("\n🔄 Available iteration strategies:")
        for strategy_type, strategy_list in strategies.items():
            print(f"\n📋 {strategy_type}:")
            for strategy in strategy_list:
                print(f"   • {strategy['name']}: {strategy['description']}")

    def run_quality_checks(self):
        """Run quality checks on all components"""
        print("\n🔍 Running quality checks...")

        checks = self.manifest.get("qualityChecks", {})
        for check_name, check_command in checks.items():
            if not self.run_command(
                check_command,
                cwd=self.base_path,
                description=f"Quality check: {check_name}"
            ):
                return False

        print("✅ All quality checks passed!")
        return True

def main():
    if len(sys.argv) < 2:
        print("Usage: python build.py [command]")
        print("Commands:")
        print("  build [component]    - Build specific component or all components")
        print("  list-variants [component] - List variants for component(s)")
        print("  show-strategies      - Show iteration strategies")
        print("  quality-check        - Run quality checks")
        print("  help                 - Show this help")
        return

    command = sys.argv[1]
    builder = ComponentBuilder()

    if command == "build":
        component = sys.argv[2] if len(sys.argv) > 2 else None
        if component:
            success = builder.build_component(component)
        else:
            success = builder.build_all_components()
        sys.exit(0 if success else 1)

    elif command == "list-variants":
        component = sys.argv[2] if len(sys.argv) > 2 else None
        builder.list_variants(component)

    elif command == "show-strategies":
        builder.show_iteration_strategies()

    elif command == "quality-check":
        success = builder.run_quality_checks()
        sys.exit(0 if success else 1)

    elif command == "help":
        print("Parseltongue Component Builder")
        print("=============================")
        print("Build system for modular Parseltongue components")
        print()
        print("This tool uses build-manifest.json to:")
        print("• Build components in dependency order")
        print("• Support multiple variants per component")
        print("• Enable fast iteration on different approaches")
        print("• Run quality checks and tests")
        print()
        print("Examples:")
        print("  python build.py build                    # Build all components")
        print("  python build.py build system-detective   # Build specific component")
        print("  python build.py list-variants           # Show all variants")
        print("  python build.py show-strategies         # Show iteration strategies")

    else:
        print(f"❌ Unknown command: {command}")
        print("Use 'python build.py help' for usage information")
        sys.exit(1)

if __name__ == "__main__":
    main()
