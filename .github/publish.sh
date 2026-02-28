#!/bin/sh -ex

rm -rf www/dist/
rm -rf rgis/pkg
# https://github.com/bevyengine/bevy/issues/9188
# -Zinline-mir=no: workaround for https://github.com/rust-lang/rust/issues/131960
RUSTFLAGS="$RUSTFLAGS --cfg=web_sys_unstable_apis -Zinline-mir=no" cargo build --release --target wasm32-unknown-unknown -p rgis
wasm-bindgen --out-dir rgis/pkg --target web --no-typescript "target/wasm32-unknown-unknown/release/rgis.wasm"
(cd www && npm run build)
