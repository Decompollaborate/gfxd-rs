/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use pretty_assertions::assert_eq;
use std::{collections::HashMap, iter::FromIterator};

use gfxd_rs::{Customizer, Disassembler, DoDefaultOutput, MacroPrinter, Microcode, Printer};

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

    let mut macro_fn = |printer: &mut MacroPrinter| {
        /* Print a 4 spaces before each macro, and a comma and newline after each macro */
        printer.write_str("    ");
        let ret = printer.macro_dflt(); /* Execute the default macro handler */
        printer.write_str(",\n");
        ret
    };
    customizer.macro_fn(&mut macro_fn);

    /*
    let mut arg_fn = |printer: &mut MacroPrinter, arg_num| {
        printer.arg_dflt(arg_num);
    };
    customizer.arg_fn(&mut arg_fn);
    */

    let mut before = |printer: &mut Printer| {
        printer.write_str("{\n");
    };
    let mut after = |printer: &mut Printer| {
        printer.write_str("}\n");
    };
    customizer.before_after_execution_callback(&mut before, &mut after);

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

    let mut macro_fn = |printer: &mut MacroPrinter| {
        /* Print a 4 spaces before each macro, and a comma and newline after each macro */
        printer.write_str("    ");
        let ret = printer.macro_dflt(); /* Execute the default macro handler */
        printer.write_str(",\n");
        ret
    };
    customizer.macro_fn(&mut macro_fn);

    let mut before = |printer: &mut Printer| {
        printer.write_str("{\n");
    };
    let mut after = |printer: &mut Printer| {
        printer.write_str("}\n");
    };
    customizer.before_after_execution_callback(&mut before, &mut after);

    let mut vtx_tracker = HashMap::new();
    let mut vtx_callback = |printer: &mut Printer, vtx, num| {
        vtx_tracker.insert(vtx, num);

        printer.write_str(&format!("D_{vtx:08X}"));
        DoDefaultOutput::Override
    };
    customizer.vtx_callback(&mut vtx_callback);

    let out = Disassembler::new().disassemble(&INPUT, Microcode::F3dex, &mut customizer);
    assert_eq!(OUTPUT, out);
    assert_eq!(vtx_tracker, HashMap::from_iter([(0x000002E0, 12)]));
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

    let mut macro_fn = |printer: &mut MacroPrinter| {
        /* Print a 4 spaces before each macro, and a comma and newline after each macro */
        printer.write_str("    ");
        let ret = printer.macro_dflt(); /* Execute the default macro handler */
        printer.write_str(",\n");
        ret
    };
    customizer.macro_fn(&mut macro_fn);

    let mut before = |printer: &mut Printer| {
        printer.write_str("{\n");
    };
    let mut after = |printer: &mut Printer| {
        printer.write_str("}\n");
    };
    customizer.before_after_execution_callback(&mut before, &mut after);

    let mut vtx_tracker = HashMap::new();
    let mut vtx_callback = |printer: &mut Printer, vtx, num| {
        vtx_tracker.insert(vtx, num);

        printer.write_str("(Vtx *)");
        DoDefaultOutput::DoDefault
    };
    customizer.vtx_callback(&mut vtx_callback);

    let out = Disassembler::new().disassemble(&INPUT, Microcode::F3dex, &mut customizer);
    assert_eq!(OUTPUT, out);
    assert_eq!(vtx_tracker, HashMap::from_iter([(0x000002E0, 12)]));
}
