"""Generate a float32 GeoTIFF with LERC compression."""

from pathlib import Path

import numpy as np

from rasterio_generated.write_utils import write_cog

HERE = Path(__file__).parent


def generate(output_path: Path) -> None:
    """Generate a 128x128 tiled float32 GeoTIFF with LERC compression."""
    data = np.linspace(0.0, 1.0, 128 * 128, dtype=np.float32).reshape(128, 128)

    write_cog(
        output_path,
        data,
        blocksize=64,
        compress="LERC",
    )
