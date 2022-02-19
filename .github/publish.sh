#!/bin/sh -ex

rm -rf www/dist/
wasm-pack build rgis
(cd www && npm run build)
