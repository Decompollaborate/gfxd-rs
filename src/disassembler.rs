/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use alloc::string::String;
use core::ffi::{self, CStr};
use gfxd_sys::ptr::{NonNullConst, NonNullMut};

use crate::Microcode;

#[must_use]
pub struct Disassembler {
    // Placeholder to avoid constructing this type
    _unit: (),
}

impl Disassembler {
    #[expect(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self { _unit: () }
    }

    #[must_use]
    pub fn disassemble(self, data: &[u8], microcode: Microcode) -> String {
        // Write to an out buffer
        extern "C" fn output_callback(buf: *const ffi::c_char, count: ffi::c_int) -> ffi::c_int {
            // Retrieve the out_buf from the user data pointer.
            let user_data = unsafe { gfxd_sys::settings::gfxd_udata_get() }
                .expect("Did we forget to setup the user data pointer with gfxd_udata_set ??");
            let out_buf = unsafe { user_data.cast::<String>().as_mut() };

            // SAFETY: We just trust gfxd to give us a valid string pointer...
            let data = unsafe { CStr::from_ptr(buf) };

            // Push the output into our buffer.
            out_buf.push_str(&data.to_string_lossy());

            // We read the whole buffer.
            count
        }

        let mut out_buf = String::new();

        // Setup
        unsafe {
            gfxd_sys::settings::gfxd_target(Some(microcode.to_microcode_ptr()));
            gfxd_sys::io::gfxd_input_buffer(NonNullConst::new_void(data.as_ptr()), data.len() as _);

            // Write the disassembly output to out_buf.
            // We pass it around by passing it as a user data pointer.
            gfxd_sys::settings::gfxd_udata_set(NonNullMut::new_void(&mut out_buf));
            gfxd_sys::io::gfxd_output_callback(Some(output_callback));
        }

        // Make the output pretier
        extern "C" fn macro_fn() -> ffi::c_int {
            unsafe {
                /* Print a tab before each macro, and a comma and newline after each macro */
                gfxd_sys::custom_output::gfxd_puts(c_str_from_bytes(b"    \0"));
                gfxd_sys::handlers::gfxd_macro_dflt(); /* Execute the default macro handler */
                gfxd_sys::custom_output::gfxd_puts(c_str_from_bytes(b",\n\0"));
            }
            0
        }
        unsafe {
            gfxd_sys::handlers::gfxd_macro_fn(Some(macro_fn));
        }

        // Run
        unsafe {
            gfxd_sys::custom_output::gfxd_puts(c_str_from_bytes(b"{\n\0"));
            gfxd_sys::execution::gfxd_execute();
            gfxd_sys::custom_output::gfxd_puts(c_str_from_bytes(b"}\n\0"));
        }

        out_buf
    }
}

// `bytes` must be nul-terminated
unsafe fn c_str_from_bytes(bytes: &[u8]) -> NonNullConst<ffi::c_char> {
    let buf = unsafe { CStr::from_bytes_with_nul_unchecked(bytes) }.as_ptr();

    unsafe { NonNullConst::new_unchecked(buf) }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn asdf() {
        static INPUT: [u8; 0x60] = [
            0xE7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
            0xFC, 0x12, 0x7E, 0x03, 0xFF, 0xFF, 0xFD, 0xF8, //
            0xB9, 0x00, 0x03, 0x1D, 0xC8, 0x11, 0x20, 0x78, //
            0xB6, 0x00, 0x00, 0x00, 0x00, 0x0E, 0x00, 0x00, //
            0xB7, 0x00, 0x00, 0x00, 0x00, 0x01, 0x20, 0x00, //
            0xFA, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, //
            0x04, 0x00, 0x30, 0xBF, 0x00, 0x00, 0x02, 0xE0, //
            0xB1, 0x00, 0x02, 0x04, 0x00, 0x02, 0x06, 0x04, //
            0xB1, 0x08, 0x0A, 0x0C, 0x00, 0x0A, 0x0E, 0x0C, //
            0xB1, 0x0A, 0x10, 0x12, 0x00, 0x0A, 0x12, 0x0E, //
            0xB1, 0x14, 0x02, 0x00, 0x00, 0x14, 0x00, 0x16, //
            0xB8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        ];
        static OUTPUT: &str = "\
{
    gsDPPipeSync(),
    gsDPSetCombineLERP(TEXEL0, 0, SHADE, 0, 0, 0, 0, 1, COMBINED, 0, PRIMITIVE, 0, 0, 0, 0, COMBINED),
    gsDPSetRenderMode(G_RM_FOG_SHADE_A, G_RM_AA_ZB_OPA_SURF2),
    gsSPClearGeometryMode(G_LIGHTING | G_TEXTURE_GEN | G_TEXTURE_GEN_LINEAR),
    gsSPSetGeometryMode(G_CULL_BACK | G_FOG),
    gsDPSetPrimColor(0, 0, 0xFF, 0xFF, 0xFF, 0xFF),
    gsSPVertex(0x000002E0, 12, 0),
    gsSP2Triangles(0, 1, 2, 0, 1, 3, 2, 0),
    gsSP2Triangles(4, 5, 6, 0, 5, 7, 6, 0),
    gsSP1Quadrangle(5, 8, 9, 7, 0),
    gsSP1Quadrangle(10, 1, 0, 11, 0),
    gsSPEndDisplayList(),
}
";

        let out = Disassembler::new().disassemble(&INPUT, Microcode::F3dex);
        assert_eq!(OUTPUT, out);
    }
}
