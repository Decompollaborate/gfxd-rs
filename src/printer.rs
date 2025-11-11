/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use gfxd_sys::ptr::NonNullConst;

use crate::{ArgType, ArgValue, MacroFnRet};

/// An utility that allows writing to `gfxd`'s output buffer from inside
/// user-defined callbacks.
pub struct Printer {
    // Placeholder to avoid constructing this type
    _unit: (),
}

/// An utility to write to `gfxd`'s output buffer from macro-specific
/// user-defined callbacks.
pub struct MacroPrinter {
    printer: Printer,
}

impl Printer {
    // It should not be possible to construct this type by library consumers.
    pub(crate) const fn new() -> Self {
        Self { _unit: () }
    }

    /// Write the given str to the output buffer.
    pub fn write_str(&mut self, s: &str) {
        let buf = NonNullConst::from_ref(s).cast();
        // s.len() is the number of bytes in str instead of number of chars
        #[allow(clippy::cast_possible_truncation)]
        let len = s.len() as _;

        unsafe {
            gfxd_sys::custom_output::gfxd_write(buf, len);
        }
    }

    /// Write the given Argument Value to the output buffer.
    pub fn write_arg_value(&mut self, typ: ArgType, value: &ArgValue) {
        let (_, v) = value.to_gfxd_value();
        let ptr = NonNullConst::from_ref(&v);

        unsafe {
            gfxd_sys::custom_output::gfxd_print_value(typ.into(), ptr);
        }
    }
}

impl MacroPrinter {
    // It should not be possible to construct this type by library consumers.
    pub(crate) const fn new() -> Self {
        Self {
            printer: Printer::new(),
        }
    }

    /// Write the given str to the output buffer.
    pub fn write_str(&mut self, s: &str) {
        self.printer.write_str(s);
    }

    /// Write the given Argument Value to the output buffer.
    pub fn write_arg_value(&mut self, typ: ArgType, value: &ArgValue) {
        self.printer.write_arg_value(typ, value);
    }

    /// The default macro handler.
    ///
    /// Outputs the macro name, dynamic display list pointer if one has been
    /// specified, and then each argument in order using the function
    /// registered using [`arg_fn`] ([`arg_dflt`] by default).
    ///
    /// Because it is designed to be extended, it only outputs the macro text,
    /// without any whitespace or punctuation before or after.
    ///
    /// [`arg_fn`]: crate::Customizer::arg_fn
    /// [`arg_dflt`]: MacroPrinter::arg_dflt
    pub fn macro_dflt(&mut self) -> MacroFnRet {
        let ret = unsafe { gfxd_sys::handlers::gfxd_macro_dflt() };

        match ret {
            0 => MacroFnRet::Continue,
            _ => MacroFnRet::Stop,
        }
    }

    /// The default argument handler for [`macro_dflt`].
    ///
    /// For the argument with index `arg_num`, calls `arg_callbacks`, and
    /// prints the argument value if the callback returns zero, or if there is
    /// no callback for the given argument.
    ///
    /// [`macro_dflt`]: MacroPrinter::macro_dflt
    pub fn arg_dflt(&mut self, arg_num: i32) {
        unsafe {
            gfxd_sys::handlers::gfxd_arg_dflt(arg_num as _);
        }
    }
}
