
set -e

#rustup target add wasm32-unknown-unknown
rm -rf target/itch.io
mkdir target/itch.io
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --out-name bevy_game --out-dir target/itch.io --target web target/wasm32-unknown-unknown/release/test_ldtk_custom_material.wasm
cp -r assets target/itch.io/
cp -r wasm/* target/itch.io/
cd target/itch.io/
zip --recurse-paths ../test.zip .
cd ../..
#butler push target/test.zip bdavier/test-template:wasm