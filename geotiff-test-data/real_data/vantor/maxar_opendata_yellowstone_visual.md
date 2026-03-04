## Repro

From
```
https://maxar-opendata.s3.amazonaws.com/events/yellowstone-flooding22/ard/12/120000020112/2022-06-18/10300100D51B8C00-visual.tif
```

```bash
pixi run gdal_translate \
  -srcwin 0 0 128 128 \
  -of COG \
  -co COMPRESS=JPEG \
  -co BLOCKSIZE=64 \
  /vsicurl/https://maxar-opendata.s3.amazonaws.com/events/yellowstone-flooding22/ard/12/120000020112/2022-06-18/10300100D51B8C00-visual.tif \
  maxar_opendata_yellowstone_visual.tif
```
