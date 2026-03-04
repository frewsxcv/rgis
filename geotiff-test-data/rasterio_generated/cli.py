"""CLI for generating test GeoTIFF files."""

import importlib.util
import sys
from pathlib import Path


def main() -> None:
    """Generate all test GeoTIFF files."""
    package_dir = Path(__file__).parent
    fixtures_dir = package_dir / "fixtures"
    output_dir = fixtures_dir
    output_dir.mkdir(exist_ok=True)

    # Find all fixture Python files
    fixture_files = sorted(fixtures_dir.glob("*.py"))
    fixture_files = [f for f in fixture_files if f.name != "__init__.py"]

    if not fixture_files:
        print("No fixture files found!", file=sys.stderr)
        sys.exit(1)

    print(f"Found {len(fixture_files)} fixture(s)")
    print(f"Output directory: {output_dir.absolute()}\n")

    for fixture_path in fixture_files:
        # Import the module
        module_name = f"rasterio_generated.fixtures.{fixture_path.stem}"
        spec = importlib.util.spec_from_file_location(module_name, fixture_path)
        if spec is None or spec.loader is None:
            raise ImportError(f"Could not load {fixture_path}")

        module = importlib.util.module_from_spec(spec)
        sys.modules[module_name] = module
        spec.loader.exec_module(module)

        # Call generate function with output path matching the script name
        if hasattr(module, "generate"):
            output_path = output_dir / f"{fixture_path.stem}.tif"
            module.generate(output_path)
            print(f"✓ Generated: {output_path}")
        else:
            raise ValueError(
                f"Module {module_name} does not have a generate() function"
            )

    print(f"\nComplete! Generated files in {output_dir}/")


if __name__ == "__main__":
    main()
