/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use alloc::string::{String, ToString};
use core::ffi;
use gfxd_sys::ptr::NonNullConst;

use crate::{
    lib_data::{LibData, LibDataWrap},
    Customizer, Microcode,
};

// TODO: figure out where and how to expose gfxd_arg_callbacks

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Disassembler<'d> {
    // TODO: endian and wordsize
    dynamic: Option<&'d str>,
    stop_on_invalid: bool,
    stop_on_end: bool,
    emit_dec_color: bool,
    emit_q_macro: bool,
    emit_ext_macro: bool,
}

impl<'d> Disassembler<'d> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            dynamic: None,
            stop_on_invalid: true,
            stop_on_end: true,
            emit_dec_color: false,
            emit_q_macro: false,
            emit_ext_macro: false,
        }
    }

    pub fn dynamic(&mut self, dynamic: Option<&'d str>) -> &mut Self {
        self.dynamic = dynamic;
        self
    }

    pub fn stop_on_invalid(&mut self, value: bool) -> &mut Self {
        self.stop_on_invalid = value;
        self
    }
    pub fn stop_on_end(&mut self, value: bool) -> &mut Self {
        self.stop_on_end = value;
        self
    }
    pub fn emit_dec_color(&mut self, value: bool) -> &mut Self {
        self.emit_dec_color = value;
        self
    }
    pub fn emit_q_macro(&mut self, value: bool) -> &mut Self {
        self.emit_q_macro = value;
        self
    }
    pub fn emit_ext_macro(&mut self, value: bool) -> &mut Self {
        self.emit_ext_macro = value;
        self
    }

    #[must_use]
    pub fn disassemble(
        self,
        data: &[u8],
        microcode: Microcode,
        customizer: &mut Customizer,
    ) -> String {
        let mut lib_data = LibData::new(customizer);
        // Make sure we don't get surprises by the data somehow moving.
        // tbh I'm not sure if this is needed at all, I need to investigate.
        // let lib_data = alloc::boxed::Box::pin(lib_data);

        // We need to ensure this String doesn't get dropped before we execute
        // gfxd. Allocating it outside disassemble_impl and passing the
        // reference should ensure that, as far as I understand it.
        let dynamic = self.dynamic.map(|x| {
            let mut x = x.to_string();
            // nul-terminate the string
            x.push('\0');
            x
        });
        // `as_ref` is needed, otherwise the string gets consumed and this ends
        // up pointing to itself or something weird like that.
        let dynamic_ptr = dynamic
            .as_ref()
            .and_then(|x| NonNullConst::new(x.as_ptr().cast()));

        {
            let mut lib_data_wrap = lib_data.gfxd_set();

            self.disassemble_impl(data, microcode, &mut lib_data_wrap, dynamic_ptr);
        }

        lib_data.consume()
    }

    // Use a wrapper function to make sure lib_data does not get dropped too
    // soon.
    fn disassemble_impl(
        self,
        data: &[u8],
        microcode: Microcode,
        lib_data_wrap: &mut LibDataWrap,
        dynamic: Option<NonNullConst<ffi::c_char>>,
    ) {
        // Setup input and output
        unsafe {
            // We only use the input_buffer and the output_callback functions
            // of libgfxd, completely ignoring the output_buffer, input_callback
            // or the fd ones.
            // I don't know if it is worth to expose those in the API.

            gfxd_sys::io::gfxd_input_buffer(NonNullConst::new_void(data.as_ptr()), data.len() as _);
        }

        // Set the microcode
        unsafe {
            gfxd_sys::settings::gfxd_target(Some(microcode.to_microcode_ptr()));
        }

        // Set the dynamic arg, if any
        unsafe {
            gfxd_sys::settings::gfxd_dynamic(dynamic);
        }

        // Set the options
        set_feature_option(
            self.stop_on_invalid,
            gfxd_sys::settings::FeatureOption::gfxd_stop_on_invalid,
        );
        set_feature_option(
            self.stop_on_end,
            gfxd_sys::settings::FeatureOption::gfxd_stop_on_end,
        );
        set_feature_option(
            self.emit_dec_color,
            gfxd_sys::settings::FeatureOption::gfxd_emit_dec_color,
        );
        set_feature_option(
            self.emit_q_macro,
            gfxd_sys::settings::FeatureOption::gfxd_emit_q_macro,
        );
        set_feature_option(
            self.emit_ext_macro,
            gfxd_sys::settings::FeatureOption::gfxd_emit_ext_macro,
        );

        // Run
        lib_data_wrap.do_before();
        unsafe {
            gfxd_sys::execution::gfxd_execute();
        }
        lib_data_wrap.do_after();
    }
}

fn set_feature_option(on: bool, cap: gfxd_sys::settings::FeatureOption) {
    unsafe {
        if on {
            gfxd_sys::settings::gfxd_enable(cap);
        } else {
            gfxd_sys::settings::gfxd_disable(cap);
        }
    }
}

impl Default for Disassembler<'_> {
    fn default() -> Self {
        Self::new()
    }
}
