#!/bin/sh -ex

rm -rf www/dist/
rm -rf rgis/pkg
# https://github.com/bevyengine/bevy/issues/9188
RUSTFLAGS=--cfg=web_sys_unstable_apis cargo build --release --target wasm32-unknown-unknown -p rgis
wasm-bindgen --out-dir rgis/pkg --target web --no-typescript "target/wasm32-unknown-unknown/release/rgis.wasm"
(cd www && npm run build)
