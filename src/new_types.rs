/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::fmt;

/// An address pointed to by a `Gfx` macro.
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Address(pub u32);

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Address(0x{:08X})", self.0)
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:08X}", self.0)
    }
}

/// How many colors a TLUT has.
///
/// The most common ones are `Pal16` and `Pal256`.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TlutCount {
    /// 16 colors TLUT. Usually paired with CI4 textures.
    Pal16,
    /// 256 colors TLUT. Usually paired with CI8 textures.
    Pal256,
    /// Any other amount of colors. Very unusual.
    // This value is 14 bits wide, so a u16 is more than enough.
    Other(u16),
}

impl TlutCount {
    pub(crate) const fn new(value: i32) -> Self {
        match value {
            16 => Self::Pal16,
            256 => Self::Pal256,
            x => Self::Other(x as _),
        }
    }
}

/// The format of a texture.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TexFmt {
    /// Explicit Red/Green/Blue/Alpha textures.
    ///
    /// Only supported with [`Siz16b`] and [`Siz32b`].
    ///
    /// [`Siz16b`]: TexSiz::Siz16b
    /// [`Siz32b`]: TexSiz::Siz32b
    Rgba,
    /// Luminance (Y) and two chrominance (UV).
    ///
    /// Only supported with [`Siz16b`].
    ///
    /// [`Siz16b`]: TexSiz::Siz16b
    Yuv,
    /// Color indexed textures.
    ///
    /// Only supported with [`Siz8b`] and  [`Siz4b`].
    ///
    /// [`Siz8b`]: TexSiz::Siz8b
    /// [`Siz4b`]: TexSiz::Siz4b
    CI,
    /// Intensity with Alpha textures.
    ///
    /// Only supported with [`Siz16b`], [`Siz8b`] and  [`Siz4b`].
    ///
    /// [`Siz16b`]: TexSiz::Siz16b
    /// [`Siz8b`]: TexSiz::Siz8b
    /// [`Siz4b`]: TexSiz::Siz4b
    IA,
    /// Intensity textures.
    ///
    /// Only supported with [`Siz8b`] and  [`Siz4b`].
    ///
    /// [`Siz8b`]: TexSiz::Siz8b
    /// [`Siz4b`]: TexSiz::Siz4b
    I,
    /// Other raw values.
    // This value is 3 bits wide, so a u8 is more than enough.
    Other(u8),
}

impl TexFmt {
    pub(crate) const fn new(value: i32) -> Self {
        match value {
            0 => Self::Rgba,
            1 => Self::Yuv,
            2 => Self::CI,
            3 => Self::IA,
            4 => Self::I,
            x => Self::Other(x as _),
        }
    }
}

/// The bit size of a texture.
// This value is 2 bits wide, so the enum should exhaust every posibility.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TexSiz {
    /// 4 bit per pixel.
    Siz4b,
    /// 8 bit per pixel.
    Siz8b,
    /// 16 bit per pixel.
    Siz16b,
    /// 32 bit per pixel.
    Siz32b,
}

impl TexSiz {
    pub(crate) fn new(value: i32) -> Self {
        match value {
            0 => Self::Siz4b,
            1 => Self::Siz8b,
            2 => Self::Siz16b,
            3 => Self::Siz32b,
            x => unreachable!(
                "Oh, this shouldn't have had happen. Could you make a bug report? Value: {}",
                x
            ),
        }
    }
}

/// How many macro packets the current macro actually refers to.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum LookatCount {
    /// This macro does not correspond to a complete `gsSPLookAt`, but instead
    /// is a singular use of either `gsSPLookAtX` or `gsSPLookAtY`.
    N1 = 1,
    /// This macro is a complete `gsSPLookAt` macro.
    N2 = 2,
}

impl LookatCount {
    pub(crate) fn new(value: i32) -> Self {
        match value {
            1 => Self::N1,
            2 => Self::N2,
            // SAFETY: This is a value made by gfxd, not decoded from a macro,
            // and we already cover every possible value, so if we reach this
            // part it means gfxd itself update itself to add new values.
            x => unreachable!(
                "Oh, this shouldn't have had happen. Could you make a bug report? Value: {}",
                x
            ),
        }
    }
}

/// `NUMLIGHTS_0` is absent from this enum because it just expands to
/// `NUMLIGHTS_1`, making them indistinguishable from each other.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum LightsNum {
    /// 1 diffuse light.
    #[doc(alias = "NUMLIGHTS_1")]
    NumLights1 = 1,
    /// 2 diffuse lights.
    #[doc(alias = "NUMLIGHTS_2")]
    NumLights2 = 2,
    /// 3 diffuse lights.
    #[doc(alias = "NUMLIGHTS_3")]
    NumLights3 = 3,
    /// 4 diffuse lights.
    #[doc(alias = "NUMLIGHTS_4")]
    NumLights4 = 4,
    /// 5 diffuse lights.
    #[doc(alias = "NUMLIGHTS_5")]
    NumLights5 = 5,
    /// 6 diffuse lights.
    #[doc(alias = "NUMLIGHTS_6")]
    NumLights6 = 6,
    /// 7 diffuse lights.
    #[doc(alias = "NUMLIGHTS_7")]
    NumLights7 = 7,
}

impl LightsNum {
    pub(crate) fn new(value: i32) -> Self {
        match value {
            1 => Self::NumLights1,
            2 => Self::NumLights2,
            3 => Self::NumLights3,
            4 => Self::NumLights4,
            5 => Self::NumLights5,
            6 => Self::NumLights6,
            7 => Self::NumLights7,
            x => unreachable!(
                "Oh, this shouldn't have had happen. Could you make a bug report? Value: {}",
                x
            ),
        }
    }
}
