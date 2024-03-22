/// A zarrs data type.
#[repr(i32)]
pub enum ZarrsDataType {
    ZARRS_UNDEFINED = -1,
    ZARRS_BOOL = 0,
    ZARRS_INT8 = 1,
    ZARRS_INT16 = 2,
    ZARRS_INT32 = 3,
    ZARRS_INT64 = 4,
    ZARRS_UINT8 = 5,
    ZARRS_UINT16 = 6,
    ZARRS_UINT32 = 7,
    ZARRS_UINT64 = 8,
    ZARRS_FLOAT16 = 9,
    ZARRS_FLOAT32 = 10,
    ZARRS_FLOAT64 = 11,
    ZARRS_COMPLEX64 = 12,
    ZARRS_COMPLEX128 = 13,
    ZARRS_RAW_BITS = 14,
    ZARRS_BFLOAT16 = 15,
}
