// SPDX-License-Identifier: GPL-2.0

//! Tty management interfaces.
//!
//! C header: [`include/linux/tty_driver.h`](../../../../include/linux/tty_driver.h)

use crate::{bindings, c_types, Error, Result};
use core::{pin::Pin, ptr};

extern "C" {
    #[allow(improper_ctypes)]
    fn rust_helper_tty_alloc_driver(
        lines: c_types::c_uint,
        flags: c_types::c_ulong,
    ) -> *mut bindings::tty_driver;
}

/// Wraps the kernel's `struct tty_struct`.
///
/// # Invariants
///
/// The pointer `TtyStruct::ptr` is null or valid.
pub struct TtyStruct {
    ptr: *mut bindings::tty_struct,
}

impl TtyStruct {
    /// Constructors a new `struct tty_struct` wrapper.
    ///
    /// # Safety
    ///
    /// The pointer `ptr` must be either null or a valid pointer for the lifetime of the object.
    unsafe fn from_ptr(ptr: *mut bindings::tty_struct) -> Self {
        Self { ptr }
    }
}

pub struct Registration {
    registered: bool,
    driver: *mut bindings::tty_driver,
}

impl Registration {
    pub fn new() -> Self {
        Self {
            registered: false,
            driver: ptr::null_mut(),
        }
    }

    pub fn register(
        self: Pin<&mut Self>,
        lines: c_types::c_uint,
        flags: c_types::c_uint,
    ) -> Result {
        let this = unsafe { self.get_unchecked_mut() };
        if this.registered {
            return Err(Error::EINVAL);
        }

        let ret = unsafe { rust_helper_tty_alloc_driver(lines, flags.into()) };
        this.driver = ret;
        this.registered = true;
        Ok(())
    }
}

impl Default for Registration {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Registration {
    fn drop(&mut self) {
        unimplemented!()
    }
}