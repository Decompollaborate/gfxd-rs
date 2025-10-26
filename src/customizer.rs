/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::ffi::{self, CStr};

use gfxd_sys::ptr::NonNullConst;

use crate::{lib_data::LibData, Printer};

#[allow(clippy::type_complexity)]
pub struct Customizer<'vtx> {
    // tlut_fn: Option<Box<dyn FnMut(&Printer, u32, i32, i32) -> DoDefaultOutput>>,
    vtx_fn: Option<&'vtx mut dyn FnMut(&mut Printer, u32, i32) -> DoDefaultOutput>,
}

impl<'vtx> Customizer<'vtx> {
    pub const fn new() -> Self {
        Self {
            // tlut_fn: None,
            vtx_fn: None,
        }
    }

    pub(crate) fn apply_callbacks(&mut self) {
        self.apply_out_callback();
        self.apply_user_callbacks();
    }

    fn apply_out_callback(&mut self) {
        // Write to an out buffer
        extern "C" fn output_callback(
            buf: NonNullConst<ffi::c_char>,
            count: ffi::c_int,
        ) -> ffi::c_int {
            // Retrieve the out_buf from the user data pointer.
            let lib_data = LibData::get().expect("Welp. Maybe race condition?");

            // SAFETY: We just trust gfxd to give us a valid string pointer...
            let data = unsafe { CStr::from_ptr(buf.as_ptr()) };

            // Push the output into our buffer.
            lib_data.write_to_buf(&data.to_string_lossy());

            // We read the whole buffer.
            count
        }

        // Write the disassembly output to out_buf.
        // We pass it around by passing it as a user data pointer (LibData).
        unsafe {
            gfxd_sys::io::gfxd_output_callback(Some(output_callback));
        }
    }

    fn apply_user_callbacks(&mut self) {
        if self.vtx_fn.is_some() {
            extern "C" fn callback(vtx: u32, num: i32) -> ffi::c_int {
                let lib_data = LibData::get().expect("Welp. Maybe race condition?");

                let mut printer = Printer::new();
                let ret = if let Some(closure) = &mut lib_data.get_customizer_mut().vtx_fn {
                    (closure)(&mut printer, vtx, num)
                } else {
                    panic!("vtx_fn closure was None?")
                };

                ret.into_ret()
            }
            unsafe {
                gfxd_sys::argument_callbacks::gfxd_vtx_callback(Some(callback));
            }
        } else {
            unsafe {
                gfxd_sys::argument_callbacks::gfxd_vtx_callback(None);
            }
        }
    }

    /*
    pub fn tlut_callback<F>(&mut self, callback: F)
    where
        F: FnMut(&Printer, u32, i32, i32) -> DoDefaultOutput + 'static,
    {
        self.tlut_fn = Some(Box::new(callback));
    }
    */

    pub fn vtx_callback<F>(&mut self, callback: &'vtx mut F)
    where
        F: FnMut(&mut Printer, u32, i32) -> DoDefaultOutput,
    {
        self.vtx_fn = Some(callback);
    }
}

impl<'vtx> Default for Customizer<'vtx> {
    fn default() -> Self {
        Self::new()
    }
}

pub enum DoDefaultOutput {
    DoDefault,
    Override,
}

impl DoDefaultOutput {
    #[inline]
    pub(crate) const fn into_ret(self) -> ffi::c_int {
        match self {
            DoDefaultOutput::DoDefault => 0,
            DoDefaultOutput::Override => 1,
        }
    }
}
