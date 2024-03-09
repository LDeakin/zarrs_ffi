# zarrs-ffi &emsp; [![build](https://github.com/LDeakin/zarrs-ffi/actions/workflows/ci.yml/badge.svg)](https://github.com/LDeakin/zarrs-ffi/actions/workflows/ci.yml)

FFI bindings for the [zarrs] crate, a rust library for the [Zarr V3](https://zarr.dev) storage format for multidimensional arrays and metadata.

Developed at the [Department of Materials Physics](https://physics.anu.edu.au/research/mp/), Australian National University, Canberra, Australia.

**zarrs and zarrs-ffi are experimental and in limited production use. Use at your own risk!**

Example usage can be found in the [examples](./examples).

> [!NOTE]
> Currently `zarrs-ffi` only supports a small subset of the [zarrs] API.

## Install

### Basic Build
Building generates a header, static library, and dynamic library.
```bash
cargo build --release --features cbindgen # -> zarrs.h and target/release/[lib]{zarrs.a,zarrs.so,zarrs.dll,zarrs.dylib}
```

### Enabling SIMD intrinsics
Encoding and decoding performance may be improved with `avx2`/`sse2` enabled (if supported).

This can be enabled by compiling with either of:
 - `RUSTFLAGS="-C target-cpu=native"`
 - `RUSTFLAGS="-C target-feature=+avx2,+sse2"`

### Enabling non-default `zarrs` codecs
Non-default `zarrs` codecs (see [`zarrs` crate features](https://docs.rs/zarrs/latest/zarrs/#crate-features)) can be enabled by passing them as feature flags.

For example:
```bash
cargo build --release --features cbindgen --features zarrs/bitround,zarrs/zfp,zarrs/bz2,zarrs/pcodec
```

## Licence
`zarrs-ffi` is licensed under either of
 - the Apache License, Version 2.0 [LICENSE-APACHE](./LICENCE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0> or
 - the MIT license [LICENSE-MIT](./LICENCE-MIT) or <http://opensource.org/licenses/MIT>, at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[zarrs]: https://github.com/LDeakin/zarrs
