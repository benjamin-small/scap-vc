# Testing

Run the deterministic library gate with:

```sh
cargo fmt --all --check
cargo clippy --all-targets --all-features
cargo test --all-features
```

The automated tests cover removal of alpha bytes, BGRA-to-RGB channel conversion, row cropping for valid frame dimensions, and preservation of input when dimensions do not match the buffer. The dedicated integration test exercises those public frame-conversion functions through the crate API.

Instrumentation-based line and branch coverage is currently 0% because CI does not configure a Rust coverage reporter. Hardware-backed capture startup, permissions, live frames, cross-process windows, and platform-specific engines require OS integration tests and are not represented by the deterministic unit suite.
