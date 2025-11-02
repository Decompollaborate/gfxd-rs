/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

// TODO: Use #![doc = include_str!("../README.md")] when the MSRV is 1.54+
//! A safe Rust wrapper for [glankk](https://github.com/glankk)'s C library
//! [`libgfxd`](https://github.com/glankk/libgfxd), the defacto N64 display
//! list disassembler library.
//!
//! This crate uses the [`gfxd-sys`](https://crates.io/crates/gfxd-sys) crate,
//! which exposes Raw FFI bindings for `libgfxd`, meaning this crate requires a
//! C compiler.
//!
//! ## Example usage
//!
//! The [`Disassembler`] is the entrypoint for using this crate.
//!
//! ```rust
//! use gfxd_rs::{Customizer, Disassembler, MacroPrinter, Microcode, Printer};
//!
//! // F3DEX data
//! static INPUT: [u8; 0x60] = [
//!     0xE7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//!     0xFC, 0x12, 0x7E, 0x03, 0xFF, 0xFF, 0xFD, 0xF8,
//!     0xB9, 0x00, 0x03, 0x1D, 0xC8, 0x11, 0x20, 0x78,
//!     0xB6, 0x00, 0x00, 0x00, 0x00, 0x0E, 0x00, 0x00,
//!     0xB7, 0x00, 0x00, 0x00, 0x00, 0x01, 0x20, 0x00,
//!     0xFA, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF,
//!     0x04, 0x00, 0x30, 0xBF, 0x00, 0x00, 0x02, 0xE0,
//!     0xB1, 0x00, 0x02, 0x04, 0x00, 0x02, 0x06, 0x04,
//!     0xB1, 0x08, 0x0A, 0x0C, 0x00, 0x0A, 0x0E, 0x0C,
//!     0xB1, 0x0A, 0x10, 0x12, 0x00, 0x0A, 0x12, 0x0E,
//!     0xB1, 0x14, 0x02, 0x00, 0x00, 0x14, 0x00, 0x16,
//!     0xB8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//! ];
//! static OUTPUT: &str = "\
//! {
//!     gsDPPipeSync(),
//!     gsDPSetCombineLERP(TEXEL0, 0, SHADE, 0, 0, 0, 0, 1, COMBINED, 0, PRIMITIVE, 0, 0, 0, 0, COMBINED),
//!     gsDPSetRenderMode(G_RM_FOG_SHADE_A, G_RM_AA_ZB_OPA_SURF2),
//!     gsSPClearGeometryMode(G_LIGHTING | G_TEXTURE_GEN | G_TEXTURE_GEN_LINEAR),
//!     gsSPSetGeometryMode(G_CULL_BACK | G_FOG),
//!     gsDPSetPrimColor(0, 0, 0xFF, 0xFF, 0xFF, 0xFF),
//!     gsSPVertex(0x000002E0, 12, 0),
//!     gsSP2Triangles(0, 1, 2, 0, 1, 3, 2, 0),
//!     gsSP2Triangles(4, 5, 6, 0, 5, 7, 6, 0),
//!     gsSP1Quadrangle(5, 8, 9, 7, 0),
//!     gsSP1Quadrangle(10, 1, 0, 11, 0),
//!     gsSPEndDisplayList(),
//! }
//! ";
//!
//! // Customize the generated output.
//! let mut customizer = Customizer::new();
//!
//! // Override the default macro handler to make the output prettier.
//! let mut macro_fn = |printer: &mut MacroPrinter, _info: &mut _| {
//!     // Print 4 spaces before each macro, and a comma and newline after each macro.
//!     printer.write_str("    ");
//!     // Execute the default macro handler.
//!     let ret = printer.macro_dflt();
//!     printer.write_str(",\n");
//!     ret
//! };
//! customizer.macro_fn(&mut macro_fn);
//!
//! // Print an opening and closing brace around the disassembled data.
//! let mut before = |printer: &mut Printer| {
//!     printer.write_str("{\n");
//! };
//! let mut after = |printer: &mut Printer| {
//!     printer.write_str("}\n");
//! };
//! customizer.before_after_execution_callback(&mut before, &mut after);
//!
//! // Select F3DEX as the target microcode and pass a data buffer, executing
//! // until either the end of input or encountering an invalid command.
//! let out = Disassembler::new().disassemble(&INPUT, Microcode::F3dex, &mut customizer);
//!
//! // Check the disassembly is formatted as we expect.
//! assert_eq!(OUTPUT, out);
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(improper_ctypes)]
// TODO: enable in MSRV 1.52+
// #![deny(unsafe_op_in_unsafe_fn)]

extern crate alloc;

pub(crate) mod arg_type;
pub(crate) mod customizer;
pub(crate) mod disassembler;
pub(crate) mod lib_data;
pub(crate) mod macro_id;
pub(crate) mod macro_info;
pub(crate) mod microcode;
pub(crate) mod new_types;
pub(crate) mod printer;
pub(crate) mod utils;

pub use arg_type::ArgType;
pub use customizer::{Customizer, DoDefaultOutput, MacroFnRet};
pub use disassembler::Disassembler;
pub use macro_id::MacroId;
pub use macro_info::{ArgValue, MacroInfo};
pub use microcode::Microcode;
pub use new_types::{Address, LightsNum, LookatCount, TexFmt, TexSiz, TlutCount};
pub use printer::{MacroPrinter, Printer};
