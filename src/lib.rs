/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(improper_ctypes)]

extern crate alloc;

pub(crate) mod disassembler;
pub(crate) mod microcode;

pub use disassembler::Disassembler;
pub use microcode::Microcode;
