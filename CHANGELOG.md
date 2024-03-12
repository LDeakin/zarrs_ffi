# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
 - Use `tempfile` rather than `tempdir` and move to dev dependency
 - Remove `derive_more` and `serde_json` dependency

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

[unreleased]: https://github.com/LDeakin/zarrs_ffi/compare/v0.5.0...HEAD
[0.5.0]: https://github.com/LDeakin/zarrs_ffi/releases/tag/v0.5.0
[0.4.0]: https://github.com/LDeakin/zarrs_ffi/releases/tag/v0.4.0
[0.3.0]: https://github.com/LDeakin/zarrs_ffi/releases/tag/v0.3.0
[0.2.1]: https://github.com/LDeakin/zarrs_ffi/releases/tag/v0.2.1
[0.2.0]: https://github.com/LDeakin/zarrs_ffi/releases/tag/v0.2.0
