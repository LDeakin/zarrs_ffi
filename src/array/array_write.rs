use zarrs::{array::Array, storage::WritableStorageTraits};

use crate::{
    array::{ZarrsArray, ZarrsArrayEnum},
    ZarrsResult, LAST_ERROR,
};

use super::array_fn;

fn zarrsArrayStoreMetadataImpl<T: WritableStorageTraits + ?Sized + 'static>(
    array: &Array<T>,
) -> ZarrsResult {
    match array.store_metadata() {
        Ok(()) => ZarrsResult::ZARRS_SUCCESS,
        Err(err) => {
            unsafe { *LAST_ERROR = err.to_string() };
            ZarrsResult::ZARRS_ERROR_STORAGE
        }
    }
}

/// Write array metadata to store.
///
/// # Errors
/// Returns an error if the array does not have write capability.
///
/// # Safety
/// `array` must be a valid `ZarrsArray` handle.
#[no_mangle]
pub unsafe extern "C" fn zarrsArrayStoreMetadata(array: ZarrsArray) -> ZarrsResult {
    if array.is_null() {
        return ZarrsResult::ZARRS_ERROR_NULL_PTR;
    }
    let array = &**array;
    match array {
        ZarrsArrayEnum::W(array) => zarrsArrayStoreMetadataImpl(array),
        ZarrsArrayEnum::RW(array) => zarrsArrayStoreMetadataImpl(array),
        ZarrsArrayEnum::RWL(array) => zarrsArrayStoreMetadataImpl(array),
        _ => {
            *LAST_ERROR = "storage does not have write capability".to_string();
            ZarrsResult::ZARRS_ERROR_STORAGE_CAPABILITY
        }
    }
}

fn zarrsArrayStoreChunkImpl<T: WritableStorageTraits + ?Sized + 'static>(
    array: &Array<T>,
    chunk_indices: &[u64],
    chunk_bytes: &[u8],
) -> ZarrsResult {
    if let Err(err) = array.store_chunk(chunk_indices, chunk_bytes.to_vec()) {
        unsafe { *LAST_ERROR = err.to_string() };
        ZarrsResult::ZARRS_ERROR_ARRAY
    } else {
        ZarrsResult::ZARRS_SUCCESS
    }
}

/// Write a chunk to an array.
///
/// # Errors
/// Returns an error if the array does not have write capability.
///
/// # Safety
/// `array`  must be a valid `ZarrsArray` handle.
/// `path` and `chunk_indices` must have length `chunk_indices_len`.
#[no_mangle]
pub unsafe extern "C" fn zarrsArrayStoreChunk(
    array: ZarrsArray,
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

    let chunk_representation = array_fn!(array, chunk_array_representation, chunk_indices);
    let Ok(chunk_representation) = chunk_representation else {
        unsafe { *LAST_ERROR = chunk_representation.unwrap_err_unchecked().to_string() };
        return ZarrsResult::ZARRS_ERROR_INVALID_INDICES;
    };
    if chunk_bytes_length as u64 != chunk_representation.size() {
        *LAST_ERROR =
                        format!("zarrsArrayRetrieveChunk chunk_bytes_length {chunk_bytes_length} does not match expected length {}", chunk_representation.size());
        return ZarrsResult::ZARRS_ERROR_BUFFER_LENGTH;
    }

    // Store the chunk bytes
    match array {
        ZarrsArrayEnum::W(array) => zarrsArrayStoreChunkImpl(array, chunk_indices, chunk_bytes),
        ZarrsArrayEnum::RW(array) => zarrsArrayStoreChunkImpl(array, chunk_indices, chunk_bytes),
        ZarrsArrayEnum::RWL(array) => zarrsArrayStoreChunkImpl(array, chunk_indices, chunk_bytes),
        _ => {
            *LAST_ERROR = "storage does not have write capability".to_string();
            ZarrsResult::ZARRS_ERROR_STORAGE_CAPABILITY
        }
    }
}
