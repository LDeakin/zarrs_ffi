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
  zarrs_assert(zarrsCreateArrayRWWithMetadata(storage, "/array", metadata, &array));
  assert(array);

  size_t dimensionality;
  zarrs_assert(zarrsArrayGetDimensionality(array, &dimensionality));
  assert(dimensionality == 2);

  std::vector<uint64_t> shape(dimensionality);
  zarrs_assert(zarrsArrayGetShape(array, dimensionality, shape.data()));
  assert(shape.size() == 2);
  assert(shape[0] == 8);
  assert(shape[1] == 8);

  ZarrsDataType data_type;
  zarrs_assert(zarrsArrayGetDataType(array, &data_type));
  assert(data_type == ZarrsDataType::ZARRS_FLOAT32);

  // Update a subset
  size_t subset_start[] = {1, 1};
  size_t subset_shape[] = {2, 2};
  float subset_elements[] = {-1.0f, -2.0f, -3.0f, -4.0f};
  uint8_t *subset_bytes = reinterpret_cast<uint8_t *>(subset_elements);
  size_t subset_size;
  zarrs_assert(zarrsArrayGetSubsetSize(array, 2, subset_shape, &subset_size));
  assert(subset_size == 4 * sizeof(float));
  zarrs_assert(zarrsArrayStoreSubset(array, 2, subset_start, subset_shape, 4 * sizeof(float), subset_bytes));

  // Get the chunk size
  size_t indices[] = {0, 0};
  size_t chunk_size;
  zarrs_assert(zarrsArrayGetChunkSize(array, 2, indices, &chunk_size));
  std::cout << chunk_size << std::endl;

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
