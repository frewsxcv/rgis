#!/bin/sh -ex

rm -rf www/dist/
# https://github.com/bevyengine/bevy/issues/9188
RUSTFLAGS=--cfg=web_sys_unstable_apis wasm-pack build --no-typescript --release rgis
(cd www && npm run build)
