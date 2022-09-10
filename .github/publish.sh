#!/bin/sh -ex

rm -rf www/dist/
wasm-pack build --release rgis
(cd www && npm run build)
