/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::{ffi::CStr, fmt, slice};

const SIZEOF_GFX: usize = 8;

pub struct MacroInfo {
    // Placeholder to avoid constructing this type
    _unit: (),
}

impl MacroInfo {
    // It should not be possible to construct this type by the user.
    #[must_use]
    pub(crate) const fn new() -> Self {
        Self { _unit: () }
    }

    #[must_use]
    pub fn macro_offset(&self) -> u32 {
        let offset = unsafe { gfxd_sys::macro_info::gfxd_macro_offset() };
        offset as _
    }

    #[must_use]
    pub fn macro_packets(&self) -> u32 {
        let offset = unsafe { gfxd_sys::macro_info::gfxd_macro_packets() };
        offset as _
    }

    // TODO: I'm not sure how to implement this
    // foreach_pkt

    /// Returns a slice of the binary data for the current macro.
    ///
    /// The data is not byte-swapped.
    #[must_use]
    pub fn macro_data(&self) -> &[u8] {
        let data = unsafe { gfxd_sys::macro_info::gfxd_macro_data() };
        let data_ptr = data.cast().as_ptr();
        let byte_len = self.macro_packets() as usize * SIZEOF_GFX;

        // SAFETY: pointer is non-null and the length of the data should be
        // correct according to gfxd's docs.
        unsafe { slice::from_raw_parts(data_ptr, byte_len) }
    }

    // macro_id

    // macro_name

    /// Returns the number of arguments to the current macro, not including a
    /// dynamic display list pointer if one has been specified.
    #[must_use]
    pub fn arg_count(&self) -> u32 {
        let count = unsafe { gfxd_sys::macro_info::gfxd_arg_count() };

        count as _
    }

    // arg_type

    /// Returns the name of the argument with index `arg_num` or `None` if
    /// `arg_num` is bigger than the argument count for the current macro.
    ///
    /// Argument names are not canonical, nor are they needed for macro
    /// disassembly, but they can be useful for informational and diagnostic
    /// purposes.
    #[must_use]
    pub fn arg_name(&self, arg_num: u32) -> Option<&str> {
        if arg_num >= self.arg_count() {
            return None;
        }

        let buf = unsafe { gfxd_sys::macro_info::gfxd_arg_name(arg_num as _) };

        // SAFETY: Simply trust the data is nul-terminated.
        let name = unsafe { CStr::from_ptr(buf.as_ptr()) };

        name.to_str().ok()
    }

    #[must_use]
    pub fn arg_value(&self, arg_num: u32) -> Option<ArgValue> {
        if arg_num >= self.arg_count() {
            return None;
        }

        let arg_value = unsafe { gfxd_sys::macro_info::gfxd_arg_value(arg_num as _) };
        let arg_value = unsafe { arg_value.as_ref() };

        let arg_fmt = unsafe { gfxd_sys::macro_info::gfxd_arg_fmt(arg_num as _) };
        match arg_fmt as u32 {
            gfxd_sys::macro_info::gfxd_argfmt_i => Some(ArgValue::I(unsafe { arg_value.i })),
            gfxd_sys::macro_info::gfxd_argfmt_u => Some(ArgValue::U(unsafe { arg_value.u })),
            gfxd_sys::macro_info::gfxd_argfmt_f => Some(ArgValue::F(unsafe { arg_value.f })),
            _ => None,
        }
    }

    // value_by_type

    // arg_valid
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[must_use]
pub enum ArgValue {
    I(i32),
    U(u32),
    F(f32),
}

impl fmt::Display for ArgValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgValue::I(x) => write!(f, "{x}"),
            ArgValue::U(x) => write!(f, "0x{x:08X}"),
            ArgValue::F(x) => write!(f, "{x}"),
        }
    }
}
