# Umbra Sydney Airport

## Features

- Rotated transform:

    ```json
    [
        -0.0000025955530347975644,
        -0.0000021620279292929635,
        150.7563842406136,
        -0.0000018031258572430161,
        0.0000021646845467054394,
        -33.8895671886414,
    ]
    ```

## Misc

```
aws s3 cp --no-sign-request 's3://umbra-open-data-catalog/sar-data/tasks/Sydney International Airport, Australia/d1378053-398d-4ee6-87ab-e212c9b1aeb2/2024-05-11-23-06-40_UMBRA-04/2024-05-11-23-06-40_UMBRA-04_METADATA.json' sydney_airport.json
```


```bash
pixi run gdal_translate \
  -srcwin 0 0 512 512 \
  -of COG \
  "/vsis3/umbra-open-data-catalog/sar-data/tasks/Sydney International Airport, Australia/d1378053-398d-4ee6-87ab-e212c9b1aeb2/2024-05-11-23-06-40_UMBRA-04/2024-05-11-23-06-40_UMBRA-04_GEC.tif" \
  sydney_airport_GEC.tif
```

For future reference, there are more TIFF images from the same directory:

> aws s3 ls --no-sign-request 's3://umbra-open-data-catalog/sar-data/tasks/Sydney International Airport, Australia/d1378053-398d-4ee6-87ab-e212c9b1aeb2/2024-05-11-23-06-40_UMBRA-04/'
2024-05-12 04:39:49  768106649 2024-05-11-23-06-40_UMBRA-04_CSI-SIDD.nitf
2024-05-12 04:39:46  604071003 2024-05-11-23-06-40_UMBRA-04_CSI.tif
2024-05-12 04:39:30  277018485 2024-05-11-23-06-40_UMBRA-04_GEC.tif
2024-05-12 04:39:28       3699 2024-05-11-23-06-40_UMBRA-04_METADATA.json
2024-05-12 04:39:32 4304478921 2024-05-11-23-06-40_UMBRA-04_SICD.nitf
2024-05-12 04:39:29  256042603 2024-05-11-23-06-40_UMBRA-04_SIDD.nitf
