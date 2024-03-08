use derive_more::Deref;
use ffi_support::FfiStr;
use std::sync::Arc;

use crate::{ZarrsResult, LAST_ERROR};

#[doc(hidden)]
pub enum ZarrsStorageEnum {
    R(Arc<dyn zarrs::storage::ReadableStorageTraits>),
    W(Arc<dyn zarrs::storage::WritableStorageTraits>),
    L(Arc<dyn zarrs::storage::ListableStorageTraits>),
    RL(Arc<dyn zarrs::storage::ReadableListableStorageTraits>),
    RW(Arc<dyn zarrs::storage::ReadableWritableStorageTraits>),
    RWL(Arc<dyn zarrs::storage::ReadableWritableListableStorageTraits>),
}

#[doc(hidden)]
#[derive(Deref)]
pub struct ZarrsStorage_T(ZarrsStorageEnum);

/// An opaque handle to zarr storage.
pub type ZarrsStorage = *mut ZarrsStorage_T;

/// Create a handle to a filesystem storage.
///
/// `pStorage` is a pointer to a handle in which the created [`ZarrsStorage`] is returned.
///
/// # Safety
///
/// `pStorage` must be a valid pointer to a [`ZarrsStorage`] handle.
#[no_mangle]
pub unsafe extern "C" fn zarrsCreateStorageFilesystem(
    path: FfiStr,
    pStorage: *mut ZarrsStorage,
) -> ZarrsResult {
    let path = std::path::Path::new(path.as_str());
    match zarrs::storage::store::FilesystemStore::new(path) {
        Ok(store) => {
            *pStorage = Box::into_raw(Box::new(ZarrsStorage_T(ZarrsStorageEnum::RW(Arc::new(
                store,
            )))));
            ZarrsResult::ZARRS_SUCCESS
        }
        Err(err) => {
            *LAST_ERROR = err.to_string();
            ZarrsResult::ZARRS_ERROR_STORAGE
        }
    }
}

/// Destroy storage.
///
/// # Safety
///
/// `storage` must be a valid storage device created with a `zarrsStorage` function.
#[no_mangle]
pub unsafe extern "C" fn zarrsDestroyStorage(storage: ZarrsStorage) {
    if storage.is_null() {
        return;
    }

    unsafe { storage.to_owned().drop_in_place() };
}
