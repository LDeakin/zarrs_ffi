use derive_more::Deref;
use ffi_support::FfiStr;
use std::{path::Path, sync::Arc};
use zarrs::storage::{store::FilesystemStore, ReadableWritableStorageTraits};

use crate::{ZarrsResult, LAST_ERROR};

#[doc(hidden)]
#[derive(Deref)]
pub struct ZarrsStorageRW_T(pub Arc<dyn ReadableWritableStorageTraits>);

/// An opaque handle to readable and writable zarr storage.
pub type ZarrsStorageRW = *mut ZarrsStorageRW_T;

/// Create a handle to a filesystem storage.
///
/// `pStorage` is a pointer to a handle in which the created [`ZarrsStorageRW`] is returned.
///
/// # Safety
///
/// `pStorage` must be a valid pointer to a [`ZarrsStorageRW`] handle.
#[no_mangle]
pub unsafe extern "C" fn zarrsCreateStorageFilesystem(
    path: FfiStr,
    pStorage: *mut ZarrsStorageRW,
) -> ZarrsResult {
    match FilesystemStore::new(Path::new(path.as_str())) {
        Ok(store) => {
            *pStorage = Box::into_raw(Box::new(ZarrsStorageRW_T(Arc::new(store))));
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
pub unsafe extern "C" fn zarrsDestroyStorage(storage: ZarrsStorageRW) {
    if storage.is_null() {
        return;
    }

    unsafe { storage.to_owned().drop_in_place() };
}
