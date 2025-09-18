cargo build --target wasm32-unknown-unknown --release -p backend $BUILD_FLAGS
candid-extractor target/wasm32-unknown-unknown/release/backend.wasm > src/backend/backend.did