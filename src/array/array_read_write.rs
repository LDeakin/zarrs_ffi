use zarrs::{array::Array, array_subset::ArraySubset, storage::ReadableWritableStorageTraits};

use crate::{ZarrsResult, LAST_ERROR};

use super::{ZarrsArray, ZarrsArrayEnum};

fn zarrsArrayStoreSubsetImpl<T: ReadableWritableStorageTraits + ?Sized + 'static>(
    array: &Array<T>,
    array_subset: &ArraySubset,
    subset_bytes: &[u8],
) -> ZarrsResult {
    if let Err(err) = array.store_array_subset(array_subset, subset_bytes.to_vec()) {
        unsafe { *LAST_ERROR = err.to_string() };
        ZarrsResult::ZARRS_ERROR_ARRAY
    } else {
        ZarrsResult::ZARRS_SUCCESS
    }
}

/// Store an array subset.
///
/// # Errors
/// Returns an error if the array does not have read/write capability.
///
/// # Safety
/// `array`  must be a valid `ZarrsArray` handle.
/// `path` and `chunk_indices` must have length `chunk_indices_len`.
#[no_mangle]
pub unsafe extern "C" fn zarrsArrayStoreSubset(
    array: ZarrsArray,
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
    match array {
        ZarrsArrayEnum::RW(array) => zarrsArrayStoreSubsetImpl(array, &array_subset, subset_bytes),
        ZarrsArrayEnum::RWL(array) => zarrsArrayStoreSubsetImpl(array, &array_subset, subset_bytes),
        _ => {
            *LAST_ERROR = "storage does not have read/write capability".to_string();
            ZarrsResult::ZARRS_ERROR_STORAGE_CAPABILITY
        }
    }
}
