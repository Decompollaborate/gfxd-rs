/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::{fmt, slice};
use gfxd_sys::{ffi, macro_info::gfxd_value_t, ptr::NonNullConst};

use crate::{utils, ArgType, MacroId};

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

    /// Returns a number that uniquely identifies the current macro.
    #[must_use]
    pub fn macro_id(&self) -> Option<MacroId> {
        let macro_id_raw = unsafe { gfxd_sys::macro_info::gfxd_macro_id() };

        MacroId::from_u32(macro_id_raw as _)
    }

    /// Returns the name of the current macro.
    ///
    /// If the macro does not have a name (i.e. it's invalid), `None` is
    /// returned.
    ///
    /// If a dynamic display list pointer has been specified, the dynamic `g`
    /// version is returned. Otherwise the static `gs` version is returned.
    // This function takes `&mut self` instead of plain `&self` because the
    // returned data from `gfxd_macro_name` is invalidated on subsequent calls
    // to said function, so `&mut` is used to ensure a unique pointer only ever
    // exists.
    #[must_use]
    pub fn macro_name(&mut self) -> Option<&str> {
        let macro_name_raw = unsafe { gfxd_sys::macro_info::gfxd_macro_name() }?;

        // SAFETY: The pointer given by gfxd is a nul-terminated C string, and
        // the.data should be UTF-8 already.
        let macro_str = unsafe { utils::str_from_c_str(macro_name_raw) };

        Some(macro_str)
    }

    /// Returns the number of arguments to the current macro, not including a
    /// dynamic display list pointer if one has been specified.
    #[must_use]
    pub fn arg_count(&self) -> u32 {
        let count = unsafe { gfxd_sys::macro_info::gfxd_arg_count() };

        count as _
    }

    /// Returns a number that identifies the type of the argument with index
    /// `arg_num`, or `None` if `arg_num` is larger than the argument count for
    /// the current macro
    #[must_use]
    pub fn arg_type(&self, arg_num: u32) -> Option<ArgType> {
        if arg_num >= self.arg_count() {
            return None;
        }

        let arg_type_raw = unsafe { gfxd_sys::macro_info::gfxd_arg_type(arg_num as _) };

        ArgType::from_u32(arg_type_raw as _)
    }

    /// Returns the name of the argument with index `arg_num` or `None` if
    /// `arg_num` is larger than the argument count for the current macro.
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

        // SAFETY: The pointer given by gfxd is a nul-terminated C string, and
        // the.data should be UTF-8 already.
        let name = unsafe { utils::str_from_c_str(buf) };

        Some(name)
    }

    #[must_use]
    pub fn arg_value(&self, arg_num: u32) -> Option<ArgValue> {
        if arg_num >= self.arg_count() {
            return None;
        }

        let raw_value = unsafe { gfxd_sys::macro_info::gfxd_arg_value(arg_num as _) };
        let raw_fmt = unsafe { gfxd_sys::macro_info::gfxd_arg_fmt(arg_num as _) };

        ArgValue::new(raw_fmt, raw_value)
    }

    // TODO
    // value_by_type

    /// Returns `Some(true)` if the argument with index `arg_num` is "valid",
    /// for some definition of valid.
    ///
    /// An invalid argument generally means that the disassembler found
    /// inconsistencies in the input data, or that the data can not be
    /// reproduced by the current macro type.
    ///
    /// The argument still has a value that can be printed, though the value is
    /// not guaranteed to make any sense.
    #[must_use]
    pub fn arg_valid(&self, arg_num: u32) -> Option<bool> {
        if arg_num >= self.arg_count() {
            return None;
        }

        let arg_valid = unsafe { gfxd_sys::macro_info::gfxd_arg_valid(arg_num as _) };

        Some(arg_valid != 0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[must_use]
pub enum ArgValue {
    I(i32),
    U(u32),
    F(f32),
}

impl ArgValue {
    fn new(raw_fmt: ffi::c_int, raw_value: NonNullConst<gfxd_value_t>) -> Option<Self> {
        let arg_value = unsafe { raw_value.as_ref() };

        match raw_fmt as u32 {
            gfxd_sys::macro_info::gfxd_argfmt_i => Some(Self::I(unsafe { arg_value.i })),
            gfxd_sys::macro_info::gfxd_argfmt_u => Some(Self::U(unsafe { arg_value.u })),
            gfxd_sys::macro_info::gfxd_argfmt_f => Some(Self::F(unsafe { arg_value.f })),
            _ => None,
        }
    }

    pub(crate) fn to_gfxd_value(
        self,
    ) -> (
        gfxd_sys::macro_info::ArgFmt,
        gfxd_sys::macro_info::gfxd_value_t,
    ) {
        match self {
            ArgValue::I(x) => (
                gfxd_sys::macro_info::ArgFmt::gfxd_argfmt_i,
                gfxd_sys::macro_info::gfxd_value_t { i: x },
            ),
            ArgValue::U(x) => (
                gfxd_sys::macro_info::ArgFmt::gfxd_argfmt_u,
                gfxd_sys::macro_info::gfxd_value_t { u: x },
            ),
            ArgValue::F(x) => (
                gfxd_sys::macro_info::ArgFmt::gfxd_argfmt_f,
                gfxd_sys::macro_info::gfxd_value_t { f: x },
            ),
        }
    }
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
