/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

pub struct MacroInfo {
    // Placeholder to avoid constructing this type
    _unit: (),
}

impl MacroInfo {
    // It should not be possible to construct this type by the user.
    pub(crate) const fn new() -> Self {
        Self { _unit: () }
    }

    pub fn macro_offset(&self) -> u32 {
        let offset = unsafe { gfxd_sys::macro_info::gfxd_macro_offset() };
        offset as _
    }

    pub fn macro_packets(&self) -> u32 {
        let offset = unsafe { gfxd_sys::macro_info::gfxd_macro_packets() };
        offset as _
    }
}
