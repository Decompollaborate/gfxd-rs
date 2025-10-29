/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use alloc::string::String;
use gfxd_sys::ptr::NonNullConst;

use crate::{
    lib_data::{LibData, LibDataWrap},
    Customizer, Microcode,
};

// TODO: figure out where and how to expose gfxd_arg_callbacks

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Disassembler {
    // Placeholder to avoid constructing this type
    _unit: (),
    // TODO: endian and wordsize
    // TODO: dynamic
    // TODO: enable/disable cap
}

// TODO: expose macro information inside the custom handlers and callbacks

impl Disassembler {
    #[must_use]
    #[expect(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self { _unit: () }
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

        {
            let mut lib_data_wrap = lib_data.gfxd_set();

            unsafe { self.disassemble_impl(data, microcode, &mut lib_data_wrap) };
        }

        lib_data.consume()
    }

    /// # SAFETY
    ///
    /// `gfxd_udata_set` must have been called with a valid pointer to the
    /// current `LibData` instance.
    // Use a wrapper function to make sure lib_data does not get dropped too
    // soon.
    unsafe fn disassemble_impl(
        self,
        data: &[u8],
        microcode: Microcode,
        lib_data_wrap: &mut LibDataWrap,
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

        // Run
        lib_data_wrap.do_before();
        unsafe {
            gfxd_sys::execution::gfxd_execute();
        }
        lib_data_wrap.do_after();
    }
}
