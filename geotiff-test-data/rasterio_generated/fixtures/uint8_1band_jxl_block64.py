"""Generate a basic uint8 GeoTIFF."""

from pathlib import Path

import numpy as np

from rasterio_generated.write_utils import write_cog

HERE = Path(__file__).parent


def generate(output_path: Path) -> None:
    """Generate a basic 256x256 uint8 GeoTIFF wdith gradient pattern."""
    data = np.arange(128, dtype=np.uint8).reshape(1, 128)
    data = np.repeat(data, 128, axis=0)

    write_cog(
        output_path,
        data,
        blocksize=64,
        compress="JXL",
        nodata=0,
    )
