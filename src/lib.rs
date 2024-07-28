#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::ffi::{c_char, CString};

use once_cell::sync::Lazy;

extern crate zarrs;

mod array;
mod storage;
mod version;

pub use array::{
    array_read::{zarrsArrayRetrieveChunk, zarrsArrayRetrieveSubset},
    array_read_write::zarrsArrayStoreSubset,
    array_write::{zarrsArrayStoreChunk, zarrsArrayStoreMetadata},
    data_type::ZarrsDataType,
    zarrsArrayGetChunkGridShape, zarrsArrayGetChunkOrigin, zarrsArrayGetChunkShape,
    zarrsArrayGetChunkSize, zarrsArrayGetChunksInSubset, zarrsArrayGetDataType,
    zarrsArrayGetDimensionality, zarrsArrayGetShape, zarrsArrayGetSubsetSize, zarrsCreateArrayRW,
    zarrsDestroyArray, zarrsOpenArrayRW, ZarrsArray,
};
pub use storage::{zarrsCreateStorageFilesystem, zarrsDestroyStorage, ZarrsStorage};
pub use version::{zarrsVersion, zarrsVersionMajor, zarrsVersionMinor, zarrsVersionPatch};

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
    ZARRS_ERROR_STORAGE_CAPABILITY = -9,
    ZARRS_ERROR_UNKNOWN_CHUNK_GRID_SHAPE = -10,
    ZARRS_ERROR_UNKNOWN_INTERSECTING_CHUNKS = -11,
    ZARRS_ERROR_UNSUPPORTED_DATA_TYPE = -12,
}

static mut LAST_ERROR: Lazy<String> = Lazy::new(|| "".to_string());

/// Get the last error string.
#[no_mangle]
pub extern "C" fn zarrsLastError() -> *const c_char {
    let c_str = CString::new(unsafe { LAST_ERROR.as_str() }).unwrap();
    c_str.into_raw()
}
