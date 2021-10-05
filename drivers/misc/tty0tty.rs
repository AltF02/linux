// SPDX-License-Identifier: GPL-2.0

//!

#![no_std]
#![feature(allocator_api, global_asm)]

use kernel::prelude::*;
use kernel::{bindings, c_types, tty::tty_driver};

module! {
    type: Tty0tty,
    name: b"tty0tty",
    author: b"Matthew Bakhtiari <dev@mtbk.me>",
    description: b"tty0tty null modem driver",
    license: b"GPL v2",
    params: {
        pairs: u32 {
            default: 4,
            permissions: 0,
            description: b"Number of pairs of devices to be created, maximum of 128",
        },
    },
}

struct Tty0tty {
    message: String,
}

impl KernelModule for Tty0tty {
    fn init() -> Result<Self> {
        let lock = THIS_MODULE.kernel_param_lock();
        pr_info!("Rust minimal sample (init)\n");
        pr_info!("Am I built-in? {}\n", !cfg!(MODULE));

        let tty = tty_driver::Registration::new();
        tty.register(pairs.read() * 2, 0);

        Ok(Tty0tty {
            message: "on the heap!".try_to_owned()?,
        })
    }
}

impl Drop for Tty0tty {
    fn drop(&mut self) {
        pr_info!("My message is {}\n", self.message);
        pr_info!("Rust minimal sample (exit)\n");
    }
}
