//! FFI bindings for the [zarrs](https://github.com/LDeakin/zarrs) crate, a rust library for the [Zarr V3](https://zarr.dev) storage format for multidimensional arrays and metadata.
//!
//! Developed at the [Department of Materials Physics](https://physics.anu.edu.au/research/mp/), Australian National University, Canberra, Australia.
//!
//! **zarrs and zarrs-ffi are experimental and in limited production use. Use at your own risk!**
//!
//! ## Licence
//! `zarrs-ffi` is licensed under either of
//!  - the Apache License, Version 2.0 [LICENSE-APACHE](./LICENCE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0> or
//!  - the MIT license [LICENSE-MIT](./LICENCE-MIT) or <http://opensource.org/licenses/MIT>, at your option.
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::ffi::{c_char, CString};

use once_cell::sync::Lazy;

extern crate zarrs;

pub mod array;
pub mod storage;
pub mod version;

#[repr(i32)]
pub enum ZarrsResult {
    ZARRS_SUCCESS = 0,
    ZARRS_ERROR_NULL_PTR = -1,
    ZARRS_ERROR_STORAGE = -2,
    ZARRS_ERROR_ARRAY = -3,
    ZARRS_ERROR_BUFFER_LENGTH = -4,
    ZARRS_ERROR_INVALID_INDICES = -5,
    ZARRS_ERROR_NODE_PATH = -6,
    ZARRS_ERROR_STORE_PREFIX = -7,
    ZARRS_ERROR_INVALID_METADATA = -8,
}

static mut LAST_ERROR: Lazy<String> = Lazy::new(|| "".to_string());

/// Get the last error string.
#[no_mangle]
pub extern "C" fn zarrsLastError() -> *const c_char {
    let c_str = CString::new(unsafe { LAST_ERROR.as_str() }).unwrap();
    c_str.into_raw()
}
