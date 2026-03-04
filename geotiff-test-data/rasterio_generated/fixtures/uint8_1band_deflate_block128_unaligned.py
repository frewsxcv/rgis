"""Generate a uint8 COG with DEFLATE compression and unaligned tile boundaries.

Similar to:
https://github.com/developmentseed/titiler/blob/1.1.1/src/titiler/mosaic/tests/fixtures/cog.tif
"""

from pathlib import Path

import numpy as np
from rasterio.transform import from_origin

from rasterio_generated.write_utils import write_cog


def generate(output_path: Path) -> None:
    width = 265
    height = 266

    # Create gradient data to have meaningful pixel values
    y = np.linspace(0, 255, height, dtype=np.uint8)
    x = np.linspace(0, 255, width, dtype=np.uint8)
    data = ((y[:, np.newaxis].astype(np.float64) + x[np.newaxis, :]) / 2).astype(
        np.uint8
    )

    # UTM zone 21N projection with 100m resolution (similar to original)
    transform = from_origin(373185.0, 8286015.0, 100.0, 100.0)

    write_cog(
        output_path,
        data,
        blocksize=128,
        compress="DEFLATE",
        crs="EPSG:32621",
        transform=transform,
    )
