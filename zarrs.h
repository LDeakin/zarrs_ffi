#pragma once

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

#undef NDEBUG
#ifdef __cplusplus
#include <cassert>
#else // __cplusplus
#include <assert.h>
#endif // __cplusplus

#define zarrs_assert(expr) assert(ZARRS_SUCCESS == expr)


enum ZarrsResult
#ifdef __cplusplus
  : int32_t
#endif // __cplusplus
 {
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
};
#ifndef __cplusplus
typedef int32_t ZarrsResult;
#endif // __cplusplus

typedef struct ZarrsArray_T ZarrsArray_T;

typedef struct ZarrsStorage_T ZarrsStorage_T;

/**
 * An opaque handle to a zarr array.
 */
typedef struct ZarrsArray_T *ZarrsArray;

/**
 * An opaque handle to a zarr store or storage transformer.
 */
typedef struct ZarrsStorage_T *ZarrsStorage;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Get the size of a chunk in bytes.
 *
 * `pChunkIndices` is a pointer to an array of length `dimensionality` holding the chunk indices.
 *
 * # Safety
 * `array` must be a valid `ZarrsArray` handle.
 * `dimensionality` must match the dimensionality of the array and the length of the array pointed to by `pChunkIndices`.
 */
ZarrsResult zarrsArrayGetChunkSize(ZarrsArray array,
                                   size_t dimensionality,
                                   const uint64_t *pChunkIndices,
                                   size_t *chunkSize);

/**
 * Returns the dimensionality of the array.
 *
 * # Errors
 * Returns `ZarrsResult::ZARRS_ERROR_NULL_PTR` if `array` is a null pointer.
 *
 * # Safety
 * If not null, `array` must be a valid `ZarrsArray` handle.
 */
ZarrsResult zarrsArrayGetDimensionality(ZarrsArray array, size_t *dimensionality);

/**
 * Returns the shape of the array.
 *
 * # Errors
 * Returns `ZarrsResult::ZARRS_ERROR_NULL_PTR` if `array` is a null pointer.
 *
 * # Safety
 * If not null, `array` must be a valid `ZarrsArray` handle.
 */
ZarrsResult zarrsArrayGetShape(ZarrsArray array, size_t dimensionality, uint64_t *pShape);

/**
 * Get the size of a subset in bytes.
 *
 * `pSubsetShape` is a pointer to an array of length `dimensionality` holding the shape of the subset.
 *
 * # Safety
 * `array` must be a valid `ZarrsArray` handle.
 * `dimensionality` must match the dimensionality of the array and the length of the array pointed to by `pSubsetShape`.
 */
ZarrsResult zarrsArrayGetSubsetSize(ZarrsArray array,
                                    size_t dimensionality,
                                    const uint64_t *pSubsetShape,
                                    size_t *subsetSize);

/**
 * Retrieve a chunk from an array.
 *
 * `pChunkIndices` is a pointer to an array of length `dimensionality` holding the chunk indices.
 * `pChunkBytes` is a pointer to an array of bytes of length `chunkBytesCount` that must match the expected size of the chunk as returned by `zarrsArrayGetChunkSize()`.
 *
 * # Errors
 * Returns an error if the array does not have read capability.
 *
 * # Safety
 * `array` must be a valid `ZarrsArray` handle.
 * `dimensionality` must match the dimensionality of the array and the length of the array pointed to by `pChunkIndices`.
 */
ZarrsResult zarrsArrayRetrieveChunk(ZarrsArray array,
                                    size_t dimensionality,
                                    const uint64_t *pChunkIndices,
                                    size_t chunkBytesCount,
                                    uint8_t *pChunkBytes);

/**
 * Retrieve a subset from an array.
 *
 * `pSubsetStart` and `pSubsetShape` are pointers to arrays of length `dimensionality` holding the chunk start and shape respectively.
 * `pSubsetBytes` is a pointer to an array of bytes of length `subsetBytesCount` that must match the expected size of the subset as returned by `zarrsArrayGetSubsetSize()`.
 *
 * # Errors
 * Returns an error if the array does not have read capability.
 *
 * # Safety
 * `array` must be a valid `ZarrsArray` handle.
 * `dimensionality` must match the dimensionality of the array and the length of the arrays pointed to by `pSubsetStart` and `pSubsetShape`.
 */
ZarrsResult zarrsArrayRetrieveSubset(ZarrsArray array,
                                     size_t dimensionality,
                                     const uint64_t *pSubsetStart,
                                     const uint64_t *pSubsetShape,
                                     size_t subsetBytesCount,
                                     uint8_t *pSubsetBytes);

/**
 * Store a chunk.
 *
 * `pChunkIndices` is a pointer to an array of length `dimensionality` holding the chunk indices.
 * `pChunkBytes` is a pointer to an array of bytes of length `chunkBytesCount` that must match the expected size of the chunk as returned by `zarrsArrayGetChunkSize()`.
 *
 * # Errors
 * Returns an error if the array does not have write capability.
 *
 * # Safety
 * `array`  must be a valid `ZarrsArray` handle.
 * `dimensionality` must match the dimensionality of the array and the length of the array pointed to by `pChunkIndices`.
 */
ZarrsResult zarrsArrayStoreChunk(ZarrsArray array,
                                 size_t dimensionality,
                                 const uint64_t *pChunkIndices,
                                 size_t chunkBytesCount,
                                 const uint8_t *pChunkBytes);

/**
 * Store array metadata.
 *
 * # Errors
 * Returns an error if the array does not have write capability.
 *
 * # Safety
 * `array` must be a valid `ZarrsArray` handle.
 */
ZarrsResult zarrsArrayStoreMetadata(ZarrsArray array);

/**
 * Store an array subset.
 *
 * `pSubsetStart` and `pSubsetShape` are pointers to arrays of length `dimensionality` holding the chunk start and shape respectively.
 * `pSubsetBytes` is a pointer to an array of bytes of length `subsetBytesCount` that must match the expected size of the subset as returned by `zarrsArrayGetSubsetSize()`.
 *
 * # Errors
 * Returns an error if the array does not have read/write capability.
 *
 * # Safety
 * `array`  must be a valid `ZarrsArray` handle.
 * `dimensionality` must match the dimensionality of the array and the length of the arrays pointed to by `pSubsetStart` and `pSubsetShape`.
 */
ZarrsResult zarrsArrayStoreSubset(ZarrsArray array,
                                  size_t dimensionality,
                                  const uint64_t *pSubsetStart,
                                  const uint64_t *pSubsetShape,
                                  size_t subsetBytesCount,
                                  const uint8_t *pSubsetBytes);

/**
 * Create a handle to an existing array (read/write capability).
 *
 * `pArray` is a pointer to a handle in which the created `ZarrsArray` is returned.
 *
 * # Safety
 * `pArray` must be a valid pointer to a `ZarrsArray` handle.
 */
ZarrsResult zarrsCreateArrayRW(ZarrsStorage storage, const char* path, ZarrsArray *pArray);

/**
 * Create a handle to a new array (read/write capability).
 *
 * `metadata` is expected to be a JSON string representing a zarr V3 array `zarr.json`.
 * `pArray` is a pointer to a handle in which the created `ZarrsArray` is returned.
 *
 * # Safety
 * `pArray` must be a valid pointer to a `ZarrsArray` handle.
 */
ZarrsResult zarrsCreateArrayRWWithMetadata(ZarrsStorage storage,
                                           const char* path,
                                           const char* metadata,
                                           ZarrsArray *pArray);

/**
 * Create a storage handle to a filesystem store.
 *
 * `pStorage` is a pointer to a handle in which the created `ZarrsStorage` is returned.
 *
 * # Safety
 * `pStorage` must be a valid pointer to a `ZarrsStorage` handle.
 */
ZarrsResult zarrsCreateStorageFilesystem(const char* path, ZarrsStorage *pStorage);

/**
 * Destroy array.
 *
 * # Errors
 * Returns `ZarrsResult::ZARRS_ERROR_NULL_PTR` if `array` is a null pointer.
 *
 * # Safety
 * If not null, `array` must be a valid `ZarrsArray` handle.
 */
ZarrsResult zarrsDestroyArray(ZarrsArray array);

/**
 * Destroy storage.
 *
 * # Errors
 * Returns `ZarrsResult::ZARRS_ERROR_NULL_PTR` if `storage` is a null pointer.
 *
 * # Safety
 * If not null, `storage` must be a valid storage device created with a `zarrsStorage` function.
 */
ZarrsResult zarrsDestroyStorage(ZarrsStorage storage);

/**
 * Get the last error string.
 */
const char *zarrsLastError(void);

/**
 * Get the zarrs version.
 *
 * A u32 representation of the version encoded as `(zarrsVersionMajor() << 22) | (zarrsVersionMinor() << 12) | zarrsVersionPatch()`.
 */
uint32_t zarrsVersion(void);

/**
 * Get the zarrs major version.
 */
uint32_t zarrsVersionMajor(void);

/**
 * Get the zarrs minor version.
 */
uint32_t zarrsVersionMinor(void);

/**
 * Get the zarrs patch version.
 */
uint32_t zarrsVersionPatch(void);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
