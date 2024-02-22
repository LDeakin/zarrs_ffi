## Testing
```bash
# Must have no warnings/errors to pass CI
cargo build && \
cargo test && \
cargo doc && \
cargo fmt --all -- --check && \
cargo clippy -- -D warnings
```
