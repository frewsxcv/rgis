from __future__ import annotations

from pathlib import Path
from typing import TYPE_CHECKING, Literal

import numpy as np
import rasterio
from rasterio.crs import CRS
from rasterio.enums import ColorInterp, Resampling
from rasterio.io import MemoryFile
from rasterio.rio.overview import get_maximum_overview_level
from rasterio.shutil import copy
from rasterio.transform import from_origin

if TYPE_CHECKING:
    from affine import Affine


def write_cog(
    path: Path,
    data: np.ndarray,
    *,
    mask: np.ndarray | None = None,
    blocksize: int = 256,
    compress: Literal[
        "CCITTFAX3",
        "CCITTFAX4",
        "CCITTRLE",
        "DEFLATE",
        "JPEG",
        "JPEG2000",
        "JXL",
        "LERC_DEFLATE",
        "LERC_ZSTD",
        "LERC",
        "LZMA",
        "LZW",
        "PACKBITS",
        "WEBP",
        "ZSTD",
    ]
    | None = None,
    interleave: Literal["pixel", "band", "tile"] = "pixel",
    crs: str | CRS = "EPSG:4326",
    nodata_type: Literal["nodata", "mask", "alpha"] | None = "nodata",
    transform: Affine = from_origin(0, 0, 0.01, 0.01),
    predictor: Literal[2, 3] | None = None,
    nodata: int | float | None = None,
    rasterio_env: dict[str, str | bool | int] | None = None,
    colorinterp: list[ColorInterp] | None = None,
):
    """Write a COG to disk.

    This function creates an in-memory GeoTIFF dataset, writes the provided data
    to it, builds overviews, and then copies it to the specified path as a COG.

    This was taken from rio-tiler:
    https://github.com/cogeotiff/rio-tiler/blob/766e7b0e72803731d5b05822d93c1dcdd19fbe08/tests/conftest.py

    Args:
        path: The path of the output COG.
        data: The numpy array data to write.
        mask: The mask array, if any. Defaults to None.
        blocksize: The block size for tiling. Defaults to 256.
        compress: The compression method to use. Defaults to None.
        crs: The coordinate reference system. Defaults to "EPSG:4326".
        transform: The affine transform. Defaults to from_origin(0, 0, 0.01, 0.01).
        predictor: The predictor to use for compression. Defaults to None.
        nodata: The nodata value to use. Defaults to None.
        rasterio_env: Parameters to set in the rasterio.Env context. E.g. you may want to set `{'GDAL_TIFF_INTERNAL_MASK': True}`. Defaults to None.
    """

    if data.ndim == 2:
        nband = 1
        height, width = data.shape
    elif data.ndim == 3:
        nband, height, width = data.shape
    else:
        raise ValueError("data must be 2D or 3D (bands, height, width)")

    if blocksize & (blocksize - 1) != 0:
        raise ValueError("blocksize must be a power of two")

    if blocksize < 64:
        raise ValueError("blocksize must be at least 64")

    src_profile = {
        "driver": "GTiff",
        "count": nband,
        "dtype": data.dtype,
        "height": height,
        "width": width,
        "crs": crs,
        "transform": transform,
    }

    if predictor is not None:
        src_profile["predictor"] = predictor

    if nodata is not None:
        src_profile["nodata"] = nodata

    if nodata_type == "nodata":
        src_profile["nodata"] = 0

    elif nodata_type == "alpha":
        src_profile["count"] = nband + 1

    env = {
        **(rasterio_env or {}),
        "GDAL_TIFF_OVR_BLOCKSIZE": str(blocksize),
    }
    with rasterio.Env(env):
        with MemoryFile() as memfile:
            with memfile.open(**src_profile) as mem:
                if colorinterp is not None:
                    assert len(colorinterp) == mem.count

                if nodata_type == "alpha" and mask is not None:
                    data = np.concatenate([data, mask])
                    assert (
                        colorinterp is not None and colorinterp[-1] == ColorInterp.alpha
                    )

                if colorinterp is not None:
                    mem.colorinterp = colorinterp

                # Write Data
                if nband == 1:
                    mem.write(data, 1)
                else:
                    mem.write(data)

                # Write Mask
                if nodata_type == "mask" and mask is not None:
                    mem.write_mask(mask[0])

                overview_level = get_maximum_overview_level(
                    mem.width, mem.height, minsize=blocksize
                )
                overviews = [2**j for j in range(1, overview_level + 1)]
                mem.build_overviews(overviews, Resampling.bilinear)

                cog_profile = {
                    "driver": "COG",
                    "interleave": interleave,
                    "compress": "DEFLATE",
                    "tiled": True,
                    "blocksize": blocksize,
                }

                if compress is not None:
                    cog_profile["compress"] = compress

                # Copy to output path
                copy(mem, path, copy_src_overviews=True, **cog_profile)
