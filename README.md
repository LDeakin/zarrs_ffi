# zarrs_ffi

[![Latest Version](https://img.shields.io/crates/v/zarrs_ffi.svg)](https://crates.io/crates/zarrs_ffi)
[![zarrs documentation](https://docs.rs/zarrs_ffi/badge.svg)](https://docs.rs/zarrs_ffi)
![msrv](https://img.shields.io/crates/msrv/zarrs_ffi)
[![build](https://github.com/LDeakin/zarrs_ffi/actions/workflows/ci.yml/badge.svg)](https://github.com/LDeakin/zarrs_ffi/actions/workflows/ci.yml)

FFI bindings for the [zarrs] crate, a rust library for the [Zarr V3](https://zarr.dev) storage format for multidimensional arrays and metadata.

Currently `zarrs_ffi` only supports a small subset of the [zarrs] API.

A changelog can be found [here](https://github.com/LDeakin/zarrs_ffi/blob/main/CHANGELOG.md).
Example usage can be found in the [examples](https://github.com/LDeakin/zarrs_ffi/tree/main/examples).

## CMake Quickstart
1. Install the rust compiler (and cargo).
2. Put [Findzarrs.cmake](https://github.com/LDeakin/zarrs_ffi/blob/main/examples/cmake_project/Findzarrs.cmake) in your `CMAKE_MODULE_PATH`
3. `find_package(zarrs 0.6 REQUIRED COMPONENTS zarrs/bz2)`
   - `zarrs` is retrieved from `GitHub` using [FetchContent](https://cmake.org/cmake/help/latest/module/FetchContent.html) and built using [corrosion](https://github.com/corrosion-rs/corrosion)
   - Components are optional zarrs codecs
4. the `zarrs` FFI library is available as the `zarrs::zarrs` target

A complete `CMake` example can be found in [examples/cmake_project](https://github.com/LDeakin/zarrs_ffi/tree/main/examples/cmake_project).

## Manual Build

#### Basic Build
Building generates a header, and a platform-dependent static and dynamic library.
```bash
cargo build --release --features cbindgen # -> zarrs.h and target/release/[lib]zarrs_ffi{.a,.so,.dll,.dylib}
```
`zarrs.h` is only re-generated if the `cbindgen` feature is enabled.

#### Enabling SIMD intrinsics
Encoding and decoding performance may be improved with `avx2`/`sse2` enabled (if supported).
Compile with either of:
 - `RUSTFLAGS="-C target-cpu=native"`
 - `RUSTFLAGS="-C target-feature=+avx2,+sse2"`

#### Enabling non-default `zarrs` codecs
Non-default `zarrs` codecs (see [`zarrs` crate features](https://docs.rs/zarrs/latest/zarrs/#crate-features)) can be enabled by passing them as feature flags.
For example:
```bash
cargo build --release --features cbindgen --features zarrs/bitround,zarrs/zfp,zarrs/bz2,zarrs/pcodec
```

## Licence
`zarrs_ffi` is licensed under either of
 - the Apache License, Version 2.0 [LICENSE-APACHE](./LICENCE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0> or
 - the MIT license [LICENSE-MIT](./LICENCE-MIT) or <http://opensource.org/licenses/MIT>, at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[zarrs]: https://github.com/LDeakin/zarrs
