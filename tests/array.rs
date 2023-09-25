use inline_c::assert_cxx;

#[test]
fn ffi_array_write_rust_read_c() {
    use std::sync::Arc;
    use zarrs::{
        array::{DataType, FillValue},
        storage::store::FilesystemStore,
    };

    let tmp_path = tempdir::TempDir::new("write_rust_read_c").unwrap();
    std::env::set_var(
        "INLINE_C_RS_TMP_PATH_WRITE_RUST_READ_C",
        tmp_path.path().to_string_lossy().to_string(),
    );

    let store = Arc::new(FilesystemStore::new(tmp_path.path()).unwrap());

    // Create an array
    let array_path = "/array";
    let array = zarrs::array::ArrayBuilder::new(
        vec![8, 8], // array shape
        DataType::Float32,
        vec![4, 4].into(), // regular chunk shape
        FillValue::from(f32::NAN),
    )
    .bytes_to_bytes_codecs(vec![
        #[cfg(feature = "gzip")]
        Box::new(codec::GzipCodec::new(5).unwrap()),
    ])
    .dimension_names(vec!["y".into(), "x".into()])
    .storage_transformers(vec![])
    .build(store.clone(), array_path)
    .unwrap();

    array.store_metadata().unwrap();

    array
        .store_chunk_elements::<f32>(&[0, 0], &(0..16).map(|f| f as f32).collect::<Vec<_>>())
        .unwrap();

    (assert_cxx! {
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
    })
    .success()
    .stdout("64\n0 1 2 3 4 -1 -2 7 8 -3 -4 11 12 13 14 15\n");
}

#[test]
fn ffi_array_write_c_read_c() {
    use std::io::Write;

    let tmp_path = tempdir::TempDir::new("write_c_read_c").unwrap();
    std::env::set_var(
        "INLINE_C_RS_TMP_PATH_WRITE_C_READ_C",
        tmp_path.path().to_string_lossy().to_string(),
    );

    let metadata: &'static str = r#"
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
"#;
    let mut metadata_tmp_file =
        std::fs::File::create(tmp_path.path().to_string_lossy().to_string() + "/metadata_tmp.json")
            .unwrap();
    write!(metadata_tmp_file, "{}", metadata).unwrap();
    drop(metadata_tmp_file);

    (assert_cxx! {
        #include "zarrs.h"
        #include <cassert>
        #include <iostream>
        #include <fstream>
        #include <sstream>
        #include <memory>
        #include <string>

        int main() {
            const char* tmp_path = getenv("TMP_PATH_WRITE_C_READ_C");
            ZarrsStorageRW storage = nullptr;
            assert(ZARRS_SUCCESS == zarrsCreateStorageFilesystem(tmp_path, &storage));
            assert(storage);

            // can't have a string literal in assert_cxx.. so doing this
            std::string path = std::string(tmp_path) + "/metadata_tmp.json";
            std::ifstream ifs(path);
            std::stringstream buffer;
            buffer << ifs.rdbuf();
            std::string metadata = buffer.str();

            ZarrsArrayRW array = nullptr;
            assert(ZARRS_SUCCESS == zarrsCreateArrayRWWithMetadata(storage, "/array", metadata.c_str(), &array));
            assert(array);

            // Update a subset
            size_t subset_start[] = {1, 1};
            size_t subset_shape[] = {2, 2};
            float subset_elements[] = {-1.0f, -2.0f, -3.0f, -4.0f};
            uint8_t* subset_bytes = reinterpret_cast<uint8_t*>(subset_elements);
            size_t subset_size;
            assert(ZARRS_SUCCESS == zarrsArrayGetSubsetSize(array, subset_shape, 2, &subset_size));
            assert(subset_size == 4 * sizeof(float));
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
    })
    .success()
    .stdout("64\nnan nan nan nan nan -1 -2 nan nan -3 -4 nan nan nan nan nan\n");
}
