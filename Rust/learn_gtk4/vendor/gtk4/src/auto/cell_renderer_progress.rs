// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{CellRenderer, CellRendererMode, Orientable, Orientation};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkCellRendererProgress")]
    pub struct CellRendererProgress(Object<ffi::GtkCellRendererProgress>) @extends CellRenderer, @implements Orientable;

    match fn {
        type_ => || ffi::gtk_cell_renderer_progress_get_type(),
    }
}

impl CellRendererProgress {
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_renderer_progress_new")]
    pub fn new() -> CellRendererProgress {
        assert_initialized_main_thread!();
        unsafe { CellRenderer::from_glib_none(ffi::gtk_cell_renderer_progress_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`CellRendererProgress`] objects.
    ///
    /// This method returns an instance of [`CellRendererProgressBuilder`](crate::builders::CellRendererProgressBuilder) which can be used to create [`CellRendererProgress`] objects.
    pub fn builder() -> CellRendererProgressBuilder {
        CellRendererProgressBuilder::new()
    }

    pub fn is_inverted(&self) -> bool {
        glib::ObjectExt::property(self, "inverted")
    }

    pub fn set_inverted(&self, inverted: bool) {
        glib::ObjectExt::set_property(self, "inverted", inverted)
    }

    pub fn pulse(&self) -> i32 {
        glib::ObjectExt::property(self, "pulse")
    }

    pub fn set_pulse(&self, pulse: i32) {
        glib::ObjectExt::set_property(self, "pulse", pulse)
    }

    pub fn text(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self, "text")
    }

    pub fn set_text(&self, text: Option<&str>) {
        glib::ObjectExt::set_property(self, "text", text)
    }

    #[doc(alias = "text-xalign")]
    pub fn text_xalign(&self) -> f32 {
        glib::ObjectExt::property(self, "text-xalign")
    }

    #[doc(alias = "text-xalign")]
    pub fn set_text_xalign(&self, text_xalign: f32) {
        glib::ObjectExt::set_property(self, "text-xalign", text_xalign)
    }

    #[doc(alias = "text-yalign")]
    pub fn text_yalign(&self) -> f32 {
        glib::ObjectExt::property(self, "text-yalign")
    }

    #[doc(alias = "text-yalign")]
    pub fn set_text_yalign(&self, text_yalign: f32) {
        glib::ObjectExt::set_property(self, "text-yalign", text_yalign)
    }

    pub fn value(&self) -> i32 {
        glib::ObjectExt::property(self, "value")
    }

    pub fn set_value(&self, value: i32) {
        glib::ObjectExt::set_property(self, "value", value)
    }

    #[doc(alias = "inverted")]
    pub fn connect_inverted_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_inverted_trampoline<F: Fn(&CellRendererProgress) + 'static>(
            this: *mut ffi::GtkCellRendererProgress,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::inverted\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_inverted_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pulse")]
    pub fn connect_pulse_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pulse_trampoline<F: Fn(&CellRendererProgress) + 'static>(
            this: *mut ffi::GtkCellRendererProgress,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::pulse\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pulse_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "text")]
    pub fn connect_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_trampoline<F: Fn(&CellRendererProgress) + 'static>(
            this: *mut ffi::GtkCellRendererProgress,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_text_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "text-xalign")]
    pub fn connect_text_xalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_xalign_trampoline<
            F: Fn(&CellRendererProgress) + 'static,
        >(
            this: *mut ffi::GtkCellRendererProgress,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::text-xalign\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_text_xalign_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "text-yalign")]
    pub fn connect_text_yalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_yalign_trampoline<
            F: Fn(&CellRendererProgress) + 'static,
        >(
            this: *mut ffi::GtkCellRendererProgress,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::text-yalign\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_text_yalign_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "value")]
    pub fn connect_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_value_trampoline<F: Fn(&CellRendererProgress) + 'static>(
            this: *mut ffi::GtkCellRendererProgress,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::value\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_value_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for CellRendererProgress {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`CellRendererProgress`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct CellRendererProgressBuilder {
    builder: glib::object::ObjectBuilder<'static, CellRendererProgress>,
}

impl CellRendererProgressBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn inverted(self, inverted: bool) -> Self {
        Self {
            builder: self.builder.property("inverted", inverted),
        }
    }

    pub fn pulse(self, pulse: i32) -> Self {
        Self {
            builder: self.builder.property("pulse", pulse),
        }
    }

    pub fn text(self, text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("text", text.into()),
        }
    }

    pub fn text_xalign(self, text_xalign: f32) -> Self {
        Self {
            builder: self.builder.property("text-xalign", text_xalign),
        }
    }

    pub fn text_yalign(self, text_yalign: f32) -> Self {
        Self {
            builder: self.builder.property("text-yalign", text_yalign),
        }
    }

    pub fn value(self, value: i32) -> Self {
        Self {
            builder: self.builder.property("value", value),
        }
    }

    pub fn cell_background(self, cell_background: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("cell-background", cell_background.into()),
        }
    }

    pub fn cell_background_rgba(self, cell_background_rgba: &gdk::RGBA) -> Self {
        Self {
            builder: self
                .builder
                .property("cell-background-rgba", cell_background_rgba),
        }
    }

    pub fn cell_background_set(self, cell_background_set: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("cell-background-set", cell_background_set),
        }
    }

    pub fn height(self, height: i32) -> Self {
        Self {
            builder: self.builder.property("height", height),
        }
    }

    pub fn is_expanded(self, is_expanded: bool) -> Self {
        Self {
            builder: self.builder.property("is-expanded", is_expanded),
        }
    }

    pub fn is_expander(self, is_expander: bool) -> Self {
        Self {
            builder: self.builder.property("is-expander", is_expander),
        }
    }

    pub fn mode(self, mode: CellRendererMode) -> Self {
        Self {
            builder: self.builder.property("mode", mode),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width(self, width: i32) -> Self {
        Self {
            builder: self.builder.property("width", width),
        }
    }

    pub fn xalign(self, xalign: f32) -> Self {
        Self {
            builder: self.builder.property("xalign", xalign),
        }
    }

    pub fn xpad(self, xpad: u32) -> Self {
        Self {
            builder: self.builder.property("xpad", xpad),
        }
    }

    pub fn yalign(self, yalign: f32) -> Self {
        Self {
            builder: self.builder.property("yalign", yalign),
        }
    }

    pub fn ypad(self, ypad: u32) -> Self {
        Self {
            builder: self.builder.property("ypad", ypad),
        }
    }

    pub fn orientation(self, orientation: Orientation) -> Self {
        Self {
            builder: self.builder.property("orientation", orientation),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`CellRendererProgress`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> CellRendererProgress {
        self.builder.build()
    }
}

impl fmt::Display for CellRendererProgress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CellRendererProgress")
    }
}
