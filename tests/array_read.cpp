#include "zarrs.h"
#include <cassert>
#include <iostream>
#include <memory>

int main() {
    const char* tmp_path = getenv("TMP_PATH_WRITE_RUST_READ_C");
    ZarrsStorageRW storage = nullptr;
    assert(ZARRS_SUCCESS == zarrsCreateStorageFilesystem(tmp_path, &storage));
    assert(storage);

    ZarrsArrayRW array = nullptr;
    assert(ZARRS_SUCCESS == zarrsCreateArrayRW(storage, "/array", &array));
    assert(array);

    // Update a subset
    size_t subset_start[] = {1, 1};
    size_t subset_shape[] = {2, 2};
    float subset_elements[] = {-1.0f, -2.0f, -3.0f, -4.0f};
    uint8_t* subset_bytes = reinterpret_cast<uint8_t*>(subset_elements);
    assert(ZARRS_SUCCESS == zarrsArrayStoreSubset(array, subset_start, subset_shape, 2, 4 * sizeof(float), subset_bytes));

    // Get the chunk size
    size_t indices[] = {0, 0};
    size_t chunk_size;
    assert(ZARRS_SUCCESS == zarrsArrayGetChunkSize(array, indices, 2, &chunk_size));
    std::cout << chunk_size << std::endl;

    // Get chunk bytes
    std::unique_ptr<uint8_t[]> chunk_bytes(new uint8_t[chunk_size]);
    assert(ZARRS_SUCCESS == zarrsArrayRetrieveChunk(array, indices, 2, chunk_size, chunk_bytes.get()));

    // Print the elements
    auto chunk_elements = reinterpret_cast<float*>(chunk_bytes.get());
    for (size_t i = 0; i < chunk_size / sizeof(float); ++i) {
        std::cout << (i == 0 ? "" : " ") << chunk_elements[i];
    }
    std::cout << std::endl;
    chunk_bytes.reset();

    // Cleanup
    zarrsDestroyArrayRW(array);
    zarrsDestroyStorage(storage);
}