/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use gfxd_sys::ptr::NonNullConst;

pub struct Printer {
    // Placeholder to avoid constructing this type
    _unit: (),
}

impl Printer {
    // It should not be possible to construct this type by the user.
    pub(crate) const fn new() -> Self {
        Self { _unit: () }
    }

    pub fn write_str(&mut self, s: &str) {
        let buf = NonNullConst::from_ref(s).cast();

        unsafe {
            // s.len() is the number of bytes in str instead of number of chars
            gfxd_sys::custom_output::gfxd_write(buf, s.len() as _);
        }
    }
}
