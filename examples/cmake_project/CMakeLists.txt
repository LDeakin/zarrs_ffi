cmake_minimum_required(VERSION 3.19)
project(zarrs_ffi_cmake VERSION 0.1.0)

LIST(APPEND CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}")
find_package(zarrs REQUIRED)

add_executable(main ../array_write_read.cpp)
target_link_libraries(main zarrs::zarrs)
