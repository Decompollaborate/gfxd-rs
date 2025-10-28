/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use pretty_assertions::assert_eq;
use std::{collections::HashMap, iter::FromIterator};

use gfxd_rs::{
    Customizer, Disassembler, DoDefaultOutput, MacroInfo, MacroPrinter, Microcode, Printer, TexFmt,
    TexSiz, TlutCount,
};

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

    let mut macro_fn = |printer: &mut MacroPrinter, _info: &mut _| {
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

    let mut macro_fn = |printer: &mut MacroPrinter, _info: &mut _| {
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
    let mut vtx_callback = |printer: &mut Printer, _info: &mut _, vtx, num| {
        vtx_tracker.insert(vtx, num);

        printer.write_str(&format!("D_{vtx:08X}"));
        DoDefaultOutput::Override
    };
    customizer.vtx_callback(&mut vtx_callback);

    let out = Disassembler::new().disassemble(&INPUT, Microcode::F3dex, &mut customizer);
    assert_eq!(OUTPUT, out);
    assert_eq!(HashMap::from_iter([(0x000002E0, 12)]), vtx_tracker);
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

    let mut macro_fn = |printer: &mut MacroPrinter, _info: &mut _| {
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
    let mut vtx_callback = |printer: &mut Printer, _info: &mut _, vtx, num| {
        vtx_tracker.insert(vtx, num);

        printer.write_str("(Vtx *)");
        DoDefaultOutput::DoDefault
    };
    customizer.vtx_callback(&mut vtx_callback);

    let out = Disassembler::new().disassemble(&INPUT, Microcode::F3dex, &mut customizer);
    assert_eq!(OUTPUT, out);
    assert_eq!(HashMap::from_iter([(0x000002E0, 12)]), vtx_tracker);
}

#[test]
fn test_macro_info() {
    static INPUT: [u8; 0x30] = [
        0x01, 0x00, 0x30, 0x06, 0x08, 0x01, 0x55, 0x40, //
        0x05, 0x00, 0x02, 0x04, 0x00, 0x00, 0x00, 0x00, //
        0xDB, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, //
        0xDC, 0x08, 0x06, 0x0A, 0x09, 0x00, 0x00, 0x08, //
        0xDC, 0x08, 0x09, 0x0A, 0x09, 0x00, 0x00, 0x00, //
        0xDF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
    ];
    static OUTPUT: &str = "\
{
    /* SPVertex (111), gsSPVertex */
    /* 0100300608015540 */
    /* arg_count: 3 */
        /* arg 0: v, type: Vtxptr, value: 0x08015540, valid: true */ /* 0x08015540 */
        /* arg 1: n, type: Num, value: 3, valid: true */ /* 3 */
        /* arg 2: v0, type: Vtx, value: 0, valid: true */ /* 0 */
    /* 0x00 */ gsSPVertex((Vtx *)0x08015540, 3, 0), /* packets: 1 */

    /* SP1Triangle (70), gsSP1Triangle */
    /* 0500020400000000 */
    /* arg_count: 4 */
        /* arg 0: v0, type: Vtx, value: 0, valid: true */ /* 0 */
        /* arg 1: v1, type: Vtx, value: 1, valid: true */ /* 1 */
        /* arg 2: v2, type: Vtx, value: 2, valid: true */ /* 2 */
        /* arg 3: flag, type: Vtxflag, value: 0, valid: true */ /* 0 */
    /* 0x08 */ gsSP1Triangle(0, 1, 2, 0), /* packets: 1 */

    /* SPSetLights1 (98), gsSPSetLights1 */
    /* DB02000000000018DC08060A09000008DC08090A09000000 */
    /* arg_count: 1 */
        /* arg 0: l, type: Lightsn, value: 0x09000000, valid: true */ /* *(Lightsn *)0x09000000 */
    /* 0x10 */ gsSPSetLights1(*(Lightsn *)0x09000000), /* packets: 3 */

    /* SPEndDisplayList (78), gsSPEndDisplayList */
    /* DF00000000000000 */
    /* arg_count: 0 */
    /* 0x28 */ gsSPEndDisplayList(), /* packets: 1 */

}
";

    let mut customizer = Customizer::new();

    let mut macro_fn = |printer: &mut MacroPrinter, info: &mut MacroInfo| {
        let macro_id = info.macro_id().unwrap();
        let macro_name = info.macro_name().unwrap();
        printer.write_str(&format!(
            "    /* {} ({}), {} */\n",
            macro_id.as_str(),
            macro_id.to_u32(),
            macro_name,
        ));

        let macro_data = info.macro_data();
        printer.write_str("    /* ");
        for x in macro_data {
            printer.write_str(&format!("{x:02X}"));
        }
        printer.write_str(" */\n");

        let arg_count = info.arg_count();
        printer.write_str(&format!("    /* arg_count: {arg_count} */\n"));
        for i in 0..arg_count {
            let arg_type = info.arg_type(i).unwrap();
            let arg_name = info.arg_name(i).unwrap();
            let arg_value = info.arg_value(i).unwrap();
            let arg_valid = info.arg_valid(i).unwrap();
            printer.write_str(&format!(
                "        /* arg {i}: {arg_name}, type: {arg_type}, value: {arg_value}, valid: {arg_valid} */"
            ));

            printer.write_str(" /* ");
            printer.write_arg_value(arg_type, &arg_value);
            printer.write_str(" */\n");
        }

        // The actual macro

        printer.write_str("    ");

        let offset = info.macro_offset();
        printer.write_str(&format!("/* 0x{offset:02X} */ "));

        let ret = printer.macro_dflt();

        printer.write_str(",");

        let packets = info.macro_packets();
        printer.write_str(&format!(" /* packets: {packets} */"));

        printer.write_str("\n\n");
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

    let mut vtx_callback = |printer: &mut Printer, _info: &mut _, _vtx, _num| {
        printer.write_str("(Vtx *)");
        DoDefaultOutput::DoDefault
    };
    customizer.vtx_callback(&mut vtx_callback);

    let out = Disassembler::new().disassemble(&INPUT, Microcode::F3dex2, &mut customizer);
    assert_eq!(OUTPUT, out);
}

fn image_callback_common(
    input: &[u8],
    microcode: Microcode,
) -> (
    String,
    HashMap<u32, (Option<u8>, TlutCount)>,
    HashMap<u32, (TexFmt, TexSiz, u8, u8, u8)>,
    HashMap<u32, i32>,
) {
    let mut customizer = Customizer::new();

    let mut macro_fn = |printer: &mut MacroPrinter, _info: &mut _| {
        printer.write_str("    ");
        let ret = printer.macro_dflt();
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

    let mut tlut_tracker = HashMap::new();
    let mut tlut_callback = |printer: &mut Printer, _info: &mut _, tlut, index, count| {
        tlut_tracker.insert(tlut, (index, count));

        printer.write_str(&format!("D_{tlut:08X}"));
        DoDefaultOutput::Override
    };
    customizer.tlut_callback(&mut tlut_callback);

    let mut timg_tracker = HashMap::new();
    let mut timg_callback =
        |printer: &mut Printer, _info: &mut _, timg, fmt, siz, width, height, pal| {
            timg_tracker.insert(timg, (fmt, siz, width, height, pal));

            printer.write_str(&format!("D_{timg:08X}"));
            DoDefaultOutput::Override
        };
    customizer.timg_callback(&mut timg_callback);

    let mut vtx_tracker = HashMap::new();
    let mut vtx_callback = |printer: &mut Printer, _info: &mut _, vtx, num| {
        vtx_tracker.insert(vtx, num);

        printer.write_str(&format!("D_{vtx:08X}"));
        DoDefaultOutput::Override
    };
    customizer.vtx_callback(&mut vtx_callback);

    let out = Disassembler::new().disassemble(input, microcode, &mut customizer);

    (out, tlut_tracker, timg_tracker, vtx_tracker)
}

#[test]
fn test_image_callback_ci4() {
    static INPUT: [u8; 0xF0] = [
        0xE7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0xE3, 0x00, 0x0A, 0x01, 0x00, 0x00, 0x00, 0x00, //
        0xD9, 0x40, 0xFD, 0xFE, 0x00, 0x00, 0x00, 0x00, //
        0xD9, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x04, 0x04, //
        0xD7, 0x00, 0x00, 0x02, 0x80, 0x00, 0x80, 0x00, //
        0xFC, 0xFF, 0xFF, 0xFF, 0xFF, 0xFC, 0xF2, 0x79, //
        0xE3, 0x00, 0x17, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0xE2, 0x00, 0x00, 0x1C, 0x00, 0x50, 0x42, 0x40, //
        0xE2, 0x00, 0x1E, 0x01, 0x00, 0x00, 0x00, 0x01, //
        0xFD, 0x10, 0x00, 0x00, 0x06, 0x00, 0x20, 0x00, //
        0xE8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0xF5, 0x00, 0x01, 0x00, 0x07, 0x00, 0x00, 0x00, //
        0xE6, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0xF0, 0x00, 0x00, 0x00, 0x07, 0x03, 0xC0, 0x00, //
        0xE7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0xFD, 0x50, 0x00, 0x00, 0x06, 0x00, 0x40, 0x00, //
        0xF5, 0x50, 0x00, 0x00, 0x07, 0x09, 0x42, 0x50, //
        0xE6, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0xF3, 0x00, 0x00, 0x00, 0x07, 0x0F, 0xF4, 0x00, //
        0xE7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0xF5, 0x40, 0x04, 0x00, 0x00, 0x09, 0x42, 0x50, //
        0xF2, 0x00, 0x00, 0x00, 0x00, 0x07, 0xC0, 0x7C, //
        0xE3, 0x00, 0x10, 0x01, 0x00, 0x00, 0x80, 0x00, //
        0xE3, 0x00, 0x12, 0x01, 0x00, 0x00, 0x20, 0x00, //
        0xE3, 0x00, 0x0C, 0x00, 0x00, 0x08, 0x00, 0x00, //
        0xE3, 0x00, 0x0F, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0xE3, 0x00, 0x0D, 0x01, 0x00, 0x00, 0x00, 0x00, //
        0x01, 0x00, 0x40, 0x08, 0x06, 0x00, 0x60, 0x00, //
        0x06, 0x04, 0x00, 0x02, 0x00, 0x06, 0x02, 0x00, //
        0xDF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
    ];
    static OUTPUT: &str = "\
{
    gsDPPipeSync(),
    gsDPSetCycleType(G_CYC_1CYCLE),
    gsSPClearGeometryMode(G_ZBUFFER | G_CULL_FRONT | G_FOG | G_LIGHTING | G_TEXTURE_GEN | G_TEXTURE_GEN_LINEAR | G_LOD | G_SHADING_SMOOTH | G_CLIPPING),
    gsSPSetGeometryMode(G_SHADE | G_CULL_BACK),
    gsSPTexture(0x8000, 0x8000, 0, G_TX_RENDERTILE, G_ON),
    gsDPSetCombineMode(G_CC_DECALRGBA, G_CC_DECALRGBA),
    gsDPSetCombineKey(G_CK_NONE),
    gsDPSetRenderMode(G_RM_XLU_SURF, G_RM_XLU_SURF2),
    gsDPSetAlphaCompare(G_AC_THRESHOLD),
    gsDPLoadTLUT_pal16(0, D_06002000),
    gsDPLoadTextureBlock_4b(D_06004000, G_IM_FMT_CI, 32, 32, 0, G_TX_NOMIRROR | G_TX_CLAMP, G_TX_NOMIRROR | G_TX_CLAMP, 5, 5, G_TX_NOLOD, G_TX_NOLOD),
    gsDPSetTextureLUT(G_TT_RGBA16),
    gsDPSetTextureFilter(G_TF_BILERP),
    gsDPSetTexturePersp(G_TP_PERSP),
    gsDPSetTextureLOD(G_TL_TILE),
    gsDPSetTextureDetail(G_TD_CLAMP),
    gsSPVertex(D_06006000, 4, 0),
    gsSP2Triangles(2, 0, 1, 0, 3, 1, 0, 0),
    gsSPEndDisplayList(),
}
";

    let (out, tlut_tracker, timg_tracker, vtx_tracker) =
        image_callback_common(&INPUT, Microcode::F3dex2);
    assert_eq!(OUTPUT, out);
    assert_eq!(
        HashMap::from_iter([(0x06002000, (Some(0), TlutCount::Pal16))]),
        tlut_tracker
    );
    assert_eq!(
        HashMap::from_iter([(0x06004000, (TexFmt::CI, TexSiz::Siz4b, 32, 32, 0))]),
        timg_tracker
    );
    assert_eq!(HashMap::from_iter([(0x06006000, 4)]), vtx_tracker);
}

#[test]
fn test_image_callback_ci8() {
    static INPUT: [u8; 0xF0] = [
        0xE7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0xE3, 0x00, 0x0A, 0x01, 0x00, 0x00, 0x00, 0x00, //
        0xD9, 0x40, 0xF9, 0xFE, 0x00, 0x00, 0x00, 0x00, //
        0xD9, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x04, //
        0xD7, 0x00, 0x00, 0x02, 0x80, 0x00, 0x80, 0x00, //
        0xFC, 0xFF, 0xFF, 0xFF, 0xFF, 0xFC, 0xF2, 0x79, //
        0xE3, 0x00, 0x17, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0xE2, 0x00, 0x00, 0x1C, 0x00, 0x50, 0x42, 0x40, //
        0xE2, 0x00, 0x1E, 0x01, 0x00, 0x00, 0x00, 0x00, //
        0xFD, 0x10, 0x00, 0x00, 0x06, 0x00, 0x20, 0x00, //
        0xE8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0xF5, 0x00, 0x01, 0x00, 0x07, 0x00, 0x00, 0x00, //
        0xE6, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0xF0, 0x00, 0x00, 0x00, 0x07, 0x3F, 0xC0, 0x00, //
        0xE7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0xFD, 0x50, 0x00, 0x00, 0x06, 0x00, 0x40, 0x00, //
        0xF5, 0x50, 0x00, 0x00, 0x07, 0x08, 0x02, 0x00, //
        0xE6, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0xF3, 0x00, 0x00, 0x00, 0x07, 0x3E, 0xF1, 0x25, //
        0xE7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0xF5, 0x48, 0x0E, 0x00, 0x00, 0x08, 0x02, 0x00, //
        0xF2, 0x00, 0x00, 0x00, 0x00, 0x0D, 0xC0, 0x8C, //
        0xE3, 0x00, 0x10, 0x01, 0x00, 0x00, 0x80, 0x00, //
        0xE3, 0x00, 0x12, 0x01, 0x00, 0x00, 0x20, 0x00, //
        0xE3, 0x00, 0x0C, 0x00, 0x00, 0x08, 0x00, 0x00, //
        0xE3, 0x00, 0x0F, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0xE3, 0x00, 0x0D, 0x01, 0x00, 0x00, 0x00, 0x00, //
        0x01, 0x00, 0x40, 0x08, 0x06, 0x00, 0x60, 0x00, //
        0x06, 0x04, 0x00, 0x02, 0x00, 0x06, 0x02, 0x00, //
        0xDF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
    ];
    static OUTPUT: &str = "\
{
    gsDPPipeSync(),
    gsDPSetCycleType(G_CYC_1CYCLE),
    gsSPClearGeometryMode(G_ZBUFFER | G_CULL_BOTH | G_FOG | G_LIGHTING | G_TEXTURE_GEN | G_TEXTURE_GEN_LINEAR | G_LOD | G_SHADING_SMOOTH | G_CLIPPING),
    gsSPSetGeometryMode(G_SHADE),
    gsSPTexture(0x8000, 0x8000, 0, G_TX_RENDERTILE, G_ON),
    gsDPSetCombineMode(G_CC_DECALRGBA, G_CC_DECALRGBA),
    gsDPSetCombineKey(G_CK_NONE),
    gsDPSetRenderMode(G_RM_XLU_SURF, G_RM_XLU_SURF2),
    gsDPSetAlphaCompare(G_AC_NONE),
    gsDPLoadTLUT_pal256(D_06002000),
    gsDPLoadTextureBlock(D_06004000, G_IM_FMT_CI, G_IM_SIZ_8b, 56, 36, 0, G_TX_NOMIRROR | G_TX_CLAMP, G_TX_NOMIRROR | G_TX_CLAMP, G_TX_NOMASK, G_TX_NOMASK, G_TX_NOLOD, G_TX_NOLOD),
    gsDPSetTextureLUT(G_TT_RGBA16),
    gsDPSetTextureFilter(G_TF_BILERP),
    gsDPSetTexturePersp(G_TP_PERSP),
    gsDPSetTextureLOD(G_TL_TILE),
    gsDPSetTextureDetail(G_TD_CLAMP),
    gsSPVertex(D_06006000, 4, 0),
    gsSP2Triangles(2, 0, 1, 0, 3, 1, 0, 0),
    gsSPEndDisplayList(),
}
";

    let (out, tlut_tracker, timg_tracker, vtx_tracker) =
        image_callback_common(&INPUT, Microcode::F3dex2);
    assert_eq!(OUTPUT, out);
    assert_eq!(
        HashMap::from_iter([(0x06002000, (None, TlutCount::Pal256))]),
        tlut_tracker
    );
    assert_eq!(
        HashMap::from_iter([(0x06004000, (TexFmt::CI, TexSiz::Siz8b, 56, 36, 0))]),
        timg_tracker
    );
    assert_eq!(HashMap::from_iter([(0x06006000, 4)]), vtx_tracker);
}
