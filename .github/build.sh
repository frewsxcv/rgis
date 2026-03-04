#!/bin/sh -ex

rm -rf www/dist/
rm -rf rgis/pkg

if [ "$1" = "--release" ]; then
  PROFILE_FLAG="--release"
  PROFILE_DIR="release"
else
  PROFILE_FLAG=""
  PROFILE_DIR="debug"
fi

# https://github.com/bevyengine/bevy/issues/9188
# -Zinline-mir=no: workaround for https://github.com/rust-lang/rust/issues/131960
RUSTFLAGS="$RUSTFLAGS --cfg=web_sys_unstable_apis -Zinline-mir=no" cargo build $PROFILE_FLAG --target wasm32-unknown-unknown -p rgis --lib
wasm-bindgen --out-dir rgis/pkg --target web --no-typescript "target/wasm32-unknown-unknown/$PROFILE_DIR/rgis.wasm"
(cd www && npm run build)
