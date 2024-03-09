include(FetchContent)

option(ZARRS_USE_EXPERIMENTAL_CODECS "enable experimental zarrs codecs" ON)

# Corrosion: integrate rust into a cmake project
FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.4
)
FetchContent_MakeAvailable(Corrosion)

# zarrs-ffi: C/C++ bindings to zarrs
FetchContent_Declare(
    zarrs_source
        # SOURCE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../.."
        GIT_REPOSITORY https://github.com/LDeakin/zarrs-ffi.git
        GIT_TAG v0.5.0
)
FetchContent_Populate(zarrs_source)

# Build zarrs-ffi, creates a zarrs target
if(ZARRS_USE_EXPERIMENTAL_CODECS)
  set(ZARRS_FEATURES zarrs/bitround zarrs/zfp zarrs/bz2 zarrs/pcodec)
  message(STATUS "Enabling experimental zarrs codecs: ${ZARRS_FEATURES}")
else()
  set(ZARRS_FEATURES)
endif()
corrosion_import_crate(MANIFEST_PATH ${zarrs_source_SOURCE_DIR}/Cargo.toml FEATURES ${ZARRS_FEATURES})
# corrosion_experimental_cbindgen(TARGET zarrs HEADER_NAME "zarrs.h") # not working
target_include_directories(zarrs INTERFACE ${zarrs_source_SOURCE_DIR}) # add zarrs.h to include directories
add_library(zarrs::zarrs ALIAS zarrs)
