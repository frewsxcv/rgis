# rgis

Geospatial data viewer written in Rust. Availble on the web ([rgis.app](https://rgis.app)) or natively on desktop.

## License

rgis is released under [The Anti-Capitalist Software License (version 1.4)](https://anticapitalist.software/).

## Install

Install `proj` (e.g. `brew intall proj`).

```sh
cargo install --git https://github.com/frewsxcv/rgis
```

## Usage

Render GeoJSON files:

```sh
rgis file1.geojson file2.geojson
```

Read GeoJSON files from standard in:

```sh
cat file1.geojson | rgis
```

Print help information:

```sh
rgis --help
```

## Screenshots

<img width="1392" alt="Screen Shot 2021-06-13 at 9 56 42 PM" src="https://user-images.githubusercontent.com/416575/121830120-50ee0600-cc92-11eb-8e0c-3a26fbdbec75.png">
