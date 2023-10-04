use derive_more::Deref;
use ffi_support::FfiStr;
use zarrs::{
    array::{Array, ArrayMetadata},
    array_subset::ArraySubset,
    storage::ReadableWritableStorageTraits,
};

use crate::{storage::ZarrsStorageRW, ZarrsResult, LAST_ERROR};

#[doc(hidden)]
#[derive(Deref)]
pub struct ZarrsArrayRW_T(pub Array<dyn ReadableWritableStorageTraits>);

/// An opaque handle to a readable and writable array.
pub type ZarrsArrayRW = *mut ZarrsArrayRW_T;

/// Create a readable and writable handle to an existing array.
///
/// `pArray` is a pointer to a handle in which the created [`ZarrsArrayRW`] is returned.
///
/// # Safety
///
/// `pArray` must be a valid pointer to a [`ZarrsArrayRW`] handle.
#[no_mangle]
pub unsafe extern "C" fn zarrsCreateArrayRW(
    storage: ZarrsStorageRW,
    path: FfiStr,
    pArray: *mut ZarrsArrayRW,
) -> ZarrsResult {
    if storage.is_null() {
        *LAST_ERROR = "storage is null".to_string();
        return ZarrsResult::ZARRS_ERROR_NULL_PTR;
    }

    let storage = &**storage;

    match Array::new(storage.clone(), path.into()) {
        Ok(array) => {
            *pArray = Box::into_raw(Box::new(ZarrsArrayRW_T(array)));
            ZarrsResult::ZARRS_SUCCESS
        }
        Err(err) => {
            *LAST_ERROR = err.to_string();
            ZarrsResult::ZARRS_ERROR_ARRAY
        }
    }
}

/// Create a readable and writable handle to an array with metadata.
///
/// `pArray` is a pointer to a handle in which the created [`ZarrsArrayRW`] is returned.
///
/// # Safety
///
/// `pArray` must be a valid pointer to a [`ZarrsArrayRW`] handle.
#[no_mangle]
pub unsafe extern "C" fn zarrsCreateArrayRWWithMetadata(
    storage: ZarrsStorageRW,
    path: FfiStr,
    metadata: FfiStr,
    pArray: *mut ZarrsArrayRW,
) -> ZarrsResult {
    if storage.is_null() {
        *LAST_ERROR = "storage is null".to_string();
        return ZarrsResult::ZARRS_ERROR_NULL_PTR;
    }

    let storage = &**storage;

    let metadata = match serde_json::from_str::<ArrayMetadata>(metadata.as_str()) {
        Ok(metadata) => metadata,
        Err(err) => {
            *LAST_ERROR = err.to_string();
            return ZarrsResult::ZARRS_ERROR_INVALID_METADATA;
        }
    };

    match Array::new_with_metadata(storage.clone(), path.into(), metadata) {
        Ok(array) => {
            *pArray = Box::into_raw(Box::new(ZarrsArrayRW_T(array)));
            ZarrsResult::ZARRS_SUCCESS
        }
        Err(err) => {
            *LAST_ERROR = err.to_string();
            ZarrsResult::ZARRS_ERROR_ARRAY
        }
    }
}

/// Destroy array.
///
/// # Safety
///
/// `array` must be a valid `ZarrsArrayRW` handle.
#[no_mangle]
pub unsafe extern "C" fn zarrsDestroyArrayRW(array: ZarrsArrayRW) {
    if array.is_null() {
        return;
    }

    unsafe { array.to_owned().drop_in_place() };
}

/// Write array metadata to store.
///
/// # Safety
///
/// `array` must be a valid `ZarrsArrayRW` handle.
#[no_mangle]
pub unsafe extern "C" fn zarrsArrayStoreMetadata(array: ZarrsArrayRW) -> ZarrsResult {
    if array.is_null() {
        return ZarrsResult::ZARRS_ERROR_NULL_PTR;
    }
    let array = &**array;
    match array.store_metadata() {
        Ok(()) => ZarrsResult::ZARRS_SUCCESS,
        Err(err) => {
            *LAST_ERROR = err.to_string();
            ZarrsResult::ZARRS_ERROR_STORAGE
        }
    }
}

/// Get the size of a chunk in bytes.
///
/// # Safety
///
/// `array` must be a valid `ZarrsArrayRW` handle.
#[no_mangle]
pub unsafe extern "C" fn zarrsArrayGetChunkSize(
    array: ZarrsArrayRW,
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
    let chunk_representation = array.chunk_array_representation(chunk_indices, array.shape());
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
///
/// `array` must be a valid `ZarrsArrayRW` handle.
#[no_mangle]
pub unsafe extern "C" fn zarrsArrayGetSubsetSize(
    array: ZarrsArrayRW,
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

    // Get the subset size
    *subset_bytes_length = subset_shape.iter().product::<usize>() * array.data_type().size();
    ZarrsResult::ZARRS_SUCCESS
}

/// Write a chunk to an array.
///
/// # Safety
///
/// `array`  must be a valid `ZarrsArrayRW` handle.
/// `path` and `chunk_indices` must have length `chunk_indices_len`.
#[no_mangle]
pub unsafe extern "C" fn zarrsArrayStoreChunk(
    array: ZarrsArrayRW,
    chunk_indices: *const u64,
    chunk_indices_len: usize,
    chunk_bytes_length: usize,
    chunk_bytes: *const u8,
) -> ZarrsResult {
    // Validation
    if array.is_null() {
        return ZarrsResult::ZARRS_ERROR_NULL_PTR;
    }
    let array = &**array;
    let chunk_indices = std::slice::from_raw_parts(chunk_indices, chunk_indices_len);
    let chunk_bytes = std::slice::from_raw_parts(chunk_bytes, chunk_bytes_length);

    let chunk_representation = match array.chunk_array_representation(chunk_indices, array.shape())
    {
        Ok(chunk_representation) => chunk_representation,
        Err(err) => {
            *LAST_ERROR = err.to_string();
            return ZarrsResult::ZARRS_ERROR_INVALID_INDICES;
        }
    };
    if chunk_bytes_length as u64 != chunk_representation.size() {
        *LAST_ERROR =
                        format!("zarrsArrayRetrieveChunk chunk_bytes_length {chunk_bytes_length} does not match expected length {}", chunk_representation.size());
        return ZarrsResult::ZARRS_ERROR_BUFFER_LENGTH;
    }

    // Store the chunk bytes
    if let Err(err) = array.store_chunk(chunk_indices, chunk_bytes) {
        *LAST_ERROR = err.to_string();
        ZarrsResult::ZARRS_ERROR_ARRAY
    } else {
        ZarrsResult::ZARRS_SUCCESS
    }
}

/// Write an array subset to an array.
///
/// # Safety
///
/// `array`  must be a valid `ZarrsArrayRW` handle.
/// `path` and `chunk_indices` must have length `chunk_indices_len`.
#[no_mangle]
pub unsafe extern "C" fn zarrsArrayStoreSubset(
    array: ZarrsArrayRW,
    subset_start: *const u64,
    subset_shape: *const u64,
    subset_dimensionality: usize,
    subset_bytes_length: usize,
    subset_bytes: *const u8,
) -> ZarrsResult {
    // Validation
    if array.is_null() {
        return ZarrsResult::ZARRS_ERROR_NULL_PTR;
    }
    let array = &**array;
    let subset_start = std::slice::from_raw_parts(subset_start, subset_dimensionality);
    let subset_shape = std::slice::from_raw_parts(subset_shape, subset_dimensionality);
    let subset_bytes = std::slice::from_raw_parts(subset_bytes, subset_bytes_length);
    let array_subset =
        ArraySubset::new_with_start_shape_unchecked(subset_start.to_vec(), subset_shape.to_vec());

    // Store the subset bytes
    if let Err(err) = array.store_array_subset(&array_subset, subset_bytes) {
        *LAST_ERROR = err.to_string();
        ZarrsResult::ZARRS_ERROR_ARRAY
    } else {
        ZarrsResult::ZARRS_SUCCESS
    }
}

/// Retrieve a chunk from an array.
///
/// # Safety
///
/// `array` must be a valid `ZarrsArrayRW` handle.
/// `path` and `chunk_indices` must have length `chunk_indices_len`.
#[no_mangle]
pub unsafe extern "C" fn zarrsArrayRetrieveChunk(
    array: ZarrsArrayRW,
    chunk_indices: *const u64,
    chunk_indices_len: usize,
    chunk_bytes_length: usize,
    chunk_bytes: *mut u8,
) -> ZarrsResult {
    if array.is_null() {
        return ZarrsResult::ZARRS_ERROR_NULL_PTR;
    }
    let array = &**array;
    let chunk_indices = std::slice::from_raw_parts(chunk_indices, chunk_indices_len);

    // Get the chunk bytes
    match array.retrieve_chunk(chunk_indices) {
        Ok(bytes) => {
            if bytes.len() != chunk_bytes_length {
                *LAST_ERROR = format!(
                    "chunk_bytes_length {chunk_bytes_length} does not match decoded chunk size {}",
                    bytes.len()
                );
                ZarrsResult::ZARRS_ERROR_BUFFER_LENGTH
            } else {
                std::ptr::copy(bytes.as_ptr(), chunk_bytes, chunk_bytes_length);
                ZarrsResult::ZARRS_SUCCESS
            }
        }
        Err(err) => {
            *LAST_ERROR = err.to_string();
            ZarrsResult::ZARRS_ERROR_ARRAY
        }
    }
}

/// Retrieve a subset from an array.
///
/// # Safety
///
/// `array` must be a valid `ZarrsArrayRW` handle.
/// `path` and `chunk_indices` must have length `chunk_indices_len`.
#[no_mangle]
pub unsafe extern "C" fn zarrsArrayRetrieveSubset(
    array: ZarrsArrayRW,
    subset_start: *const u64,
    subset_shape: *const u64,
    subset_dimensionality: usize,
    subset_bytes_length: usize,
    subset_bytes: *mut u8,
) -> ZarrsResult {
    // Validation
    if array.is_null() {
        return ZarrsResult::ZARRS_ERROR_NULL_PTR;
    }
    let array = &**array;
    let subset_start = std::slice::from_raw_parts(subset_start, subset_dimensionality);
    let subset_shape = std::slice::from_raw_parts(subset_shape, subset_dimensionality);
    let array_subset =
        ArraySubset::new_with_start_shape_unchecked(subset_start.to_vec(), subset_shape.to_vec());

    // Get the subset bytes
    match array.retrieve_array_subset(&array_subset) {
        Ok(bytes) => {
            if bytes.len() != subset_bytes_length {
                *LAST_ERROR = format!(
                    "subset_bytes_length {subset_bytes_length} does not match decoded subset size {}",
                    bytes.len()
                );
                ZarrsResult::ZARRS_ERROR_BUFFER_LENGTH
            } else {
                std::ptr::copy(bytes.as_ptr(), subset_bytes, subset_bytes_length);
                ZarrsResult::ZARRS_SUCCESS
            }
        }
        Err(err) => {
            *LAST_ERROR = err.to_string();
            ZarrsResult::ZARRS_ERROR_ARRAY
        }
    }
}
