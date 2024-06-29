pub mod array_read;
pub mod array_read_write;
pub mod array_write;
pub mod data_type;

use ffi_support::FfiStr;
use zarrs::{
    array::{chunk_shape_to_array_shape, Array, ArrayMetadata, DataType},
    array_subset::ArraySubset,
};

use crate::{
    storage::{ZarrsStorage, ZarrsStorageEnum},
    ZarrsDataType, ZarrsResult, LAST_ERROR,
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
/// `pArray` is a pointer to a handle in which the created `ZarrsArray` is returned.
///
/// # Safety
/// `pArray` must be a valid pointer to a `ZarrsArray` handle.
#[no_mangle]
pub unsafe extern "C" fn zarrsOpenArrayRW(
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
        match Array::open(storage.clone(), path.into()) {
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
/// `pArray` is a pointer to a handle in which the created `ZarrsArray` is returned.
///
/// # Safety
/// `pArray` must be a valid pointer to a `ZarrsArray` handle.
#[no_mangle]
pub unsafe extern "C" fn zarrsCreateArrayRW(
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

/// Returns the dimensionality of the array.
///
/// # Errors
/// Returns `ZarrsResult::ZARRS_ERROR_NULL_PTR` if `array` is a null pointer.
///
/// # Safety
/// If not null, `array` must be a valid `ZarrsArray` handle.
#[no_mangle]
pub unsafe extern "C" fn zarrsArrayGetDimensionality(
    array: ZarrsArray,
    dimensionality: *mut usize,
) -> ZarrsResult {
    if array.is_null() {
        return ZarrsResult::ZARRS_ERROR_NULL_PTR;
    }
    let array = &**array;
    *dimensionality = array_fn!(array, dimensionality);
    ZarrsResult::ZARRS_SUCCESS
}

/// Returns the shape of the array.
///
/// # Errors
/// Returns `ZarrsResult::ZARRS_ERROR_NULL_PTR` if `array` is a null pointer.
///
/// # Safety
/// If not null, `array` must be a valid `ZarrsArray` handle.
/// `dimensionality` must match the dimensionality of the array and the length of the array pointed to by `pShape`.
#[no_mangle]
pub unsafe extern "C" fn zarrsArrayGetShape(
    array: ZarrsArray,
    dimensionality: usize,
    pShape: *mut u64,
) -> ZarrsResult {
    if array.is_null() {
        return ZarrsResult::ZARRS_ERROR_NULL_PTR;
    }
    let array = &**array;
    let shape = array_fn!(array, shape);
    let pShape = unsafe { std::slice::from_raw_parts_mut(pShape, dimensionality) };
    pShape.copy_from_slice(shape);
    ZarrsResult::ZARRS_SUCCESS
}

/// Returns the data type of the array.
///
/// # Errors
/// Returns `ZarrsResult::ZARRS_ERROR_NULL_PTR` if `array` is a null pointer.
///
/// # Safety
/// If not null, `array` must be a valid `ZarrsArray` handle.
#[no_mangle]
pub unsafe extern "C" fn zarrsArrayGetDataType(
    array: ZarrsArray,
    pDataType: *mut ZarrsDataType,
) -> ZarrsResult {
    if array.is_null() {
        return ZarrsResult::ZARRS_ERROR_NULL_PTR;
    }
    let array = &**array;
    let data_type = array_fn!(array, data_type);
    *pDataType = match data_type {
        DataType::Bool => ZarrsDataType::ZARRS_BOOL,
        DataType::Int8 => ZarrsDataType::ZARRS_INT8,
        DataType::Int16 => ZarrsDataType::ZARRS_INT16,
        DataType::Int32 => ZarrsDataType::ZARRS_INT32,
        DataType::Int64 => ZarrsDataType::ZARRS_INT64,
        DataType::UInt8 => ZarrsDataType::ZARRS_UINT8,
        DataType::UInt16 => ZarrsDataType::ZARRS_UINT16,
        DataType::UInt32 => ZarrsDataType::ZARRS_UINT32,
        DataType::UInt64 => ZarrsDataType::ZARRS_UINT64,
        DataType::Float16 => ZarrsDataType::ZARRS_FLOAT16,
        DataType::Float32 => ZarrsDataType::ZARRS_FLOAT32,
        DataType::Float64 => ZarrsDataType::ZARRS_FLOAT64,
        DataType::BFloat16 => ZarrsDataType::ZARRS_BFLOAT16,
        DataType::Complex64 => ZarrsDataType::ZARRS_COMPLEX64,
        DataType::Complex128 => ZarrsDataType::ZARRS_COMPLEX128,
        DataType::RawBits(_) => ZarrsDataType::ZARRS_RAW_BITS,
        _ => ZarrsDataType::ZARRS_UNDEFINED,
    };
    ZarrsResult::ZARRS_SUCCESS
}

/// Return the number of chunks in the chunk grid.
///
/// # Errors
/// Returns `ZarrsResult::ZARRS_ERROR_NULL_PTR` if `array` is a null pointer.
/// Returns `ZarrsResult::ZARRS_ERROR_UNKNOWN_CHUNK_GRID_SHAPE` if the chunk grid shape cannot be determined.
///
/// # Safety
/// If not null, `array` must be a valid `ZarrsArray` handle.
/// `dimensionality` must match the dimensionality of the array and the length of the array pointed to by `pChunkGridShape`.
#[no_mangle]
pub unsafe extern "C" fn zarrsArrayGetChunkGridShape(
    array: ZarrsArray,
    dimensionality: usize,
    pChunkGridShape: *mut u64,
) -> ZarrsResult {
    if array.is_null() {
        return ZarrsResult::ZARRS_ERROR_NULL_PTR;
    }
    let array = &**array;
    let shape = array_fn!(array, chunk_grid_shape);
    if let Some(shape) = shape {
        let pChunkGridShape =
            unsafe { std::slice::from_raw_parts_mut(pChunkGridShape, dimensionality) };
        pChunkGridShape.copy_from_slice(&shape);
        ZarrsResult::ZARRS_SUCCESS
    } else {
        ZarrsResult::ZARRS_ERROR_UNKNOWN_CHUNK_GRID_SHAPE
    }
}

/// Return the chunks indicating the chunks intersecting `array_subset`.
///
/// # Errors
/// Returns `ZarrsResult::ZARRS_ERROR_NULL_PTR` if `array` is a null pointer.
/// Returns `ZarrsResult::ZARRS_ERROR_UNKNOWN_INTERSECTING_CHUNKS` if the intersecting chunks cannot be determined.
///
/// # Safety
/// If not null, `array` must be a valid `ZarrsArray` handle.
/// `dimensionality` must match the dimensionality of the array and the length of the arrays pointed to by `pSubsetStart`, `pSubsetShape`, `pChunksStart`, and `pChunksShape`.
#[no_mangle]
pub unsafe extern "C" fn zarrsArrayGetChunksInSubset(
    array: ZarrsArray,
    dimensionality: usize,
    pSubsetStart: *const u64,
    pSubsetShape: *const u64,
    pChunksStart: *mut u64,
    pChunksShape: *mut u64,
) -> ZarrsResult {
    if array.is_null() {
        return ZarrsResult::ZARRS_ERROR_NULL_PTR;
    }
    let array = &**array;
    let pSubsetStart = unsafe { std::slice::from_raw_parts(pSubsetStart, dimensionality) };
    let pSubsetShape = unsafe { std::slice::from_raw_parts(pSubsetShape, dimensionality) };
    let array_subset = unsafe {
        ArraySubset::new_with_start_shape_unchecked(pSubsetStart.to_vec(), pSubsetShape.to_vec())
    };
    let shape = array_fn!(array, chunks_in_array_subset, &array_subset);
    if let Ok(Some(chunks_subset)) = shape {
        let pChunksStart = unsafe { std::slice::from_raw_parts_mut(pChunksStart, dimensionality) };
        pChunksStart.copy_from_slice(chunks_subset.start());
        let pChunksShape = unsafe { std::slice::from_raw_parts_mut(pChunksShape, dimensionality) };
        pChunksShape.copy_from_slice(chunks_subset.shape());
        ZarrsResult::ZARRS_SUCCESS
    } else {
        ZarrsResult::ZARRS_ERROR_UNKNOWN_INTERSECTING_CHUNKS
    }
}

/// Get the size of a chunk in bytes.
///
/// `pChunkIndices` is a pointer to an array of length `dimensionality` holding the chunk indices.
///
/// # Safety
/// `array` must be a valid `ZarrsArray` handle.
/// `dimensionality` must match the dimensionality of the array and the length of the array pointed to by `pChunkIndices`.
#[no_mangle]
pub unsafe extern "C" fn zarrsArrayGetChunkSize(
    array: ZarrsArray,
    dimensionality: usize,
    pChunkIndices: *const u64,
    chunkSize: *mut usize,
) -> ZarrsResult {
    // Validation
    if array.is_null() {
        return ZarrsResult::ZARRS_ERROR_NULL_PTR;
    }
    let array = &**array;
    let chunk_indices = std::slice::from_raw_parts(pChunkIndices, dimensionality);

    // Get the chunk size
    let chunk_representation = array_fn!(array, chunk_array_representation, chunk_indices);
    match chunk_representation {
        Ok(chunk_representation) => {
            *chunkSize = usize::try_from(chunk_representation.size()).unwrap();
            ZarrsResult::ZARRS_SUCCESS
        }
        Err(err) => {
            *LAST_ERROR = err.to_string();
            ZarrsResult::ZARRS_ERROR_INVALID_INDICES
        }
    }
}

/// Get the origin of a chunk.
///
/// `pChunkIndices` is a pointer to an array of length `dimensionality` holding the chunk indices.
///
/// # Safety
/// `array` must be a valid `ZarrsArray` handle.
/// `dimensionality` must match the dimensionality of the array and the length of the array pointed to by `pChunkIndices` and `pChunkOrigin`.
#[no_mangle]
pub unsafe extern "C" fn zarrsArrayGetChunkOrigin(
    array: ZarrsArray,
    dimensionality: usize,
    pChunkIndices: *const u64,
    pChunkOrigin: *mut u64,
) -> ZarrsResult {
    // Validation
    if array.is_null() {
        return ZarrsResult::ZARRS_ERROR_NULL_PTR;
    }
    let array = &**array;
    let chunk_indices = std::slice::from_raw_parts(pChunkIndices, dimensionality);

    // Get the chunk origin
    let chunk_origin = array_fn!(array, chunk_origin, chunk_indices);
    match chunk_origin {
        Ok(chunk_origin) => {
            let pChunkOrigin =
                unsafe { std::slice::from_raw_parts_mut(pChunkOrigin, dimensionality) };
            pChunkOrigin.copy_from_slice(&chunk_origin);
            ZarrsResult::ZARRS_SUCCESS
        }
        Err(err) => {
            *LAST_ERROR = err.to_string();
            ZarrsResult::ZARRS_ERROR_INVALID_INDICES
        }
    }
}

/// Get the shape of a chunk.
///
/// `pChunkIndices` is a pointer to an array of length `dimensionality` holding the chunk indices.
///
/// # Safety
/// `array` must be a valid `ZarrsArray` handle.
/// `dimensionality` must match the dimensionality of the array and the length of the array pointed to by `pChunkIndices` and `pChunkShape`.
#[no_mangle]
pub unsafe extern "C" fn zarrsArrayGetChunkShape(
    array: ZarrsArray,
    dimensionality: usize,
    pChunkIndices: *const u64,
    pChunkShape: *mut u64,
) -> ZarrsResult {
    // Validation
    if array.is_null() {
        return ZarrsResult::ZARRS_ERROR_NULL_PTR;
    }
    let array = &**array;
    let chunk_indices = std::slice::from_raw_parts(pChunkIndices, dimensionality);

    // Get the chunk shape
    let chunk_shape = array_fn!(array, chunk_shape, chunk_indices);
    match chunk_shape {
        Ok(chunk_shape) => {
            let pChunkShape =
                unsafe { std::slice::from_raw_parts_mut(pChunkShape, dimensionality) };
            pChunkShape.copy_from_slice(&chunk_shape_to_array_shape(&chunk_shape));
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
/// `pSubsetShape` is a pointer to an array of length `dimensionality` holding the shape of the subset.
///
/// # Safety
/// `array` must be a valid `ZarrsArray` handle.
/// `dimensionality` must match the dimensionality of the array and the length of the array pointed to by `pSubsetShape`.
#[no_mangle]
pub unsafe extern "C" fn zarrsArrayGetSubsetSize(
    array: ZarrsArray,
    dimensionality: usize,
    pSubsetShape: *const u64,
    subsetSize: *mut usize,
) -> ZarrsResult {
    // Validation
    if array.is_null() {
        return ZarrsResult::ZARRS_ERROR_NULL_PTR;
    }
    let array = &**array;
    let subset_shape = std::slice::from_raw_parts(pSubsetShape, dimensionality);

    // Get the data type
    let data_type = array_fn!(array, data_type);

    // Get the subset size
    *subsetSize = usize::try_from(subset_shape.iter().product::<u64>()).unwrap() * data_type.size();
    ZarrsResult::ZARRS_SUCCESS
}
