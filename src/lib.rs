/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(improper_ctypes)]

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

pub use arg_type::ArgType;
pub use customizer::{Customizer, DoDefaultOutput, MacroFnRet};
pub use disassembler::Disassembler;
pub use macro_id::MacroId;
pub use macro_info::{ArgValue, MacroInfo};
pub use microcode::Microcode;
pub use new_types::{Address, LookatCount, TexFmt, TexSiz, TlutCount};
pub use printer::{MacroPrinter, Printer};
