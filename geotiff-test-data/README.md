# geotiff-test-data

Test data for GeoTIFF parsers

## Installation

This project uses [pixi](https://pixi.sh) for dependency management.

```bash
pixi install
```

## Usage

### Generate all test files

```bash
pixi run generate
```

This will execute all generator scripts in [geotiff_test_data/generators/](geotiff_test_data/generators/) and save the output GeoTIFF files next to each Python file, with the same name but a `.tif` extension.

## Adding new test cases

To add a new test case:

1. Create a new Python file in `geotiff_test_data/fixtures/`
2. Implement a `generate(output_path: Path)` function that creates the GeoTIFF

Run `pixi run generate` and `pixi run info`.

## Image Sources

Images in the `real_data/` folder are sourced from various open data programs. See the individual README files in each subfolder for details.
