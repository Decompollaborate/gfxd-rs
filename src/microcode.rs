/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use gfxd_sys::settings::{gfxd_f3d, gfxd_f3db, gfxd_f3dex, gfxd_f3dex2, gfxd_f3dexb, gfxd_ucode_t};

/// The target microcode to decode the `Gfx` packets as.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Microcode {
    F3d,
    F3db,
    F3dex,
    F3dexb,
    F3dex2,
}

impl Microcode {
    /// Get a `Microcode` variant from its name.
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "f3d" => Some(Self::F3d),
            "f3db" => Some(Self::F3db),
            "f3dex" => Some(Self::F3dex),
            "f3dexb" => Some(Self::F3dexb),
            "f3dex2" => Some(Self::F3dex2),
            _ => None,
        }
    }
}

impl Microcode {
    pub(crate) fn to_microcode_ptr(self) -> gfxd_ucode_t {
        match self {
            Microcode::F3d => unsafe { gfxd_f3d },
            Microcode::F3db => unsafe { gfxd_f3db },
            Microcode::F3dex => unsafe { gfxd_f3dex },
            Microcode::F3dexb => unsafe { gfxd_f3dexb },
            Microcode::F3dex2 => unsafe { gfxd_f3dex2 },
        }
    }
}
