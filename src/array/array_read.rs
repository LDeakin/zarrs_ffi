use zarrs::{array::Array, array_subset::ArraySubset, storage::ReadableStorageTraits};

use crate::{ZarrsResult, LAST_ERROR};

use super::{ZarrsArray, ZarrsArrayEnum};

fn zarrsArrayRetrieveChunkImpl<T: ReadableStorageTraits + ?Sized + 'static>(
    array: &Array<T>,
    chunk_indices: &[u64],
    chunk_bytes_length: usize,
    chunk_bytes: *mut u8,
) -> ZarrsResult {
    match array.retrieve_chunk(chunk_indices) {
        Ok(bytes) => {
            if bytes.len() != chunk_bytes_length {
                unsafe {
                    *LAST_ERROR = format!(
                    "chunk_bytes_length {chunk_bytes_length} does not match decoded chunk size {}",
                    bytes.len()
                )
                };
                ZarrsResult::ZARRS_ERROR_BUFFER_LENGTH
            } else {
                unsafe { std::ptr::copy(bytes.as_ptr(), chunk_bytes, chunk_bytes_length) };
                ZarrsResult::ZARRS_SUCCESS
            }
        }
        Err(err) => {
            unsafe { *LAST_ERROR = err.to_string() };
            ZarrsResult::ZARRS_ERROR_ARRAY
        }
    }
}

/// Retrieve a chunk from an array.
///
/// # Errors
/// Returns an error if the array does not have read capability.
///
/// # Safety
/// `array` must be a valid `ZarrsArray` handle.
/// `path` and `chunk_indices` must have length `chunk_indices_len`.
#[no_mangle]
pub unsafe extern "C" fn zarrsArrayRetrieveChunk(
    array: ZarrsArray,
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
    match array {
        ZarrsArrayEnum::R(array) => {
            zarrsArrayRetrieveChunkImpl(array, chunk_indices, chunk_bytes_length, chunk_bytes)
        }
        ZarrsArrayEnum::RL(array) => {
            zarrsArrayRetrieveChunkImpl(array, chunk_indices, chunk_bytes_length, chunk_bytes)
        }
        ZarrsArrayEnum::RW(array) => {
            zarrsArrayRetrieveChunkImpl(array, chunk_indices, chunk_bytes_length, chunk_bytes)
        }
        ZarrsArrayEnum::RWL(array) => {
            zarrsArrayRetrieveChunkImpl(array, chunk_indices, chunk_bytes_length, chunk_bytes)
        }
        _ => {
            *LAST_ERROR = "storage does not have read capability".to_string();
            ZarrsResult::ZARRS_ERROR_STORAGE_CAPABILITY
        }
    }
}

fn zarrsArrayRetrieveSubsetImpl<T: ReadableStorageTraits + ?Sized + 'static>(
    array: &Array<T>,
    array_subset: &ArraySubset,
    subset_bytes_length: usize,
    subset_bytes: *mut u8,
) -> ZarrsResult {
    match array.retrieve_array_subset(array_subset) {
        Ok(bytes) => {
            if bytes.len() != subset_bytes_length {
                unsafe {
                    *LAST_ERROR = format!(
                    "subset_bytes_length {subset_bytes_length} does not match decoded subset size {}",
                    bytes.len()
                )
                };
                ZarrsResult::ZARRS_ERROR_BUFFER_LENGTH
            } else {
                unsafe { std::ptr::copy(bytes.as_ptr(), subset_bytes, subset_bytes_length) };
                ZarrsResult::ZARRS_SUCCESS
            }
        }
        Err(err) => {
            unsafe { *LAST_ERROR = err.to_string() };
            ZarrsResult::ZARRS_ERROR_ARRAY
        }
    }
}

/// Retrieve a subset from an array.
///
/// # Errors
/// Returns an error if the array does not have read capability.
///
/// # Safety
/// `array` must be a valid `ZarrsArray` handle.
/// `path` and `chunk_indices` must have length `chunk_indices_len`.
#[no_mangle]
pub unsafe extern "C" fn zarrsArrayRetrieveSubset(
    array: ZarrsArray,
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
    match array {
        ZarrsArrayEnum::R(array) => {
            zarrsArrayRetrieveSubsetImpl(array, &array_subset, subset_bytes_length, subset_bytes)
        }
        ZarrsArrayEnum::RL(array) => {
            zarrsArrayRetrieveSubsetImpl(array, &array_subset, subset_bytes_length, subset_bytes)
        }
        ZarrsArrayEnum::RW(array) => {
            zarrsArrayRetrieveSubsetImpl(array, &array_subset, subset_bytes_length, subset_bytes)
        }
        ZarrsArrayEnum::RWL(array) => {
            zarrsArrayRetrieveSubsetImpl(array, &array_subset, subset_bytes_length, subset_bytes)
        }
        _ => {
            *LAST_ERROR = "storage does not have read capability".to_string();
            ZarrsResult::ZARRS_ERROR_STORAGE_CAPABILITY
        }
    }
}
