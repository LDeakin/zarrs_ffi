// This is tested by ffi_array_write_c_read_c

#include "zarrs.h"

#include <filesystem>
#include <iostream>
#include <memory>
#include <string>
#include <vector>

const char *metadata = R""""(
{
    "zarr_format": 3,
    "node_type": "array",
    "shape": [
        8,
        8
    ],
    "data_type": "float32",
    "chunk_grid": {
        "name": "regular",
        "configuration": {
            "chunk_shape": [
                4,
                4
            ]
        }
    },
    "chunk_key_encoding": {
        "name": "default",
        "configuration": {
            "separator": "/"
        }
    },
    "fill_value": "NaN",
    "codecs": [
        {
            "name": "bytes",
            "configuration": {
                "endian": "little"
            }
        },
            {
            "name": "gzip",
            "configuration": {
                "level": 5
            }
        }
    ],
    "dimension_names": [
        "y",
        "x"
    ]
}
)"""";

auto get_tmp_path() -> std::filesystem::path {
  if (const char *tmp_path_test = getenv("TMP_PATH_WRITE_C_READ_C"); tmp_path_test) {
    return std::filesystem::path(tmp_path_test);
  } else {
    return std::filesystem::temp_directory_path();
  }
}

int main() {
  const std::filesystem::path tmp_path = get_tmp_path();

  ZarrsStorage storage = nullptr;
  zarrs_assert(zarrsCreateStorageFilesystem(tmp_path.c_str(), &storage));
  assert(storage);

  ZarrsArray array = nullptr;
  zarrs_assert(zarrsCreateArrayRW(storage, "/array", metadata, &array));
  assert(array);

  size_t dimensionality;
  zarrs_assert(zarrsArrayGetDimensionality(array, &dimensionality));
  assert(dimensionality == 2);

  std::vector<uint64_t> shape(2);
  zarrs_assert(zarrsArrayGetShape(array, 2, shape.data()));
  assert(shape[0] == 8);
  assert(shape[1] == 8);

  ZarrsDataType data_type;
  zarrs_assert(zarrsArrayGetDataType(array, &data_type));
  assert(data_type == ZarrsDataType::ZARRS_FLOAT32);
  
  std::vector<uint64_t> chunk_grid_shape(2);
  zarrs_assert(zarrsArrayGetChunkGridShape(array, 2, chunk_grid_shape.data()));
  assert(chunk_grid_shape[0] == 2);
  assert(chunk_grid_shape[1] == 2);

  const std::vector<uint64_t> array_subset_start = {3, 4};
  const std::vector<uint64_t> array_subset_shape = {2, 2};
  std::vector<uint64_t> intersecting_chunks_start(2);
  std::vector<uint64_t> intersecting_chunks_shape(2);
  zarrs_assert(zarrsArrayGetChunksInSubset(array, 2, array_subset_start.data(), array_subset_shape.data(), intersecting_chunks_start.data(), intersecting_chunks_shape.data()));
  assert(intersecting_chunks_start[0] == 0);
  assert(intersecting_chunks_start[1] == 1);
  assert(intersecting_chunks_shape[0] == 2);
  assert(intersecting_chunks_shape[1] == 1);

  // Update a subset
  size_t subset_start[] = {1, 1};
  size_t subset_shape[] = {2, 2};
  float subset_elements[] = {-1.0f, -2.0f, -3.0f, -4.0f};
  uint8_t *subset_bytes = reinterpret_cast<uint8_t *>(subset_elements);
  size_t subset_size;
  zarrs_assert(zarrsArrayGetSubsetSize(array, 2, subset_shape, &subset_size));
  assert(subset_size == 4 * sizeof(float));
  zarrs_assert(zarrsArrayStoreSubset(array, 2, subset_start, subset_shape, 4 * sizeof(float), subset_bytes));

  // Get the chunk size and shape
  size_t indices[] = {0, 0};
  size_t chunk_size;
  zarrs_assert(zarrsArrayGetChunkSize(array, 2, indices, &chunk_size));
  assert(chunk_size == (4 * 4 * sizeof(float)));
  uint64_t chunk_origin[2];
  zarrs_assert(zarrsArrayGetChunkOrigin(array, 2, indices, chunk_origin));
  assert(chunk_origin[0] == 0);
  assert(chunk_origin[1] == 0);
  uint64_t chunk_shape[2];
  zarrs_assert(zarrsArrayGetChunkShape(array, 2, indices, chunk_shape));
  assert(chunk_shape[0] == 4);
  assert(chunk_shape[1] == 4);

  // Get chunk bytes
  std::unique_ptr<uint8_t[]> chunk_bytes(new uint8_t[chunk_size]);
  zarrs_assert(zarrsArrayRetrieveChunk(array, 2, indices, chunk_size, chunk_bytes.get()));

  // Print the elements
  auto chunk_elements = reinterpret_cast<float *>(chunk_bytes.get());
  for (size_t i = 0; i < chunk_size / sizeof(float); ++i) {
    std::cout << (i == 0 ? "" : " ") << chunk_elements[i];
  }
  std::cout << std::endl;
  chunk_bytes.reset();

  // Cleanup
  zarrs_assert(zarrsDestroyArray(array));
  zarrs_assert(zarrsDestroyStorage(storage));
}
