/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::{convert::TryFrom, fmt};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub enum ArgType {
    /// generic word
    Word = 0,
    /// command opcode (G_*)
    Opcode = 1,
    /// integer coordinate
    Coordi = 2,
    /// fractional (q10.2) coordinate
    Coordq = 3,
    /// palette index
    Pal = 4,
    /// tlut pointer
    Tlut = 5,
    /// texture image pointer
    Timg = 6,
    /// tmem address
    Tmem = 7,
    /// tile index
    Tile = 8,
    /// texture format
    Fmt = 9,
    /// texture pixel size
    Siz = 10,
    /// integer dimension (width / height)
    Dim = 11,
    /// clamp and mirror flags
    Cm = 12,
    /// tile mask
    Tm = 13,
    /// tile shift
    Ts = 14,
    /// texture dxt
    Dxt = 15,
    /// generic tag
    Tag = 16,
    /// pipeline mode
    Pm = 17,
    /// color component
    Colorpart = 18,
    /// color
    Color = 19,
    /// lod fraction (q0.8)
    Lodfrac = 20,
    /// color image pointer
    Cimg = 21,
    /// depth image pointer
    Zimg = 22,
    /// alpha compare mode
    Ac = 23,
    /// alpha dither mode
    Ad = 24,
    /// color dither mode
    Cd = 25,
    /// color combiner preset index
    Ccpre = 26,
    /// color mux operand (a)
    Ccmuxa = 27,
    /// color mux operand (b)
    Ccmuxb = 28,
    /// color mux operand (c)
    Ccmuxc = 29,
    /// color mux operand (d)
    Ccmuxd = 30,
    /// alpha mux operand (a, b, or d)
    Acmuxabd = 31,
    /// alpha mux operand (c)
    Acmuxc = 32,
    /// color convert operand
    Cv = 33,
    /// texture convert mode
    Tc = 34,
    /// cycle type
    Cyc = 35,
    /// depth source mode
    Zs = 36,
    /// combine key mode
    Ck = 37,
    /// combine key scale
    Keyscale = 38,
    /// combine key width
    Keywidth = 39,
    /// integer depth
    Zi = 40,
    /// cycle 1 render mode
    Rm1 = 41,
    /// cycle 2 render mode
    Rm2 = 42,
    /// scissor mode
    Sc = 43,
    /// texture detail mode
    Td = 44,
    /// texture filter mode
    Tf = 45,
    /// texture LOD mode
    Tl = 46,
    /// textuure LUT mode
    Tt = 47,
    /// texture perspective mode
    Tp = 48,
    /// texture line size
    Line = 49,
    /// vertex index
    Vtx = 50,
    /// vertex flag
    Vtxflag = 51,
    /// display list pointer
    Dl = 52,
    /// raw depth value (q16.16)
    Zraw = 53,
    /// display list flag
    Dlflag = 54,
    /// clip ratio
    Cr = 55,
    /// element count
    Num = 56,
    /// fog factor
    Fogz = 57,
    /// fog position (0 - 1000)
    Fogp = 58,
    /// matrix pointer
    Mtxptr = 59,
    /// geometry mode
    Gm = 60,
    /// matrix moveword offset
    Mwo_matrix = 61,
    /// line width (1.5 + q7.1)
    Linewd = 62,
    /// microcode text pointer
    Uctext = 63,
    /// microcode data pointer
    Ucdata = 64,
    /// data size
    Size = 65,
    /// lookat pointer
    Lookatptr = 66,
    /// matrix param
    Mtxparam = 67,
    /// matrix param (stack select only)
    Mtxstack = 68,
    /// vertex moveword offset
    Mwo_point = 69,
    /// w-component scale (perspnorm)
    Wscale = 70,
    /// segment number
    Seg = 71,
    /// segment pointer
    Segptr = 72,
    /// dereferenced LightsM (0-7 or n) pointer
    Lightsn = 73,
    /// light count (NUMLIGHTS_*)
    Numlights = 74,
    /// light number (LIGHT_*)
    Lightnum = 75,
    /// diffuse or ambient light pointer
    Lightptr = 76,
    /// texture coordinate scale
    Tcscale = 77,
    /// on-off value
    Switch = 78,
    /// vertex coordinate (q10.5)
    St = 79,
    /// vertex coordinate delta (q5.10)
    Stdelta = 80,
    /// vertex pointer
    Vtxptr = 81,
    /// viewport pointer
    Vpptr = 82,
    /// generic dram address
    Dram = 83,
    /// othermode lo shift
    Sftlo = 84,
    /// othermode lo value
    Othermodelo = 85,
    /// othermode hi shift
    Sfthi = 86,
    /// othermode hi value
    Othermodehi = 87,
    /// moveword index
    Mw = 88,
    /// moveword offset
    Mwo = 89,
    /// clip ratio moveword offset
    Mwo_clip = 90,
    /// light color moveword offset
    Mwo_lightcol = 91,
    /// movemem index
    Mv = 92,
    /// movemem offset
    Mvo = 93,
    /// dmem address
    Dmem = 94,
    /// dma io flag
    Dmaflag = 95,
}

impl ArgType {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Word => "Word",
            Self::Opcode => "Opcode",
            Self::Coordi => "Coordi",
            Self::Coordq => "Coordq",
            Self::Pal => "Pal",
            Self::Tlut => "Tlut",
            Self::Timg => "Timg",
            Self::Tmem => "Tmem",
            Self::Tile => "Tile",
            Self::Fmt => "Fmt",
            Self::Siz => "Siz",
            Self::Dim => "Dim",
            Self::Cm => "Cm",
            Self::Tm => "Tm",
            Self::Ts => "Ts",
            Self::Dxt => "Dxt",
            Self::Tag => "Tag",
            Self::Pm => "Pm",
            Self::Colorpart => "Colorpart",
            Self::Color => "Color",
            Self::Lodfrac => "Lodfrac",
            Self::Cimg => "Cimg",
            Self::Zimg => "Zimg",
            Self::Ac => "Ac",
            Self::Ad => "Ad",
            Self::Cd => "Cd",
            Self::Ccpre => "Ccpre",
            Self::Ccmuxa => "Ccmuxa",
            Self::Ccmuxb => "Ccmuxb",
            Self::Ccmuxc => "Ccmuxc",
            Self::Ccmuxd => "Ccmuxd",
            Self::Acmuxabd => "Acmuxabd",
            Self::Acmuxc => "Acmuxc",
            Self::Cv => "Cv",
            Self::Tc => "Tc",
            Self::Cyc => "Cyc",
            Self::Zs => "Zs",
            Self::Ck => "Ck",
            Self::Keyscale => "Keyscale",
            Self::Keywidth => "Keywidth",
            Self::Zi => "Zi",
            Self::Rm1 => "Rm1",
            Self::Rm2 => "Rm2",
            Self::Sc => "Sc",
            Self::Td => "Td",
            Self::Tf => "Tf",
            Self::Tl => "Tl",
            Self::Tt => "Tt",
            Self::Tp => "Tp",
            Self::Line => "Line",
            Self::Vtx => "Vtx",
            Self::Vtxflag => "Vtxflag",
            Self::Dl => "Dl",
            Self::Zraw => "Zraw",
            Self::Dlflag => "Dlflag",
            Self::Cr => "Cr",
            Self::Num => "Num",
            Self::Fogz => "Fogz",
            Self::Fogp => "Fogp",
            Self::Mtxptr => "Mtxptr",
            Self::Gm => "Gm",
            Self::Mwo_matrix => "Mwo_matrix",
            Self::Linewd => "Linewd",
            Self::Uctext => "Uctext",
            Self::Ucdata => "Ucdata",
            Self::Size => "Size",
            Self::Lookatptr => "Lookatptr",
            Self::Mtxparam => "Mtxparam",
            Self::Mtxstack => "Mtxstack",
            Self::Mwo_point => "Mwo_point",
            Self::Wscale => "Wscale",
            Self::Seg => "Seg",
            Self::Segptr => "Segptr",
            Self::Lightsn => "Lightsn",
            Self::Numlights => "Numlights",
            Self::Lightnum => "Lightnum",
            Self::Lightptr => "Lightptr",
            Self::Tcscale => "Tcscale",
            Self::Switch => "Switch",
            Self::St => "St",
            Self::Stdelta => "Stdelta",
            Self::Vtxptr => "Vtxptr",
            Self::Vpptr => "Vpptr",
            Self::Dram => "Dram",
            Self::Sftlo => "Sftlo",
            Self::Othermodelo => "Othermodelo",
            Self::Sfthi => "Sfthi",
            Self::Othermodehi => "Othermodehi",
            Self::Mw => "Mw",
            Self::Mwo => "Mwo",
            Self::Mwo_clip => "Mwo_clip",
            Self::Mwo_lightcol => "Mwo_lightcol",
            Self::Mv => "Mv",
            Self::Mvo => "Mvo",
            Self::Dmem => "Dmem",
            Self::Dmaflag => "Dmaflag",
        }
    }

    pub const fn from_u32(value: u32) -> Option<Self> {
        match value {
            0 => Some(Self::Word),
            1 => Some(Self::Opcode),
            2 => Some(Self::Coordi),
            3 => Some(Self::Coordq),
            4 => Some(Self::Pal),
            5 => Some(Self::Tlut),
            6 => Some(Self::Timg),
            7 => Some(Self::Tmem),
            8 => Some(Self::Tile),
            9 => Some(Self::Fmt),
            10 => Some(Self::Siz),
            11 => Some(Self::Dim),
            12 => Some(Self::Cm),
            13 => Some(Self::Tm),
            14 => Some(Self::Ts),
            15 => Some(Self::Dxt),
            16 => Some(Self::Tag),
            17 => Some(Self::Pm),
            18 => Some(Self::Colorpart),
            19 => Some(Self::Color),
            20 => Some(Self::Lodfrac),
            21 => Some(Self::Cimg),
            22 => Some(Self::Zimg),
            23 => Some(Self::Ac),
            24 => Some(Self::Ad),
            25 => Some(Self::Cd),
            26 => Some(Self::Ccpre),
            27 => Some(Self::Ccmuxa),
            28 => Some(Self::Ccmuxb),
            29 => Some(Self::Ccmuxc),
            30 => Some(Self::Ccmuxd),
            31 => Some(Self::Acmuxabd),
            32 => Some(Self::Acmuxc),
            33 => Some(Self::Cv),
            34 => Some(Self::Tc),
            35 => Some(Self::Cyc),
            36 => Some(Self::Zs),
            37 => Some(Self::Ck),
            38 => Some(Self::Keyscale),
            39 => Some(Self::Keywidth),
            40 => Some(Self::Zi),
            41 => Some(Self::Rm1),
            42 => Some(Self::Rm2),
            43 => Some(Self::Sc),
            44 => Some(Self::Td),
            45 => Some(Self::Tf),
            46 => Some(Self::Tl),
            47 => Some(Self::Tt),
            48 => Some(Self::Tp),
            49 => Some(Self::Line),
            50 => Some(Self::Vtx),
            51 => Some(Self::Vtxflag),
            52 => Some(Self::Dl),
            53 => Some(Self::Zraw),
            54 => Some(Self::Dlflag),
            55 => Some(Self::Cr),
            56 => Some(Self::Num),
            57 => Some(Self::Fogz),
            58 => Some(Self::Fogp),
            59 => Some(Self::Mtxptr),
            60 => Some(Self::Gm),
            61 => Some(Self::Mwo_matrix),
            62 => Some(Self::Linewd),
            63 => Some(Self::Uctext),
            64 => Some(Self::Ucdata),
            65 => Some(Self::Size),
            66 => Some(Self::Lookatptr),
            67 => Some(Self::Mtxparam),
            68 => Some(Self::Mtxstack),
            69 => Some(Self::Mwo_point),
            70 => Some(Self::Wscale),
            71 => Some(Self::Seg),
            72 => Some(Self::Segptr),
            73 => Some(Self::Lightsn),
            74 => Some(Self::Numlights),
            75 => Some(Self::Lightnum),
            76 => Some(Self::Lightptr),
            77 => Some(Self::Tcscale),
            78 => Some(Self::Switch),
            79 => Some(Self::St),
            80 => Some(Self::Stdelta),
            81 => Some(Self::Vtxptr),
            82 => Some(Self::Vpptr),
            83 => Some(Self::Dram),
            84 => Some(Self::Sftlo),
            85 => Some(Self::Othermodelo),
            86 => Some(Self::Sfthi),
            87 => Some(Self::Othermodehi),
            88 => Some(Self::Mw),
            89 => Some(Self::Mwo),
            90 => Some(Self::Mwo_clip),
            91 => Some(Self::Mwo_lightcol),
            92 => Some(Self::Mv),
            93 => Some(Self::Mvo),
            94 => Some(Self::Dmem),
            95 => Some(Self::Dmaflag),
            _ => None,
        }
    }

    pub const fn to_u32(&self) -> u32 {
        match self {
            Self::Word => 0,
            Self::Opcode => 1,
            Self::Coordi => 2,
            Self::Coordq => 3,
            Self::Pal => 4,
            Self::Tlut => 5,
            Self::Timg => 6,
            Self::Tmem => 7,
            Self::Tile => 8,
            Self::Fmt => 9,
            Self::Siz => 10,
            Self::Dim => 11,
            Self::Cm => 12,
            Self::Tm => 13,
            Self::Ts => 14,
            Self::Dxt => 15,
            Self::Tag => 16,
            Self::Pm => 17,
            Self::Colorpart => 18,
            Self::Color => 19,
            Self::Lodfrac => 20,
            Self::Cimg => 21,
            Self::Zimg => 22,
            Self::Ac => 23,
            Self::Ad => 24,
            Self::Cd => 25,
            Self::Ccpre => 26,
            Self::Ccmuxa => 27,
            Self::Ccmuxb => 28,
            Self::Ccmuxc => 29,
            Self::Ccmuxd => 30,
            Self::Acmuxabd => 31,
            Self::Acmuxc => 32,
            Self::Cv => 33,
            Self::Tc => 34,
            Self::Cyc => 35,
            Self::Zs => 36,
            Self::Ck => 37,
            Self::Keyscale => 38,
            Self::Keywidth => 39,
            Self::Zi => 40,
            Self::Rm1 => 41,
            Self::Rm2 => 42,
            Self::Sc => 43,
            Self::Td => 44,
            Self::Tf => 45,
            Self::Tl => 46,
            Self::Tt => 47,
            Self::Tp => 48,
            Self::Line => 49,
            Self::Vtx => 50,
            Self::Vtxflag => 51,
            Self::Dl => 52,
            Self::Zraw => 53,
            Self::Dlflag => 54,
            Self::Cr => 55,
            Self::Num => 56,
            Self::Fogz => 57,
            Self::Fogp => 58,
            Self::Mtxptr => 59,
            Self::Gm => 60,
            Self::Mwo_matrix => 61,
            Self::Linewd => 62,
            Self::Uctext => 63,
            Self::Ucdata => 64,
            Self::Size => 65,
            Self::Lookatptr => 66,
            Self::Mtxparam => 67,
            Self::Mtxstack => 68,
            Self::Mwo_point => 69,
            Self::Wscale => 70,
            Self::Seg => 71,
            Self::Segptr => 72,
            Self::Lightsn => 73,
            Self::Numlights => 74,
            Self::Lightnum => 75,
            Self::Lightptr => 76,
            Self::Tcscale => 77,
            Self::Switch => 78,
            Self::St => 79,
            Self::Stdelta => 80,
            Self::Vtxptr => 81,
            Self::Vpptr => 82,
            Self::Dram => 83,
            Self::Sftlo => 84,
            Self::Othermodelo => 85,
            Self::Sfthi => 86,
            Self::Othermodehi => 87,
            Self::Mw => 88,
            Self::Mwo => 89,
            Self::Mwo_clip => 90,
            Self::Mwo_lightcol => 91,
            Self::Mv => 92,
            Self::Mvo => 93,
            Self::Dmem => 94,
            Self::Dmaflag => 95,
        }
    }
}

impl fmt::Display for ArgType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<gfxd_sys::arg_type::ArgType> for ArgType {
    fn from(value: gfxd_sys::arg_type::ArgType) -> Self {
        match value {
            gfxd_sys::arg_type::gfxd_Word => Self::Word,
            gfxd_sys::arg_type::gfxd_Opcode => Self::Opcode,
            gfxd_sys::arg_type::gfxd_Coordi => Self::Coordi,
            gfxd_sys::arg_type::gfxd_Coordq => Self::Coordq,
            gfxd_sys::arg_type::gfxd_Pal => Self::Pal,
            gfxd_sys::arg_type::gfxd_Tlut => Self::Tlut,
            gfxd_sys::arg_type::gfxd_Timg => Self::Timg,
            gfxd_sys::arg_type::gfxd_Tmem => Self::Tmem,
            gfxd_sys::arg_type::gfxd_Tile => Self::Tile,
            gfxd_sys::arg_type::gfxd_Fmt => Self::Fmt,
            gfxd_sys::arg_type::gfxd_Siz => Self::Siz,
            gfxd_sys::arg_type::gfxd_Dim => Self::Dim,
            gfxd_sys::arg_type::gfxd_Cm => Self::Cm,
            gfxd_sys::arg_type::gfxd_Tm => Self::Tm,
            gfxd_sys::arg_type::gfxd_Ts => Self::Ts,
            gfxd_sys::arg_type::gfxd_Dxt => Self::Dxt,
            gfxd_sys::arg_type::gfxd_Tag => Self::Tag,
            gfxd_sys::arg_type::gfxd_Pm => Self::Pm,
            gfxd_sys::arg_type::gfxd_Colorpart => Self::Colorpart,
            gfxd_sys::arg_type::gfxd_Color => Self::Color,
            gfxd_sys::arg_type::gfxd_Lodfrac => Self::Lodfrac,
            gfxd_sys::arg_type::gfxd_Cimg => Self::Cimg,
            gfxd_sys::arg_type::gfxd_Zimg => Self::Zimg,
            gfxd_sys::arg_type::gfxd_Ac => Self::Ac,
            gfxd_sys::arg_type::gfxd_Ad => Self::Ad,
            gfxd_sys::arg_type::gfxd_Cd => Self::Cd,
            gfxd_sys::arg_type::gfxd_Ccpre => Self::Ccpre,
            gfxd_sys::arg_type::gfxd_Ccmuxa => Self::Ccmuxa,
            gfxd_sys::arg_type::gfxd_Ccmuxb => Self::Ccmuxb,
            gfxd_sys::arg_type::gfxd_Ccmuxc => Self::Ccmuxc,
            gfxd_sys::arg_type::gfxd_Ccmuxd => Self::Ccmuxd,
            gfxd_sys::arg_type::gfxd_Acmuxabd => Self::Acmuxabd,
            gfxd_sys::arg_type::gfxd_Acmuxc => Self::Acmuxc,
            gfxd_sys::arg_type::gfxd_Cv => Self::Cv,
            gfxd_sys::arg_type::gfxd_Tc => Self::Tc,
            gfxd_sys::arg_type::gfxd_Cyc => Self::Cyc,
            gfxd_sys::arg_type::gfxd_Zs => Self::Zs,
            gfxd_sys::arg_type::gfxd_Ck => Self::Ck,
            gfxd_sys::arg_type::gfxd_Keyscale => Self::Keyscale,
            gfxd_sys::arg_type::gfxd_Keywidth => Self::Keywidth,
            gfxd_sys::arg_type::gfxd_Zi => Self::Zi,
            gfxd_sys::arg_type::gfxd_Rm1 => Self::Rm1,
            gfxd_sys::arg_type::gfxd_Rm2 => Self::Rm2,
            gfxd_sys::arg_type::gfxd_Sc => Self::Sc,
            gfxd_sys::arg_type::gfxd_Td => Self::Td,
            gfxd_sys::arg_type::gfxd_Tf => Self::Tf,
            gfxd_sys::arg_type::gfxd_Tl => Self::Tl,
            gfxd_sys::arg_type::gfxd_Tt => Self::Tt,
            gfxd_sys::arg_type::gfxd_Tp => Self::Tp,
            gfxd_sys::arg_type::gfxd_Line => Self::Line,
            gfxd_sys::arg_type::gfxd_Vtx => Self::Vtx,
            gfxd_sys::arg_type::gfxd_Vtxflag => Self::Vtxflag,
            gfxd_sys::arg_type::gfxd_Dl => Self::Dl,
            gfxd_sys::arg_type::gfxd_Zraw => Self::Zraw,
            gfxd_sys::arg_type::gfxd_Dlflag => Self::Dlflag,
            gfxd_sys::arg_type::gfxd_Cr => Self::Cr,
            gfxd_sys::arg_type::gfxd_Num => Self::Num,
            gfxd_sys::arg_type::gfxd_Fogz => Self::Fogz,
            gfxd_sys::arg_type::gfxd_Fogp => Self::Fogp,
            gfxd_sys::arg_type::gfxd_Mtxptr => Self::Mtxptr,
            gfxd_sys::arg_type::gfxd_Gm => Self::Gm,
            gfxd_sys::arg_type::gfxd_Mwo_matrix => Self::Mwo_matrix,
            gfxd_sys::arg_type::gfxd_Linewd => Self::Linewd,
            gfxd_sys::arg_type::gfxd_Uctext => Self::Uctext,
            gfxd_sys::arg_type::gfxd_Ucdata => Self::Ucdata,
            gfxd_sys::arg_type::gfxd_Size => Self::Size,
            gfxd_sys::arg_type::gfxd_Lookatptr => Self::Lookatptr,
            gfxd_sys::arg_type::gfxd_Mtxparam => Self::Mtxparam,
            gfxd_sys::arg_type::gfxd_Mtxstack => Self::Mtxstack,
            gfxd_sys::arg_type::gfxd_Mwo_point => Self::Mwo_point,
            gfxd_sys::arg_type::gfxd_Wscale => Self::Wscale,
            gfxd_sys::arg_type::gfxd_Seg => Self::Seg,
            gfxd_sys::arg_type::gfxd_Segptr => Self::Segptr,
            gfxd_sys::arg_type::gfxd_Lightsn => Self::Lightsn,
            gfxd_sys::arg_type::gfxd_Numlights => Self::Numlights,
            gfxd_sys::arg_type::gfxd_Lightnum => Self::Lightnum,
            gfxd_sys::arg_type::gfxd_Lightptr => Self::Lightptr,
            gfxd_sys::arg_type::gfxd_Tcscale => Self::Tcscale,
            gfxd_sys::arg_type::gfxd_Switch => Self::Switch,
            gfxd_sys::arg_type::gfxd_St => Self::St,
            gfxd_sys::arg_type::gfxd_Stdelta => Self::Stdelta,
            gfxd_sys::arg_type::gfxd_Vtxptr => Self::Vtxptr,
            gfxd_sys::arg_type::gfxd_Vpptr => Self::Vpptr,
            gfxd_sys::arg_type::gfxd_Dram => Self::Dram,
            gfxd_sys::arg_type::gfxd_Sftlo => Self::Sftlo,
            gfxd_sys::arg_type::gfxd_Othermodelo => Self::Othermodelo,
            gfxd_sys::arg_type::gfxd_Sfthi => Self::Sfthi,
            gfxd_sys::arg_type::gfxd_Othermodehi => Self::Othermodehi,
            gfxd_sys::arg_type::gfxd_Mw => Self::Mw,
            gfxd_sys::arg_type::gfxd_Mwo => Self::Mwo,
            gfxd_sys::arg_type::gfxd_Mwo_clip => Self::Mwo_clip,
            gfxd_sys::arg_type::gfxd_Mwo_lightcol => Self::Mwo_lightcol,
            gfxd_sys::arg_type::gfxd_Mv => Self::Mv,
            gfxd_sys::arg_type::gfxd_Mvo => Self::Mvo,
            gfxd_sys::arg_type::gfxd_Dmem => Self::Dmem,
            gfxd_sys::arg_type::gfxd_Dmaflag => Self::Dmaflag,
        }
    }
}

impl From<ArgType> for gfxd_sys::arg_type::ArgType {
    fn from(value: ArgType) -> gfxd_sys::arg_type::ArgType {
        match value {
            ArgType::Word => gfxd_sys::arg_type::gfxd_Word,
            ArgType::Opcode => gfxd_sys::arg_type::gfxd_Opcode,
            ArgType::Coordi => gfxd_sys::arg_type::gfxd_Coordi,
            ArgType::Coordq => gfxd_sys::arg_type::gfxd_Coordq,
            ArgType::Pal => gfxd_sys::arg_type::gfxd_Pal,
            ArgType::Tlut => gfxd_sys::arg_type::gfxd_Tlut,
            ArgType::Timg => gfxd_sys::arg_type::gfxd_Timg,
            ArgType::Tmem => gfxd_sys::arg_type::gfxd_Tmem,
            ArgType::Tile => gfxd_sys::arg_type::gfxd_Tile,
            ArgType::Fmt => gfxd_sys::arg_type::gfxd_Fmt,
            ArgType::Siz => gfxd_sys::arg_type::gfxd_Siz,
            ArgType::Dim => gfxd_sys::arg_type::gfxd_Dim,
            ArgType::Cm => gfxd_sys::arg_type::gfxd_Cm,
            ArgType::Tm => gfxd_sys::arg_type::gfxd_Tm,
            ArgType::Ts => gfxd_sys::arg_type::gfxd_Ts,
            ArgType::Dxt => gfxd_sys::arg_type::gfxd_Dxt,
            ArgType::Tag => gfxd_sys::arg_type::gfxd_Tag,
            ArgType::Pm => gfxd_sys::arg_type::gfxd_Pm,
            ArgType::Colorpart => gfxd_sys::arg_type::gfxd_Colorpart,
            ArgType::Color => gfxd_sys::arg_type::gfxd_Color,
            ArgType::Lodfrac => gfxd_sys::arg_type::gfxd_Lodfrac,
            ArgType::Cimg => gfxd_sys::arg_type::gfxd_Cimg,
            ArgType::Zimg => gfxd_sys::arg_type::gfxd_Zimg,
            ArgType::Ac => gfxd_sys::arg_type::gfxd_Ac,
            ArgType::Ad => gfxd_sys::arg_type::gfxd_Ad,
            ArgType::Cd => gfxd_sys::arg_type::gfxd_Cd,
            ArgType::Ccpre => gfxd_sys::arg_type::gfxd_Ccpre,
            ArgType::Ccmuxa => gfxd_sys::arg_type::gfxd_Ccmuxa,
            ArgType::Ccmuxb => gfxd_sys::arg_type::gfxd_Ccmuxb,
            ArgType::Ccmuxc => gfxd_sys::arg_type::gfxd_Ccmuxc,
            ArgType::Ccmuxd => gfxd_sys::arg_type::gfxd_Ccmuxd,
            ArgType::Acmuxabd => gfxd_sys::arg_type::gfxd_Acmuxabd,
            ArgType::Acmuxc => gfxd_sys::arg_type::gfxd_Acmuxc,
            ArgType::Cv => gfxd_sys::arg_type::gfxd_Cv,
            ArgType::Tc => gfxd_sys::arg_type::gfxd_Tc,
            ArgType::Cyc => gfxd_sys::arg_type::gfxd_Cyc,
            ArgType::Zs => gfxd_sys::arg_type::gfxd_Zs,
            ArgType::Ck => gfxd_sys::arg_type::gfxd_Ck,
            ArgType::Keyscale => gfxd_sys::arg_type::gfxd_Keyscale,
            ArgType::Keywidth => gfxd_sys::arg_type::gfxd_Keywidth,
            ArgType::Zi => gfxd_sys::arg_type::gfxd_Zi,
            ArgType::Rm1 => gfxd_sys::arg_type::gfxd_Rm1,
            ArgType::Rm2 => gfxd_sys::arg_type::gfxd_Rm2,
            ArgType::Sc => gfxd_sys::arg_type::gfxd_Sc,
            ArgType::Td => gfxd_sys::arg_type::gfxd_Td,
            ArgType::Tf => gfxd_sys::arg_type::gfxd_Tf,
            ArgType::Tl => gfxd_sys::arg_type::gfxd_Tl,
            ArgType::Tt => gfxd_sys::arg_type::gfxd_Tt,
            ArgType::Tp => gfxd_sys::arg_type::gfxd_Tp,
            ArgType::Line => gfxd_sys::arg_type::gfxd_Line,
            ArgType::Vtx => gfxd_sys::arg_type::gfxd_Vtx,
            ArgType::Vtxflag => gfxd_sys::arg_type::gfxd_Vtxflag,
            ArgType::Dl => gfxd_sys::arg_type::gfxd_Dl,
            ArgType::Zraw => gfxd_sys::arg_type::gfxd_Zraw,
            ArgType::Dlflag => gfxd_sys::arg_type::gfxd_Dlflag,
            ArgType::Cr => gfxd_sys::arg_type::gfxd_Cr,
            ArgType::Num => gfxd_sys::arg_type::gfxd_Num,
            ArgType::Fogz => gfxd_sys::arg_type::gfxd_Fogz,
            ArgType::Fogp => gfxd_sys::arg_type::gfxd_Fogp,
            ArgType::Mtxptr => gfxd_sys::arg_type::gfxd_Mtxptr,
            ArgType::Gm => gfxd_sys::arg_type::gfxd_Gm,
            ArgType::Mwo_matrix => gfxd_sys::arg_type::gfxd_Mwo_matrix,
            ArgType::Linewd => gfxd_sys::arg_type::gfxd_Linewd,
            ArgType::Uctext => gfxd_sys::arg_type::gfxd_Uctext,
            ArgType::Ucdata => gfxd_sys::arg_type::gfxd_Ucdata,
            ArgType::Size => gfxd_sys::arg_type::gfxd_Size,
            ArgType::Lookatptr => gfxd_sys::arg_type::gfxd_Lookatptr,
            ArgType::Mtxparam => gfxd_sys::arg_type::gfxd_Mtxparam,
            ArgType::Mtxstack => gfxd_sys::arg_type::gfxd_Mtxstack,
            ArgType::Mwo_point => gfxd_sys::arg_type::gfxd_Mwo_point,
            ArgType::Wscale => gfxd_sys::arg_type::gfxd_Wscale,
            ArgType::Seg => gfxd_sys::arg_type::gfxd_Seg,
            ArgType::Segptr => gfxd_sys::arg_type::gfxd_Segptr,
            ArgType::Lightsn => gfxd_sys::arg_type::gfxd_Lightsn,
            ArgType::Numlights => gfxd_sys::arg_type::gfxd_Numlights,
            ArgType::Lightnum => gfxd_sys::arg_type::gfxd_Lightnum,
            ArgType::Lightptr => gfxd_sys::arg_type::gfxd_Lightptr,
            ArgType::Tcscale => gfxd_sys::arg_type::gfxd_Tcscale,
            ArgType::Switch => gfxd_sys::arg_type::gfxd_Switch,
            ArgType::St => gfxd_sys::arg_type::gfxd_St,
            ArgType::Stdelta => gfxd_sys::arg_type::gfxd_Stdelta,
            ArgType::Vtxptr => gfxd_sys::arg_type::gfxd_Vtxptr,
            ArgType::Vpptr => gfxd_sys::arg_type::gfxd_Vpptr,
            ArgType::Dram => gfxd_sys::arg_type::gfxd_Dram,
            ArgType::Sftlo => gfxd_sys::arg_type::gfxd_Sftlo,
            ArgType::Othermodelo => gfxd_sys::arg_type::gfxd_Othermodelo,
            ArgType::Sfthi => gfxd_sys::arg_type::gfxd_Sfthi,
            ArgType::Othermodehi => gfxd_sys::arg_type::gfxd_Othermodehi,
            ArgType::Mw => gfxd_sys::arg_type::gfxd_Mw,
            ArgType::Mwo => gfxd_sys::arg_type::gfxd_Mwo,
            ArgType::Mwo_clip => gfxd_sys::arg_type::gfxd_Mwo_clip,
            ArgType::Mwo_lightcol => gfxd_sys::arg_type::gfxd_Mwo_lightcol,
            ArgType::Mv => gfxd_sys::arg_type::gfxd_Mv,
            ArgType::Mvo => gfxd_sys::arg_type::gfxd_Mvo,
            ArgType::Dmem => gfxd_sys::arg_type::gfxd_Dmem,
            ArgType::Dmaflag => gfxd_sys::arg_type::gfxd_Dmaflag,
        }
    }
}

pub struct IntoArgTypeError;

impl TryFrom<u32> for ArgType {
    type Error = IntoArgTypeError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::from_u32(value).ok_or(IntoArgTypeError)
    }
}

impl From<ArgType> for u32 {
    fn from(value: ArgType) -> Self {
        value.to_u32()
    }
}
