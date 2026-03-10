import { test } from "./fixtures/app-fixture";

const geotiffFiles = [
  "rasterio_generated/fixtures/antimeridian.tif",
  // Internal mask IFD not supported by async-tiff
  // "rasterio_generated/fixtures/cog_uint8_rgb_mask.tif",
  "rasterio_generated/fixtures/cog_uint8_rgb_nodata.tif",
  "rasterio_generated/fixtures/cog_uint8_rgba.tif",
  "rasterio_generated/fixtures/custom_crs.tif",
  // LERC, JXL, and WebP codecs require C dependencies incompatible with wasm32
  // "rasterio_generated/fixtures/float32_1band_lerc_block32.tif",
  // "rasterio_generated/fixtures/float32_1band_lerc_deflate_block32.tif",
  // "rasterio_generated/fixtures/float32_1band_lerc_zstd_block32.tif",
  // ZSTD codec not enabled in async-tiff features
  // "rasterio_generated/fixtures/int8_3band_zstd_block64.tif",
  "rasterio_generated/fixtures/pixel_as_point.tif",
  "rasterio_generated/fixtures/uint16_1band_lzw_block128_predictor2.tif",
  "rasterio_generated/fixtures/uint16_1band_scale_offset.tif",
  "rasterio_generated/fixtures/uint8_1band_and_alpha_deflate_block64_cog.tif",
  "rasterio_generated/fixtures/uint8_1band_deflate_block128_unaligned.tif",
  // "rasterio_generated/fixtures/uint8_1band_jxl_block64.tif",
  "rasterio_generated/fixtures/uint8_1band_lzma_block64.tif",
  "rasterio_generated/fixtures/uint8_1band_lzw_block64_predictor2.tif",
  "rasterio_generated/fixtures/uint8_nonrgb_deflate_block64_cog.tif",
  "rasterio_generated/fixtures/uint8_rgb_deflate_block64_cog.tif",
  // "rasterio_generated/fixtures/uint8_rgb_webp_block64_cog.tif",
  // "rasterio_generated/fixtures/uint8_rgba_webp_block64_cog.tif",
  "real_data/eox/eox_cloudless.tif",
  "real_data/hot-oam/68077a72c46a9912474701ef.tif",
  "real_data/nlcd/nlcd_landcover.tif",
  // JPEG codec not enabled in async-tiff features
  // "real_data/rio-tiler/cog_rgb_with_stats.tif",
  // Stripped (non-tiled) TIFFs not supported by async-tiff
  // "real_data/rio-tiler/non-tiled.tif",
  // "real_data/source-coop-alpha-earth/xjejfvrbm1fbu1ecw-0000000000-0000008192.tif",
  "real_data/umbra/sydney_airport_GEC.tif",
  "real_data/vantor/maxar_opendata_yellowstone_visual.tif",
];

function snapshotName(filePath: string): string {
  return filePath.replace(/\//g, "-").replace(/\.tif$/, "") + ".png";
}

test("load eox_cloudless with Countries overlay", async ({ appPage }) => {
  test.setTimeout(120000);

  // Load the raster layer first
  await appPage.loadGeoTIFFFile("./dist/geotiff-test-data/real_data/eox/eox_cloudless.tif");

  // Now add the Countries library layer on top
  await appPage.addLibraryLayer("World", "Countries");

  await appPage.expectScreenshot(
    "real-data-eox-eox-cloudless-with-countries.png",
  );
});

for (const filePath of geotiffFiles) {
  test(`load GeoTIFF: ${filePath}`, async ({ appPage }) => {
    test.setTimeout(60000);

    await appPage.loadGeoTIFFFile(`./dist/geotiff-test-data/${filePath}`);

    // Remote raster data can have minor non-deterministic rendering differences
    const options = filePath.startsWith("real_data/")
      ? { maxDiffPixelRatio: 0.05 }
      : undefined;
    await appPage.expectScreenshot(snapshotName(filePath), options);
  });
}
