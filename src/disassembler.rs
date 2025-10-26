/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use alloc::string::String;
use core::ffi::{self, CStr};
use gfxd_sys::ptr::NonNullConst;

use crate::{
    lib_data::{LibData, LibDataWrap},
    Customizer, Microcode,
};

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
            let lib_data_wrap = lib_data.gfxd_set();

            unsafe { self.disassemble_impl(data, microcode, &lib_data_wrap) };
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
        _lib_data_wrap: &LibDataWrap,
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
    }
}

// `bytes` must be nul-terminated
unsafe fn c_str_from_bytes(bytes: &[u8]) -> NonNullConst<ffi::c_char> {
    let buf = unsafe { CStr::from_bytes_with_nul_unchecked(bytes) }.as_ptr();

    unsafe { NonNullConst::new_unchecked(buf) }
}

#[cfg(test)]
mod tests {
    use core::iter::FromIterator;

    use crate::{Customizer, DoDefaultOutput, Printer};

    use super::*;

    use alloc::{collections::btree_map::BTreeMap, format};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_basic() {
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

        let mut customizer = Customizer::new();

        let out = Disassembler::new().disassemble(&INPUT, Microcode::F3dex, &mut customizer);
        assert_eq!(OUTPUT, out);
    }

    #[test]
    fn test_vtx_callback() {
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
    gsSPVertex(D_000002E0, 12, 0),
    gsSP2Triangles(0, 1, 2, 0, 1, 3, 2, 0),
    gsSP2Triangles(4, 5, 6, 0, 5, 7, 6, 0),
    gsSP1Quadrangle(5, 8, 9, 7, 0),
    gsSP1Quadrangle(10, 1, 0, 11, 0),
    gsSPEndDisplayList(),
}
";

        let mut customizer = Customizer::new();

        let mut vtx_tracker = BTreeMap::new();

        let mut vtx_callback = |printer: &mut Printer, vtx, num| {
            vtx_tracker.insert(vtx, num);

            printer.write_str(&format!("D_{vtx:08X}"));
            DoDefaultOutput::Override
        };
        customizer.vtx_callback(&mut vtx_callback);

        let out = Disassembler::new().disassemble(&INPUT, Microcode::F3dex, &mut customizer);
        assert_eq!(OUTPUT, out);
        assert_eq!(vtx_tracker, BTreeMap::from_iter([(0x000002E0, 12)]));
    }

    #[test]
    fn test_vtx_callback_default() {
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
    gsSPVertex((Vtx *)0x000002E0, 12, 0),
    gsSP2Triangles(0, 1, 2, 0, 1, 3, 2, 0),
    gsSP2Triangles(4, 5, 6, 0, 5, 7, 6, 0),
    gsSP1Quadrangle(5, 8, 9, 7, 0),
    gsSP1Quadrangle(10, 1, 0, 11, 0),
    gsSPEndDisplayList(),
}
";

        let mut customizer = Customizer::new();

        let mut vtx_tracker = BTreeMap::new();

        let mut vtx_callback = |printer: &mut Printer, vtx, num| {
            vtx_tracker.insert(vtx, num);

            printer.write_str("(Vtx *)");
            DoDefaultOutput::DoDefault
        };
        customizer.vtx_callback(&mut vtx_callback);

        let out = Disassembler::new().disassemble(&INPUT, Microcode::F3dex, &mut customizer);
        assert_eq!(OUTPUT, out);
        assert_eq!(vtx_tracker, BTreeMap::from_iter([(0x000002E0, 12)]));
    }
}
