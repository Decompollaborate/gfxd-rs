/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use alloc::string::{String, ToString};
use gfxd_sys::{ffi, ptr::NonNullConst};

use crate::{
    lib_data::{LibData, LibDataWrap},
    Customizer, Microcode,
};

// TODO: figure out where and how to expose gfxd_arg_callbacks

/// `Gfx` packet macro disassembler.
///
/// This struct allows configuring general settings that control the generated
/// output. Once the disassembler is configured, call [`disassemble`] to
/// produce a string with the disassembly of the passed Gfx packets.
///
/// Refer to the [`Customizer`] struct to further customize how each `Gfx`
/// macro is outputted or register callbacks for the different kinds of macros.
///
/// ## Examples
///
/// Disassemble F3DEX packets without any kind of customization
///
/// ```rust
/// use gfxd_rs::{Customizer, Disassembler, Microcode};
///
/// pub fn plain_disasm_f3dex(data: &[u8]) -> String {
///     let mut customizer = Customizer::new();
///
///     Disassembler::new()
///         .disassemble(data, Microcode::F3dex, &mut customizer)
/// }
/// ```
///
/// Use a dynamic argument and print each macro on a different line.
///
/// ```rust
/// use gfxd_rs::{Customizer, Disassembler, MacroPrinter, Microcode};
///
/// pub fn disasm_pretty(data: &[u8], microcode: Microcode, dynamic: &str) -> String {
///     let mut customizer = Customizer::new();
///
///     // This has to be binded to a local variable to avoid dropping it too soon.
///     let mut macro_fn = |printer: &mut MacroPrinter, _info: &mut _| {
///         // Write 4 spaces.
///         printer.write_str("    ");
///         // Call the original macro handler, to emit the macro as-is.
///         let ret = printer.macro_dflt();
///         // Write a newline after the macro.
///         printer.write_str(",\n");
///         ret
///     };
///     customizer
///         .macro_fn(&mut macro_fn);
///
///     Disassembler::new()
///         .dynamic(Some(dynamic))
///         .disassemble(data, microcode, &mut customizer)
/// }
/// ```
///
/// [`disassemble`]: Disassembler::disassemble
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Disassembler<'d> {
    // TODO: endian and wordsize
    dynamic: Option<&'d str>,
    stop_on_invalid: bool,
    stop_on_end: bool,
    emit_dec_color: bool,
    emit_q_macro: bool,
    emit_ext_macro: bool,
}

impl<'d> Disassembler<'d> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            dynamic: None,
            stop_on_invalid: true,
            stop_on_end: true,
            emit_dec_color: false,
            emit_q_macro: false,
            emit_ext_macro: false,
        }
    }

    /// Enable or disable the use of dynamic `g` macros instead of static `gs`
    /// macros, and select the dynamic display list pointer argument to be used.
    ///
    /// If `Some`, this value will be used by [`macro_dflt`] as the first
    /// argument to dynamic macros.
    /// If `None`, dynamic macros are disabled and `gs` macros are used.
    /// Defaults to `None`.
    ///
    /// Also affects the result of [`macro_name`], as it will return either the
    /// dynamic or static version of the macro name as selected by this
    /// setting.
    ///
    /// [`macro_dflt`]: crate::MacroPrinter::macro_dflt
    /// [`macro_name`]: crate::MacroInfo::macro_name
    pub fn dynamic(&mut self, dynamic: Option<&'d str>) -> &mut Self {
        self.dynamic = dynamic;
        self
    }

    /// Stop execution when encountering an invalid macro.
    ///
    /// Enabled by default.
    pub fn stop_on_invalid(&mut self, value: bool) -> &mut Self {
        self.stop_on_invalid = value;
        self
    }
    /// Stop execution when encountering a [`SPBranchList`] or
    /// [`SPEndDisplayList`].
    ///
    /// Enabled by default.
    ///
    /// [`SPBranchList`]: crate::MacroId::SPBranchList
    /// [`SPEndDisplayList`]: crate::MacroId::SPEndDisplayList
    pub fn stop_on_end(&mut self, value: bool) -> &mut Self {
        self.stop_on_end = value;
        self
    }
    /// Print color components as decimal instead of hexadecimal.
    ///
    /// Disabled by default.
    pub fn emit_dec_color(&mut self, value: bool) -> &mut Self {
        self.emit_dec_color = value;
        self
    }
    /// Print fixed-point conversion `q` macros for fixed-point values.
    ///
    /// Disabled by default.
    pub fn emit_q_macro(&mut self, value: bool) -> &mut Self {
        self.emit_q_macro = value;
        self
    }
    /// Emit non-standard macros.
    ///
    /// Some commands are valid (though possibly meaningless), but have no
    /// macros associated with them, such as a standalone `G_RDPHALF_1`.
    /// When this feature is enabled, such a command will produce a
    /// non-standard [`gsDPHalf1`] macro instead of a raw hexadecimal command.
    ///
    /// Also enables some non-standard multi-packet texture loading macros.
    ///
    /// Disabled by default.
    ///
    /// [`gsDPHalf1`]: crate::MacroId::DPHalf1
    pub fn emit_ext_macro(&mut self, value: bool) -> &mut Self {
        self.emit_ext_macro = value;
        self
    }

    /// Start executing `gfxd` with the current settings.
    ///
    /// The `data` argument is a big endian byte array containing the `Gfx`
    /// packets to be disassembled.
    ///
    /// `microcode` corresponds to the target microcode to decode the data.
    ///
    /// `customizer` allows registering callbacks to customize the output or to
    /// extract data from each macro type.
    ///
    /// For each macro, the macro handler registered with [`macro_fn`] is
    /// called.
    ///
    /// Execution ends when:
    /// - the input ends,
    /// - the macro handler returns [`Stop`],
    /// - when an invalid macro is encountered and [`stop_on_invalid`] is
    ///   enabled,
    /// - or when [`SPBranchList`] or [`SPEndDisplayList`] is encountered and
    ///   [`stop_on_end`] is enabled.
    ///
    /// [`macro_fn`]: Customizer::macro_fn
    /// [`Stop`]: crate::MacroFnRet::Stop
    /// [`stop_on_invalid`]: Disassembler::stop_on_invalid
    /// [`stop_on_end`]: Disassembler::stop_on_end
    /// [`SPBranchList`]: crate::MacroId::SPBranchList
    /// [`SPEndDisplayList`]: crate::MacroId::SPEndDisplayList
    #[must_use]
    pub fn disassemble(
        self,
        data: &[u8],
        microcode: Microcode,
        customizer: &mut Customizer,
    ) -> String {
        let mut lib_data = LibData::new(customizer);
        // Make sure we don't get surprises by the data somehow moving.
        // tbh I'm not sure if this is needed at all, I need to investigate.
        // let lib_data = alloc::boxed::Box::pin(lib_data);

        // We need to ensure this String doesn't get dropped before we execute
        // gfxd. Allocating it outside disassemble_impl and passing the
        // reference should ensure that, as far as I understand it.
        let dynamic = self.dynamic.map(|x| {
            let mut x = x.to_string();
            // nul-terminate the string
            x.push('\0');
            x
        });
        // `as_ref` is needed, otherwise the string gets consumed and this ends
        // up pointing to itself or something weird like that.
        let dynamic_ptr = dynamic
            .as_ref()
            .and_then(|x| NonNullConst::new(x.as_ptr().cast()));

        {
            let mut lib_data_wrap = lib_data.gfxd_set();

            self.disassemble_impl(data, microcode, &mut lib_data_wrap, dynamic_ptr);
        }

        lib_data.consume()
    }

    // Use a wrapper function to make sure lib_data does not get dropped too
    // soon.
    fn disassemble_impl(
        self,
        data: &[u8],
        microcode: Microcode,
        lib_data_wrap: &mut LibDataWrap,
        dynamic: Option<NonNullConst<ffi::c_char>>,
    ) {
        // Setup input and output
        unsafe {
            // We only use the input_buffer and the output_callback functions
            // of libgfxd, completely ignoring the output_buffer, input_callback
            // or the fd ones.
            // I don't know if it is worth to expose those in the API.

            gfxd_sys::io::gfxd_input_buffer(NonNullConst::new_void(data.as_ptr()), data.len() as _);
        }

        // Set the microcode
        unsafe {
            gfxd_sys::settings::gfxd_target(Some(microcode.to_microcode_ptr()));
        }

        // Set the dynamic arg, if any
        unsafe {
            gfxd_sys::settings::gfxd_dynamic(dynamic);
        }

        // Set the options
        set_feature_option(
            self.stop_on_invalid,
            gfxd_sys::settings::FeatureOption::gfxd_stop_on_invalid,
        );
        set_feature_option(
            self.stop_on_end,
            gfxd_sys::settings::FeatureOption::gfxd_stop_on_end,
        );
        set_feature_option(
            self.emit_dec_color,
            gfxd_sys::settings::FeatureOption::gfxd_emit_dec_color,
        );
        set_feature_option(
            self.emit_q_macro,
            gfxd_sys::settings::FeatureOption::gfxd_emit_q_macro,
        );
        set_feature_option(
            self.emit_ext_macro,
            gfxd_sys::settings::FeatureOption::gfxd_emit_ext_macro,
        );

        // Run
        lib_data_wrap.do_before();
        unsafe {
            gfxd_sys::execution::gfxd_execute();
        }
        lib_data_wrap.do_after();
    }
}

fn set_feature_option(on: bool, cap: gfxd_sys::settings::FeatureOption) {
    unsafe {
        if on {
            gfxd_sys::settings::gfxd_enable(cap);
        } else {
            gfxd_sys::settings::gfxd_disable(cap);
        }
    }
}

impl Default for Disassembler<'_> {
    fn default() -> Self {
        Self::new()
    }
}
