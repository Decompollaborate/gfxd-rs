/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use gfxd_sys::ptr::NonNullConst;

use crate::{ArgType, ArgValue, MacroFnRet};

pub struct Printer {
    // Placeholder to avoid constructing this type
    _unit: (),
}

pub struct MacroPrinter {
    printer: Printer,
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

    pub fn write_arg_value(&mut self, typ: ArgType, value: &ArgValue) {
        let (_, v) = value.to_gfxd_value();
        let ptr = NonNullConst::from_ref(&v);

        unsafe {
            gfxd_sys::custom_output::gfxd_print_value(typ.into(), ptr);
        }
    }
}

impl MacroPrinter {
    // It should not be possible to construct this type by the user.
    pub(crate) const fn new() -> Self {
        Self {
            printer: Printer::new(),
        }
    }

    pub fn write_str(&mut self, s: &str) {
        self.printer.write_str(s);
    }

    pub fn write_arg_value(&mut self, typ: ArgType, value: &ArgValue) {
        self.printer.write_arg_value(typ, value);
    }

    pub fn macro_dflt(&mut self) -> MacroFnRet {
        let ret = unsafe { gfxd_sys::handlers::gfxd_macro_dflt() };

        match ret {
            0 => MacroFnRet::Continue,
            _ => MacroFnRet::Stop,
        }
    }

    pub fn arg_dflt(&mut self, arg_num: i32) {
        unsafe {
            gfxd_sys::handlers::gfxd_arg_dflt(arg_num as _);
        }
    }
}
