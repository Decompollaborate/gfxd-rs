/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use alloc::string::String;
use core::marker::PhantomPinned;

use gfxd_sys::ptr::NonNullMut;

use crate::Customizer;

/// Data that is passed around within libgfxd by using `gfxd_udata_set` and
/// `gfxd_udata_get`.
///
/// The whole struct is passed to libgfxd. We only use udata_* functions with
/// this type, and only this type.
#[must_use]
pub(crate) struct LibData<'c, 'vtx> {
    out_buf: String,
    customizer: &'c mut Customizer<'vtx>,
    _pin: PhantomPinned,
}

impl<'c, 'vtx> LibData<'c, 'vtx> {
    pub(crate) fn new(customizer: &'c mut Customizer<'vtx>) -> Self {
        Self {
            out_buf: String::new(),
            customizer,
            _pin: PhantomPinned,
        }
    }

    pub(crate) fn consume(self) -> String {
        self.out_buf
    }

    pub(crate) fn gfxd_set<'l>(&'l mut self) -> LibDataWrap<'l, 'c, 'vtx> {
        LibDataWrap::new(self)
    }

    #[must_use]
    pub(crate) fn get<'a>() -> Option<&'a mut Self> {
        let data = unsafe { gfxd_sys::settings::gfxd_udata_get() };

        data.map(|x| unsafe { x.cast().as_mut() })
    }

    pub(crate) fn write_to_buf(&mut self, string: &str) {
        self.out_buf.push_str(string);
    }

    pub(crate) fn get_customizer_mut<'slf>(&'slf mut self) -> &'c mut Customizer<'vtx>
    where
        'slf: 'c,
    {
        self.customizer
    }
}

pub(crate) struct LibDataWrap<'l, 'c, 'vtx> {
    lib_data: &'l mut LibData<'c, 'vtx>,
}

impl<'l, 'c, 'vtx> LibDataWrap<'l, 'c, 'vtx> {
    fn new(lib_data: &'l mut LibData<'c, 'vtx>) -> Self {
        assert!(unsafe { gfxd_sys::settings::gfxd_udata_get() }.is_none());

        let me = NonNullMut::new_void(lib_data);
        assert!(me.is_some());
        unsafe {
            gfxd_sys::settings::gfxd_udata_set(me);
        }

        lib_data.customizer.apply_callbacks();

        Self { lib_data }
    }
}

impl Drop for LibDataWrap<'_, '_, '_> {
    fn drop(&mut self) {
        let me = NonNullMut::new_void(self.lib_data);

        let current = unsafe { gfxd_sys::settings::gfxd_udata_get() };

        if me == current {
            unsafe {
                gfxd_sys::settings::gfxd_udata_set(None);
            }
        }
    }
}
