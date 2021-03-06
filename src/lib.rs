extern crate zfs_sys;

pub mod zpool;
#[derive(Debug)]
pub enum Error {
    CouldNotInitialize
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Handle {
    handle: *mut zfs_sys::libzfs_handle_t
}

impl Handle {
    pub fn new() -> Result<Handle> {
        let raw = unsafe { zfs_sys::libzfs_init() };
        if raw == std::ptr::null_mut() {
            return Err(Error::CouldNotInitialize);
        } else {
            Ok(Handle {
                handle: raw
            })
        }
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe { zfs_sys::libzfs_fini(self.handle) };
    }
}
