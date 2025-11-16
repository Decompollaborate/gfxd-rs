/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use core::slice;
use gfxd_sys::{ffi, ptr::NonNullConst};

extern "C" {
    fn strlen(s: *const ffi::c_char) -> usize;
}

/// # SAFETY
///
/// The pointer must be a valid buffer that is nul-terminated, and it should
/// only contain valid UTF-8 values.
#[inline]
pub unsafe fn str_from_c_str<'s>(c_string: NonNullConst<ffi::c_char>) -> &'s str {
    let ptr = c_string.as_ptr();
    // SAFETY: Parameter is expected to be a valid C string.
    let len = unsafe { strlen(ptr) };

    let bytes = unsafe { slice::from_raw_parts(ptr.cast(), len) };

    // SAFETY: The whole library only exposes UTF-8 valid types, and the inner
    // gfxd library uses ASCII only.
    unsafe { core::str::from_utf8_unchecked(bytes) }
}
