"""Generate a tiled, LZW-compressed GeoTIFF."""

from pathlib import Path

import numpy as np

from rasterio_generated.write_utils import write_cog

HERE = Path(__file__).parent


def generate(output_path: Path) -> None:
    """Generate a 512x512 tiled uint16 GeoTIFF with LZW compression."""
    data = np.arange(128 * 128, dtype=np.uint16).reshape(128, 128)

    write_cog(
        output_path,
        data,
        blocksize=64,
        compress="LZW",
        predictor=2,
    )
