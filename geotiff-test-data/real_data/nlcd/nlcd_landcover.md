# NLCD Landcover

Highlights:

- Paletted
- Custom CRS (not defined by EPSG code)

## Data creation

From

```
s3://usgs-landcover/annual-nlcd/c1/v1/cu/mosaic/Annual_NLCD_LndCov_2024_CU_C1V1.tif
```

```bash
AWS_REQUEST_PAYER=requester pixi run gdal_translate \
  -srcwin 0 0 256 256 \
  -of COG \
  -co BLOCKSIZE=64 \
  /vsis3/usgs-landcover/annual-nlcd/c1/v1/cu/mosaic/Annual_NLCD_LndCov_2024_CU_C1V1.tif \
  nlcd_landcover.tif
```
