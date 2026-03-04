"""Generate an RGB GeoTIFF with DEFLATE compression."""

from pathlib import Path

import numpy as np
from rasterio.enums import ColorInterp

from rasterio_generated.write_utils import write_cog


def generate(output_path: Path) -> None:
    """Generate a 256x256 RGB GeoTIFF with DEFLATE compression."""
    # Create RGB gradient pattern
    r = np.linspace(0, 127, 128, dtype=np.uint8)
    g = np.linspace(127, 0, 128, dtype=np.uint8)
    b = np.full(128, 128, dtype=np.uint8)

    data = np.stack(
        [
            np.tile(r, (128, 1)),
            np.tile(g.reshape(-1, 1), (1, 128)),
            np.tile(b, (128, 1)),
        ]
    )

    write_cog(
        output_path,
        data,
        blocksize=64,
        compress="WEBP",
        colorinterp=[ColorInterp.red, ColorInterp.green, ColorInterp.blue],
    )
