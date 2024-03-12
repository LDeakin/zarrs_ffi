pub mod array_read;
pub mod array_read_write;
pub mod array_write;

use ffi_support::FfiStr;
use zarrs::array::{Array, ArrayMetadata};

use crate::{
    storage::{ZarrsStorage, ZarrsStorageEnum},
    ZarrsResult, LAST_ERROR,
};

#[doc(hidden)]
#[allow(clippy::upper_case_acronyms)]
pub enum ZarrsArrayEnum {
    R(Array<dyn zarrs::storage::ReadableStorageTraits>),
    W(Array<dyn zarrs::storage::WritableStorageTraits>),
    L(Array<dyn zarrs::storage::ListableStorageTraits>),
    RL(Array<dyn zarrs::storage::ReadableListableStorageTraits>),
    RW(Array<dyn zarrs::storage::ReadableWritableStorageTraits>),
    RWL(Array<dyn zarrs::storage::ReadableWritableListableStorageTraits>),
}

macro_rules! array_fn {
    ($array:expr, $fn:ident ) => {
        match $array {
            ZarrsArrayEnum::R(array) => array.$fn(),
            ZarrsArrayEnum::W(array) => array.$fn(),
            ZarrsArrayEnum::L(array) => array.$fn(),
            ZarrsArrayEnum::RL(array) => array.$fn(),
            ZarrsArrayEnum::RW(array) => array.$fn(),
            ZarrsArrayEnum::RWL(array) => array.$fn(),
        }
    };
    ($array:expr, $fn:ident, $( $args:expr ),* ) => {
        match $array {
            ZarrsArrayEnum::R(array) => array.$fn($( $args ),*),
            ZarrsArrayEnum::W(array) => array.$fn($( $args ),*),
            ZarrsArrayEnum::L(array) => array.$fn($( $args ),*),
            ZarrsArrayEnum::RL(array) => array.$fn($( $args ),*),
            ZarrsArrayEnum::RW(array) => array.$fn($( $args ),*),
            ZarrsArrayEnum::RWL(array) => array.$fn($( $args ),*),
        }
    };
}

pub(crate) use array_fn;

#[doc(hidden)]
pub struct ZarrsArray_T(pub ZarrsArrayEnum);

impl std::ops::Deref for ZarrsArray_T {
    type Target = ZarrsArrayEnum;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// An opaque handle to a zarr array.
pub type ZarrsArray = *mut ZarrsArray_T;

/// Create a handle to an existing array (read/write capability).
///
/// `pArray` is a pointer to a handle in which the created [`ZarrsArray`] is returned.
///
/// # Safety
/// `pArray` must be a valid pointer to a [`ZarrsArray`] handle.
#[no_mangle]
pub unsafe extern "C" fn zarrsCreateArrayRW(
    storage: ZarrsStorage,
    path: FfiStr,
    pArray: *mut ZarrsArray,
) -> ZarrsResult {
    if storage.is_null() {
        *LAST_ERROR = "storage is null".to_string();
        return ZarrsResult::ZARRS_ERROR_NULL_PTR;
    }

    let storage = &**storage;

    if let ZarrsStorageEnum::RW(storage) = storage {
        match Array::new(storage.clone(), path.into()) {
            Ok(array) => {
                *pArray = Box::into_raw(Box::new(ZarrsArray_T(ZarrsArrayEnum::RW(array))));
                ZarrsResult::ZARRS_SUCCESS
            }
            Err(err) => {
                *LAST_ERROR = err.to_string();
                ZarrsResult::ZARRS_ERROR_ARRAY
            }
        }
    } else {
        *LAST_ERROR = "storage does not support read and write".to_string();
        ZarrsResult::ZARRS_ERROR_STORAGE_CAPABILITY
    }
}

/// Create a handle to a new array (read/write capability).
///
/// `metadata` is expected to be a JSON string representing a zarr V3 array `zarr.json`.
///
/// `pArray` is a pointer to a handle in which the created [`ZarrsArray`] is returned.
///
/// # Safety
/// `pArray` must be a valid pointer to a [`ZarrsArray`] handle.
#[no_mangle]
pub unsafe extern "C" fn zarrsCreateArrayRWWithMetadata(
    storage: ZarrsStorage,
    path: FfiStr,
    metadata: FfiStr,
    pArray: *mut ZarrsArray,
) -> ZarrsResult {
    if storage.is_null() {
        *LAST_ERROR = "storage is null".to_string();
        return ZarrsResult::ZARRS_ERROR_NULL_PTR;
    }

    let storage = &**storage;

    let metadata = match ArrayMetadata::try_from(metadata.as_str()) {
        Ok(metadata) => metadata,
        Err(err) => {
            *LAST_ERROR = err.to_string();
            return ZarrsResult::ZARRS_ERROR_INVALID_METADATA;
        }
    };

    if let ZarrsStorageEnum::RW(storage) = storage {
        match Array::new_with_metadata(storage.clone(), path.into(), metadata) {
            Ok(array) => {
                *pArray = Box::into_raw(Box::new(ZarrsArray_T(ZarrsArrayEnum::RW(array))));
                ZarrsResult::ZARRS_SUCCESS
            }
            Err(err) => {
                *LAST_ERROR = err.to_string();
                ZarrsResult::ZARRS_ERROR_ARRAY
            }
        }
    } else {
        *LAST_ERROR = "storage does not support read and write".to_string();
        ZarrsResult::ZARRS_ERROR_STORAGE_CAPABILITY
    }
}

/// Destroy array.
///
/// # Errors
/// Returns `ZarrsResult::ZARRS_ERROR_NULL_PTR` if `array` is a null pointer.
///
/// # Safety
/// If not null, `array` must be a valid `ZarrsArray` handle.
#[no_mangle]
pub unsafe extern "C" fn zarrsDestroyArray(array: ZarrsArray) -> ZarrsResult {
    if array.is_null() {
        ZarrsResult::ZARRS_ERROR_NULL_PTR
    } else {
        unsafe { array.to_owned().drop_in_place() };
        ZarrsResult::ZARRS_SUCCESS
    }
}

/// Get the size of a chunk in bytes.
///
/// # Safety
/// `array` must be a valid `ZarrsArray` handle.
#[no_mangle]
pub unsafe extern "C" fn zarrsArrayGetChunkSize(
    array: ZarrsArray,
    chunk_indices: *const u64,
    chunk_indices_len: usize,
    chunk_bytes_length: *mut usize,
) -> ZarrsResult {
    // Validation
    if array.is_null() {
        return ZarrsResult::ZARRS_ERROR_NULL_PTR;
    }
    let array = &**array;
    let chunk_indices = std::slice::from_raw_parts(chunk_indices, chunk_indices_len);

    // Get the chunk size
    let chunk_representation = array_fn!(array, chunk_array_representation, chunk_indices);
    match chunk_representation {
        Ok(chunk_representation) => {
            *chunk_bytes_length = usize::try_from(chunk_representation.size()).unwrap();
            ZarrsResult::ZARRS_SUCCESS
        }
        Err(err) => {
            *LAST_ERROR = err.to_string();
            ZarrsResult::ZARRS_ERROR_INVALID_INDICES
        }
    }
}

/// Get the size of a subset in bytes.
///
/// # Safety
/// `array` must be a valid `ZarrsArray` handle.
#[no_mangle]
pub unsafe extern "C" fn zarrsArrayGetSubsetSize(
    array: ZarrsArray,
    subset_shape: *const usize,
    subset_dimensionality: usize,
    subset_bytes_length: *mut usize,
) -> ZarrsResult {
    // Validation
    if array.is_null() {
        return ZarrsResult::ZARRS_ERROR_NULL_PTR;
    }
    let array = &**array;
    let subset_shape = std::slice::from_raw_parts(subset_shape, subset_dimensionality);

    // Get the data type
    let data_type = array_fn!(array, data_type);

    // Get the subset size
    *subset_bytes_length = subset_shape.iter().product::<usize>() * data_type.size();
    ZarrsResult::ZARRS_SUCCESS
}
