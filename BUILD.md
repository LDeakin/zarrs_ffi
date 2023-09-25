## Building
Building generates a header, static library, and dynamic library.
```bash
cargo build --release # -> target/release/{zarrs.h,zarrs_ffi.a,zarrs_ffi.so}
```

## Testing
```bash
# Must have no warnings/errors to pass CI
cargo build && \
cargo test && \
cargo doc && \
cargo fmt --all -- --check && \
cargo clippy -- -D warnings
```
