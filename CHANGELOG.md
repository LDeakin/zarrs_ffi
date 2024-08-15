# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
 - Add `Cargo.lock` to version control

## [0.8.2] - 2024-08-13

### Added
 - Add `zarrsArraySetAttributes`

### Changed
 - Bump `cbindgen` to 0.27
 - Change `zarrs{LastError,ArrayGetMetadataString,ArrayGetAttributesString}` to return non-const pointers

### Fixed
 - Bump MSRV to 1.75 (needed since 0.6.1)

## [0.8.1] - 2024-08-05

### Add
 - Add `Dockerfile` for cmake example
 - Add `all_codecs` feature

### Changed 
 - Fetch deps at configure time in `Findzarrs.cmake`

## [0.8.0] - 2024-08-05

### Add
 - Add `zarrs::array::ArrayShardedReadableExtCache` related functionality
   - `ZarrsShardIndexCache` type
   - `zarrsArrayGetInnerChunkShape()` and `zarrsArrayGetInnerChunkGridShape()`
   - `zarrsCreateShardIndexCache()` and `zarrsDestroyShardIndexCache()`
   - `zarrsArrayRetrieveInnerChunk` and `zarrsArrayRetrieveSubsetSharded()`
 - Add `zarrsFreeString`
 - Add `zarrsArrayGetMetadataString` and `zarrsArrayGetAttributesString`
 - Add `ZarrsResult:ZARRS_ERROR_UNSUPPORTED_DATA_TYPE`

### Changed
 - Bump `zarrs` to 0.16.2
 - Mark `ZarrsResult` as non-exhaustive

### Fixed
 - Add missing CMake library alias `zarrs::zarrs-static` for `zarrs_ffi-static`
 - Set CMake `target_include_directories` for `zarrs_ffi-static`

## [0.7.0] - 2024-07-07

### Changed
 - Make `cbindgen` dependency optional
 - **Breaking**: Rename `zarrsCreateArrayRWWithMetadata` to `zarrsCreateArrayRW` and `zarrsCreateArrayRW` to `zarrsOpenArrayRW`
 - Bump `zarrs` to 0.15.0

## [0.6.2] - 2024-04-20

### Fixed
 - Add new functions to `lib.rs` so they are visible in crate docs

## [0.6.1] - 2024-04-20

### Added
 - `zarrsArrayGetChunkShape()`
 - `zarrsArrayGetChunkOrigin()`

### Changed
 - Bump `zarrs` to 0.13

## [0.6.0] - 2024-04-03

### Added
 - `zarrsArrayGetDimensionality()`
 - `zarrsArrayGetShape()`
 - `zarrsArrayGetDataType()` and `ZarrsDataType`
 - `zarrsArrayGetChunkGridShape()`
 - `zarrsArrayGetChunksInSubset()`

### Changed
 - Change `subset_shape` in `zarrsArrayGetSubsetSize` to `*u64`
 - Reorder parameters to various functions so counts come first
 - Rename various function parameters to camel case, add `p` prefix to all pointers
 - Use `find_package_handle_standard_args` in `Findzarrs.cmake`

### Fixed
 - Fixed various function docs referring to non-existent parameters
 - Add more safety docs

## [0.5.1] - 2024-03-18

### Changed
 - Use `tempfile` rather than `tempdir` and move to dev dependency
 - Remove `derive_more` and `serde_json` dependency
 - Generalise `Findzarrs.cmake` and reference in `README.md`

### Fixed
 - Fixed link to examples in `README.md`

## [0.5.0] - 2024-03-10

### Added
 - `cbindgen` feature to generate `zarrs.h` in the source directory
   - `zarrs.h` is now version controlled in the source directory rather than placed in the build directory
 - Add `examples/cmake_project` demonstrating using `zarrs_ffi` in a `CMake` project

### Changed
 - Rename package to `zarrs_ffi` from `zarrs-ffi` and move repository
 - `zarrsDestroyArray` and `zarrsDestroyStorage` now return a `ZarrsResult`
 - Set MSRV to 1.71

## [0.4.0] - 2024-03-09

### Added
 - Add `zarrs_assert`

### Changed
 - Remove `ZarrsStorageRW` and add `ZarrsStorage` that can hold any type of storage
 - Remove `ZarrsArrayRW` and add `ZarrsArray` that can hold any type of array

## [0.3.0] - 2024-02-23

### Added
 - Add `examples/array_write_read.cpp`

### Changed
 - Bump `zarrs` to 0.12
 - Move `C++` test code into separate files
 - Cleanup `README.md`

## [0.2.1] - 2024-01-17

### Changed
 - Update to [`zarrs`](https://github.com/LDeakin/zarrs) 0.10.0
 - Update `cbindgen` to 0.26

## [0.2.0] - 2023-09-25

### Added
 - Initial public release

[unreleased]: https://github.com/LDeakin/zarrs_ffi/compare/v0.8.2...HEAD
[0.8.2]: https://github.com/LDeakin/zarrs_ffi/releases/tag/v0.8.2
[0.8.1]: https://github.com/LDeakin/zarrs_ffi/releases/tag/v0.8.1
[0.8.0]: https://github.com/LDeakin/zarrs_ffi/releases/tag/v0.8.0
[0.7.2]: https://github.com/LDeakin/zarrs_ffi/releases/tag/v0.7.2
[0.7.1]: https://github.com/LDeakin/zarrs_ffi/releases/tag/v0.7.1
[0.7.0]: https://github.com/LDeakin/zarrs_ffi/releases/tag/v0.7.0
[0.6.2]: https://github.com/LDeakin/zarrs_ffi/releases/tag/v0.6.2
[0.6.1]: https://github.com/LDeakin/zarrs_ffi/releases/tag/v0.6.1
[0.6.0]: https://github.com/LDeakin/zarrs_ffi/releases/tag/v0.6.0
[0.5.1]: https://github.com/LDeakin/zarrs_ffi/releases/tag/v0.5.1
[0.5.0]: https://github.com/LDeakin/zarrs_ffi/releases/tag/v0.5.0
[0.4.0]: https://github.com/LDeakin/zarrs_ffi/releases/tag/v0.4.0
[0.3.0]: https://github.com/LDeakin/zarrs_ffi/releases/tag/v0.3.0
[0.2.1]: https://github.com/LDeakin/zarrs_ffi/releases/tag/v0.2.1
[0.2.0]: https://github.com/LDeakin/zarrs_ffi/releases/tag/v0.2.0
