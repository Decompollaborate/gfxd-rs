/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use pretty_assertions::assert_eq;
use std::{
    collections::HashMap,
    iter::FromIterator,
    num::{NonZeroU16, NonZeroU32},
};

use gfxd_rs::{
    Address, Customizer, Disassembler, DoDefaultOutput, LightsNum, LookatCount, MacroInfo,
    MacroPrinter, Microcode, Printer, TexFmt, TexSiz, TlutCount,
};

#[derive(Debug, Default, PartialEq, Eq)]
struct Tracker {
    tlut: HashMap<Address, (Option<u8>, TlutCount)>,
    timg: HashMap<Address, (TexFmt, TexSiz, u8, u8, u8)>,
    cimg: HashMap<Address, (TexFmt, TexSiz, u16)>,
    zimg: HashMap<Address, ()>,
    dl: HashMap<Address, ()>,
    mtx: HashMap<Address, ()>,
    lookat: HashMap<Address, (LookatCount,)>,
    light: HashMap<Address, ()>,
    lightsn: HashMap<Address, (LightsNum,)>,
    seg: HashMap<Address, (u8,)>,
    vtx: HashMap<Address, (i32,)>,
    vp: HashMap<Address, ()>,
    uctext: HashMap<Address, (NonZeroU32,)>,
    ucdata: HashMap<Address, (NonZeroU32,)>,
    dram: HashMap<Address, (NonZeroU16,)>,
}

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

        printer.write_str(&format!("D_{vtx}"));
        DoDefaultOutput::Override
    };
    customizer.vtx_callback(&mut vtx_callback);

    let out = Disassembler::new().disassemble(&INPUT, Microcode::F3dex, &mut customizer);
    assert_eq!(OUTPUT, out);
    assert_eq!(HashMap::from_iter([(Address(0x000002E0), 12)]), vtx_tracker);
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
    assert_eq!(HashMap::from_iter([(Address(0x000002E0), 12)]), vtx_tracker);
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

fn callback_common(input: &[u8], microcode: Microcode) -> (String, Tracker) {
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

        printer.write_str(&format!("D_{tlut}"));
        DoDefaultOutput::Override
    };
    customizer.tlut_callback(&mut tlut_callback);

    let mut timg_tracker = HashMap::new();
    let mut timg_callback =
        |printer: &mut Printer, _info: &mut _, timg, fmt, siz, width, height, pal| {
            timg_tracker.insert(timg, (fmt, siz, width, height, pal));

            printer.write_str(&format!("D_{timg}"));
            DoDefaultOutput::Override
        };
    customizer.timg_callback(&mut timg_callback);

    let mut cimg_tracker = HashMap::new();
    let mut cimg_callback = |printer: &mut Printer, _info: &mut _, cimg, fmt, siz, width| {
        cimg_tracker.insert(cimg, (fmt, siz, width));

        printer.write_str(&format!("D_{cimg}"));
        DoDefaultOutput::Override
    };
    customizer.cimg_callback(&mut cimg_callback);

    let mut zimg_tracker = HashMap::new();
    let mut zimg_callback = |printer: &mut Printer, _info: &mut _, zimg| {
        zimg_tracker.insert(zimg, ());

        printer.write_str(&format!("D_{zimg}"));
        DoDefaultOutput::Override
    };
    customizer.zimg_callback(&mut zimg_callback);

    let mut dl_tracker = HashMap::new();
    let mut dl_callback = |printer: &mut Printer, _info: &mut _, dl| {
        dl_tracker.insert(dl, ());

        printer.write_str(&format!("D_{dl}"));
        DoDefaultOutput::Override
    };
    customizer.dl_callback(&mut dl_callback);

    let mut mtx_tracker = HashMap::new();
    let mut mtx_callback = |printer: &mut Printer, _info: &mut _, mtx| {
        mtx_tracker.insert(mtx, ());

        printer.write_str(&format!("D_{mtx}"));
        DoDefaultOutput::Override
    };
    customizer.mtx_callback(&mut mtx_callback);

    let mut lookat_tracker = HashMap::new();
    let mut lookat_callback = |printer: &mut Printer, _info: &mut _, lookat, count| {
        lookat_tracker.insert(lookat, (count,));

        printer.write_str(&format!("D_{lookat}"));
        DoDefaultOutput::Override
    };
    customizer.lookat_callback(&mut lookat_callback);

    let mut light_tracker = HashMap::new();
    let mut light_callback = |printer: &mut Printer, _info: &mut _, light| {
        light_tracker.insert(light, ());

        printer.write_str(&format!("D_{light}"));
        DoDefaultOutput::Override
    };
    customizer.light_callback(&mut light_callback);

    let mut lightsn_tracker = HashMap::new();
    let mut lightsn_callback = |printer: &mut Printer, _info: &mut _, lightsn, num| {
        lightsn_tracker.insert(lightsn, (num,));

        printer.write_str(&format!("D_{lightsn}"));
        DoDefaultOutput::Override
    };
    customizer.lightsn_callback(&mut lightsn_callback);

    let mut seg_tracker = HashMap::new();
    let mut seg_callback = |printer: &mut Printer, _info: &mut _, seg, num| {
        seg_tracker.insert(seg, (num,));

        printer.write_str(&format!("D_{seg}"));
        DoDefaultOutput::Override
    };
    customizer.seg_callback(&mut seg_callback);

    let mut vtx_tracker = HashMap::new();
    let mut vtx_callback = |printer: &mut Printer, _info: &mut _, vtx, num| {
        vtx_tracker.insert(vtx, (num,));

        printer.write_str(&format!("D_{vtx}"));
        DoDefaultOutput::Override
    };
    customizer.vtx_callback(&mut vtx_callback);

    let mut vp_tracker = HashMap::new();
    let mut vp_callback = |printer: &mut Printer, _info: &mut _, vp| {
        vp_tracker.insert(vp, ());

        printer.write_str(&format!("D_{vp}"));
        DoDefaultOutput::Override
    };
    customizer.vp_callback(&mut vp_callback);

    let mut uctext_tracker = HashMap::new();
    let mut uctext_callback = |printer: &mut Printer, _info: &mut _, uctext, num| {
        uctext_tracker.insert(uctext, (num,));

        printer.write_str(&format!("D_{uctext}"));
        DoDefaultOutput::Override
    };
    customizer.uctext_callback(&mut uctext_callback);

    let mut ucdata_tracker = HashMap::new();
    let mut ucdata_callback = |printer: &mut Printer, _info: &mut _, ucdata, num| {
        ucdata_tracker.insert(ucdata, (num,));

        printer.write_str(&format!("D_{ucdata}"));
        DoDefaultOutput::Override
    };
    customizer.ucdata_callback(&mut ucdata_callback);

    let mut dram_tracker = HashMap::new();
    let mut dram_callback = |printer: &mut Printer, _info: &mut _, dram, num| {
        dram_tracker.insert(dram, (num,));

        printer.write_str(&format!("D_{dram}"));
        DoDefaultOutput::Override
    };
    customizer.dram_callback(&mut dram_callback);

    let out = Disassembler::new().disassemble(input, microcode, &mut customizer);

    let tracker = Tracker {
        tlut: tlut_tracker,
        timg: timg_tracker,
        cimg: cimg_tracker,
        zimg: zimg_tracker,
        dl: dl_tracker,
        mtx: mtx_tracker,
        lookat: lookat_tracker,
        light: light_tracker,
        lightsn: lightsn_tracker,
        seg: seg_tracker,
        vtx: vtx_tracker,
        vp: vp_tracker,
        uctext: uctext_tracker,
        ucdata: ucdata_tracker,
        dram: dram_tracker,
    };
    (out, tracker)
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

    let (out, tracker) = callback_common(&INPUT, Microcode::F3dex2);
    assert_eq!(OUTPUT, out);

    let expected_tracker = Tracker {
        tlut: HashMap::from_iter([(Address(0x06002000), (Some(0), TlutCount::Pal16))]),
        timg: HashMap::from_iter([(Address(0x06004000), (TexFmt::CI, TexSiz::Siz4b, 32, 32, 0))]),
        vtx: HashMap::from_iter([(Address(0x06006000), (4,))]),
        ..Tracker::default()
    };
    assert_eq!(expected_tracker, tracker);
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

    let (out, tracker) = callback_common(&INPUT, Microcode::F3dex2);
    assert_eq!(OUTPUT, out);

    let expected_tracker = Tracker {
        tlut: HashMap::from_iter([(Address(0x06002000), (None, TlutCount::Pal256))]),
        timg: HashMap::from_iter([(Address(0x06004000), (TexFmt::CI, TexSiz::Siz8b, 56, 36, 0))]),
        vtx: HashMap::from_iter([(Address(0x06006000), (4,))]),
        ..Tracker::default()
    };
    assert_eq!(expected_tracker, tracker);
}

#[test]
fn test_framebuffer() {
    static INPUT: [u8; 0x18] = [
        0xFF, 0x10, 0x01, 0x3F, 0x80, 0x80, 0x00, 0x00, //
        0xFE, 0x00, 0x00, 0x00, 0x80, 0x90, 0x00, 0x00, //
        0xDF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
    ];
    static OUTPUT: &str = "\
{
    gsDPSetColorImage(G_IM_FMT_RGBA, G_IM_SIZ_16b, 320, D_80800000),
    gsDPSetDepthImage(D_80900000),
    gsSPEndDisplayList(),
}
";

    let (out, tracker) = callback_common(&INPUT, Microcode::F3dex2);
    assert_eq!(OUTPUT, out);

    let expected_tracker = Tracker {
        cimg: HashMap::from_iter([(Address(0x80800000), (TexFmt::Rgba, TexSiz::Siz16b, 320))]),
        zimg: HashMap::from_iter([(Address(0x80900000), ())]),
        ..Tracker::default()
    };
    assert_eq!(expected_tracker, tracker);
}

#[test]
fn test_dl_mtx() {
    static INPUT: [u8; 0x18] = [
        0xDE, 0x00, 0x00, 0x00, 0x05, 0x00, 0x02, 0x00, //
        0xDA, 0x38, 0x00, 0x07, 0x05, 0x00, 0x04, 0x00, //
        0xDE, 0x01, 0x00, 0x00, 0x05, 0x00, 0x06, 0x00, //
    ];
    static OUTPUT: &str = "\
{
    gsSPDisplayList(D_05000200),
    gsSPMatrix(D_05000400, G_MTX_NOPUSH | G_MTX_LOAD | G_MTX_PROJECTION),
    gsSPBranchList(D_05000600),
}
";

    let (out, tracker) = callback_common(&INPUT, Microcode::F3dex2);
    assert_eq!(OUTPUT, out);

    let expected_tracker = Tracker {
        dl: HashMap::from_iter([(Address(0x05000200), ()), (Address(0x05000600), ())]),
        mtx: HashMap::from_iter([(Address(0x05000400), ())]),
        ..Tracker::default()
    };
    assert_eq!(expected_tracker, tracker);
}

#[test]
fn test_look_at_viewport() {
    static INPUT: [u8; 0x30] = [
        0xDC, 0x08, 0x00, 0x0A, 0x05, 0x00, 0x02, 0x20, //
        0xDC, 0x08, 0x03, 0x0A, 0x05, 0x00, 0x02, 0x30, //
        0xDC, 0x08, 0x03, 0x0A, 0x05, 0x00, 0x02, 0x10, //
        0xDC, 0x08, 0x00, 0x0A, 0x05, 0x00, 0x02, 0x00, //
        0xDC, 0x08, 0x00, 0x08, 0x06, 0x00, 0x60, 0x00, //
        0xDF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
    ];
    static OUTPUT: &str = "\
{
    gsSPLookAt(D_05000220),
    gsSPLookAtY(D_05000210),
    gsSPLookAtX(D_05000200),
    gsSPViewport(D_06006000),
    gsSPEndDisplayList(),
}
";

    let (out, tracker) = callback_common(&INPUT, Microcode::F3dex2);
    assert_eq!(OUTPUT, out);

    let expected_tracker = Tracker {
        lookat: HashMap::from_iter([
            (Address(0x05000220), (LookatCount::N2,)),
            (Address(0x05000210), (LookatCount::N1,)),
            (Address(0x05000200), (LookatCount::N1,)),
        ]),
        vp: HashMap::from_iter([(Address(0x06006000), ())]),
        ..Tracker::default()
    };
    assert_eq!(expected_tracker, tracker);
}

#[test]
fn test_lights() {
    static INPUT: [u8; 0x160] = [
        0xDC, 0x08, 0x06, 0x0A, 0x05, 0x00, 0x02, 0x00, //
        0xDB, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, //
        0xDC, 0x08, 0x06, 0x0A, 0x05, 0x00, 0x10, 0x08, //
        0xDC, 0x08, 0x09, 0x0A, 0x05, 0x00, 0x10, 0x00, //
        0xDB, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30, //
        0xDC, 0x08, 0x06, 0x0A, 0x05, 0x00, 0x20, 0x08, //
        0xDC, 0x08, 0x09, 0x0A, 0x05, 0x00, 0x20, 0x18, //
        0xDC, 0x08, 0x0C, 0x0A, 0x05, 0x00, 0x20, 0x00, //
        0xDB, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x48, //
        0xDC, 0x08, 0x06, 0x0A, 0x05, 0x00, 0x30, 0x08, //
        0xDC, 0x08, 0x09, 0x0A, 0x05, 0x00, 0x30, 0x18, //
        0xDC, 0x08, 0x0C, 0x0A, 0x05, 0x00, 0x30, 0x28, //
        0xDC, 0x08, 0x0F, 0x0A, 0x05, 0x00, 0x30, 0x00, //
        0xDB, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x60, //
        0xDC, 0x08, 0x06, 0x0A, 0x05, 0x00, 0x40, 0x08, //
        0xDC, 0x08, 0x09, 0x0A, 0x05, 0x00, 0x40, 0x18, //
        0xDC, 0x08, 0x0C, 0x0A, 0x05, 0x00, 0x40, 0x28, //
        0xDC, 0x08, 0x0F, 0x0A, 0x05, 0x00, 0x40, 0x38, //
        0xDC, 0x08, 0x12, 0x0A, 0x05, 0x00, 0x40, 0x00, //
        0xDB, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x78, //
        0xDC, 0x08, 0x06, 0x0A, 0x05, 0x00, 0x50, 0x08, //
        0xDC, 0x08, 0x09, 0x0A, 0x05, 0x00, 0x50, 0x18, //
        0xDC, 0x08, 0x0C, 0x0A, 0x05, 0x00, 0x50, 0x28, //
        0xDC, 0x08, 0x0F, 0x0A, 0x05, 0x00, 0x50, 0x38, //
        0xDC, 0x08, 0x12, 0x0A, 0x05, 0x00, 0x50, 0x48, //
        0xDC, 0x08, 0x15, 0x0A, 0x05, 0x00, 0x50, 0x00, //
        0xDB, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x90, //
        0xDC, 0x08, 0x06, 0x0A, 0x05, 0x00, 0x60, 0x08, //
        0xDC, 0x08, 0x09, 0x0A, 0x05, 0x00, 0x60, 0x18, //
        0xDC, 0x08, 0x0C, 0x0A, 0x05, 0x00, 0x60, 0x28, //
        0xDC, 0x08, 0x0F, 0x0A, 0x05, 0x00, 0x60, 0x38, //
        0xDC, 0x08, 0x12, 0x0A, 0x05, 0x00, 0x60, 0x48, //
        0xDC, 0x08, 0x15, 0x0A, 0x05, 0x00, 0x60, 0x58, //
        0xDC, 0x08, 0x18, 0x0A, 0x05, 0x00, 0x60, 0x00, //
        0xDB, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0xA8, //
        0xDC, 0x08, 0x06, 0x0A, 0x05, 0x00, 0x70, 0x08, //
        0xDC, 0x08, 0x09, 0x0A, 0x05, 0x00, 0x70, 0x18, //
        0xDC, 0x08, 0x0C, 0x0A, 0x05, 0x00, 0x70, 0x28, //
        0xDC, 0x08, 0x0F, 0x0A, 0x05, 0x00, 0x70, 0x38, //
        0xDC, 0x08, 0x12, 0x0A, 0x05, 0x00, 0x70, 0x48, //
        0xDC, 0x08, 0x15, 0x0A, 0x05, 0x00, 0x70, 0x58, //
        0xDC, 0x08, 0x18, 0x0A, 0x05, 0x00, 0x70, 0x68, //
        0xDC, 0x08, 0x1B, 0x0A, 0x05, 0x00, 0x70, 0x00, //
        0xDF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
    ];
    static OUTPUT: &str = "\
{
    gsSPLight(D_05000200, 1),
    gsSPSetLights1(D_05001000),
    gsSPSetLights2(D_05002000),
    gsSPSetLights3(D_05003000),
    gsSPSetLights4(D_05004000),
    gsSPSetLights5(D_05005000),
    gsSPSetLights6(D_05006000),
    gsSPSetLights7(D_05007000),
    gsSPEndDisplayList(),
}
";

    let (out, tracker) = callback_common(&INPUT, Microcode::F3dex2);
    assert_eq!(OUTPUT, out);

    let expected_tracker = Tracker {
        light: HashMap::from_iter([(Address(0x05000200), ())]),
        lightsn: HashMap::from_iter([
            (Address(0x05001000), (LightsNum::NumLights1,)),
            (Address(0x05002000), (LightsNum::NumLights2,)),
            (Address(0x05003000), (LightsNum::NumLights3,)),
            (Address(0x05004000), (LightsNum::NumLights4,)),
            (Address(0x05005000), (LightsNum::NumLights5,)),
            (Address(0x05006000), (LightsNum::NumLights6,)),
            (Address(0x05007000), (LightsNum::NumLights7,)),
        ]),
        ..Tracker::default()
    };
    assert_eq!(expected_tracker, tracker);
}

#[test]
fn test_seg() {
    static INPUT: [u8; 0x88] = [
        0xDB, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0xDB, 0x06, 0x00, 0x04, 0x00, 0x00, 0x10, 0x00, //
        0xDB, 0x06, 0x00, 0x08, 0x00, 0x00, 0x20, 0x00, //
        0xDB, 0x06, 0x00, 0x0C, 0x00, 0x00, 0x30, 0x00, //
        0xDB, 0x06, 0x00, 0x10, 0x00, 0x00, 0x40, 0x00, //
        0xDB, 0x06, 0x00, 0x14, 0x00, 0x00, 0x50, 0x00, //
        0xDB, 0x06, 0x00, 0x18, 0x00, 0x00, 0x60, 0x00, //
        0xDB, 0x06, 0x00, 0x1C, 0x00, 0x00, 0x70, 0x00, //
        0xDB, 0x06, 0x00, 0x20, 0x00, 0x00, 0x80, 0x00, //
        0xDB, 0x06, 0x00, 0x24, 0x00, 0x00, 0x90, 0x00, //
        0xDB, 0x06, 0x00, 0x28, 0x00, 0x00, 0xA0, 0x00, //
        0xDB, 0x06, 0x00, 0x2C, 0x00, 0x00, 0xB0, 0x00, //
        0xDB, 0x06, 0x00, 0x30, 0x00, 0x00, 0xC0, 0x00, //
        0xDB, 0x06, 0x00, 0x34, 0x00, 0x00, 0xD0, 0x00, //
        0xDB, 0x06, 0x00, 0x38, 0x00, 0x00, 0xE0, 0x00, //
        0xDB, 0x06, 0x00, 0x3C, 0x00, 0x00, 0xF0, 0x00, //
        0xDF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
    ];
    static OUTPUT: &str = "\
{
    gsSPSegment(0x00, D_00000000),
    gsSPSegment(0x01, D_00001000),
    gsSPSegment(0x02, D_00002000),
    gsSPSegment(0x03, D_00003000),
    gsSPSegment(0x04, D_00004000),
    gsSPSegment(0x05, D_00005000),
    gsSPSegment(0x06, D_00006000),
    gsSPSegment(0x07, D_00007000),
    gsSPSegment(0x08, D_00008000),
    gsSPSegment(0x09, D_00009000),
    gsSPSegment(0x0A, D_0000A000),
    gsSPSegment(0x0B, D_0000B000),
    gsSPSegment(0x0C, D_0000C000),
    gsSPSegment(0x0D, D_0000D000),
    gsSPSegment(0x0E, D_0000E000),
    gsSPSegment(0x0F, D_0000F000),
    gsSPEndDisplayList(),
}
";

    let (out, tracker) = callback_common(&INPUT, Microcode::F3dex2);
    assert_eq!(OUTPUT, out);

    let expected_tracker = Tracker {
        seg: HashMap::from_iter([
            (Address(0x00000000), (0x00,)),
            (Address(0x00001000), (0x01,)),
            (Address(0x00002000), (0x02,)),
            (Address(0x00003000), (0x03,)),
            (Address(0x00004000), (0x04,)),
            (Address(0x00005000), (0x05,)),
            (Address(0x00006000), (0x06,)),
            (Address(0x00007000), (0x07,)),
            (Address(0x00008000), (0x08,)),
            (Address(0x00009000), (0x09,)),
            (Address(0x0000A000), (0x0A,)),
            (Address(0x0000B000), (0x0B,)),
            (Address(0x0000C000), (0x0C,)),
            (Address(0x0000D000), (0x0D,)),
            (Address(0x0000E000), (0x0E,)),
            (Address(0x0000F000), (0x0F,)),
        ]),
        ..Tracker::default()
    };
    assert_eq!(expected_tracker, tracker);
}

#[test]
fn test_ucode() {
    static INPUT: [u8; 0x28] = [
        0xE1, 0x00, 0x00, 0x00, 0x80, 0x00, 0x60, 0x00, //
        0xDD, 0x00, 0x07, 0xFF, 0x80, 0x00, 0x40, 0x00, //
        0xE1, 0x00, 0x00, 0x00, 0x80, 0x00, 0xA0, 0x00, //
        0xDD, 0x00, 0x03, 0xFF, 0x80, 0x00, 0x80, 0x00, //
        0xDF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
    ];
    static OUTPUT: &str = "\
{
    gsSPLoadUcode(D_80004000, D_80006000),
    gsSPLoadUcodeEx(D_80008000, D_8000A000, 0x0400),
    gsSPEndDisplayList(),
}
";

    let (out, tracker) = callback_common(&INPUT, Microcode::F3dex2);
    assert_eq!(OUTPUT, out);

    let expected_tracker = Tracker {
        uctext: HashMap::from_iter([
            (Address(0x80004000), (NonZeroU32::new(0x1000).unwrap(),)),
            (Address(0x80008000), (NonZeroU32::new(0x1000).unwrap(),)),
        ]),
        ucdata: HashMap::from_iter([
            (Address(0x80006000), (NonZeroU32::new(0x0800).unwrap(),)),
            (Address(0x8000A000), (NonZeroU32::new(0x0400).unwrap(),)),
        ]),
        ..Tracker::default()
    };
    assert_eq!(expected_tracker, tracker);
}

#[test]
fn test_dram() {
    static INPUT: [u8; 0x18] = [
        0xD6, 0x19, 0xE0, 0x13, 0x88, 0x88, 0x88, 0x88, //
        0xD6, 0x99, 0xE0, 0x13, 0x80, 0x88, 0x88, 0x88, //
        0xDF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
    ];
    static OUTPUT: &str = "\
{
    gsSPDmaRead(0x0678, D_88888888, 0x0014),
    gsSPDmaWrite(0x0678, D_80888888, 0x0014),
    gsSPEndDisplayList(),
}
";

    let (out, tracker) = callback_common(&INPUT, Microcode::F3dex2);
    assert_eq!(OUTPUT, out);

    let expected_tracker = Tracker {
        dram: HashMap::from_iter([
            (Address(0x88888888), (NonZeroU16::new(0x14).unwrap(),)),
            (Address(0x80888888), (NonZeroU16::new(0x14).unwrap(),)),
        ]),
        ..Tracker::default()
    };
    assert_eq!(expected_tracker, tracker);
}
