# Making it work with new Next.js versions

Most of the time the only change needed is to update Cargo.toml to have the same swc_code version
as the target Next.js version in this file:

https://github.com/vercel/next.js/blob/v15.1.6/Cargo.toml

# Running tests

```bash
cargo test --release
```

# Testing with Next.js before releasing

1. Build the binary with:

```bash
cargo build --release --target=wasm32-wasip1
```

2. Copy the binary to Next.js test project:

```bash
cp target/wasm32-wasip1/release/swc_plugin_typed_redux_saga.wasm <test_project>/node_modules/swc-plugin-typed-redux-saga/target/wasm32-wasip1/release/swc_plugin_typed_redux_saga.wasm
```

3. Run your Next.js test project and see if it crashes