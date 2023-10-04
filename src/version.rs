/// Get the major version.
///
/// See [`zarrs::version::version_major`].
#[no_mangle]
pub extern "C" fn zarrsVersionMajor() -> u32 {
    zarrs::version::version_major()
}

/// Get the minor version.
///
/// See [`zarrs::version::version_minor`].
#[no_mangle]
pub extern "C" fn zarrsVersionMinor() -> u32 {
    zarrs::version::version_minor()
}

/// Get the patch version.
///
/// See [`zarrs::version::version_patch`].
#[no_mangle]
pub extern "C" fn zarrsVersionPatch() -> u32 {
    zarrs::version::version_patch()
}

/// Get the version.
///
/// See [zarrs::version::version].
#[no_mangle]
pub extern "C" fn zarrsVersion() -> u32 {
    zarrs::version::version()
}

#[cfg(test)]
mod tests {
    use inline_c::assert_cxx;

    #[test]
    fn ffi_version() {
        (assert_cxx! {
            #include "zarrs.h"
            #include <cassert>

            int main() {
                assert(zarrsVersionMajor() == 0);
                assert(zarrsVersionMinor() == 4);
            }
        })
        .success();
    }
}
