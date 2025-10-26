/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(improper_ctypes)]

extern crate alloc;

pub(crate) mod customizer;
pub(crate) mod disassembler;
pub(crate) mod lib_data;
pub(crate) mod microcode;
pub(crate) mod printer;

pub use customizer::{Customizer, DoDefaultOutput};
pub use disassembler::Disassembler;
pub use microcode::Microcode;
pub use printer::Printer;
