// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

#[cfg(any(feature = "v4_8", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v4_8")))]
use crate::ContentFit;
use crate::{
    Accessible, AccessibleRole, Align, Buildable, ConstraintTarget, LayoutManager, Overflow, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkPicture")]
    pub struct Picture(Object<ffi::GtkPicture, ffi::GtkPictureClass>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget;

    match fn {
        type_ => || ffi::gtk_picture_get_type(),
    }
}

impl Picture {
    #[doc(alias = "gtk_picture_new")]
    pub fn new() -> Picture {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_picture_new()).unsafe_cast() }
    }

    #[doc(alias = "gtk_picture_new_for_file")]
    #[doc(alias = "new_for_file")]
    pub fn for_file(file: &impl IsA<gio::File>) -> Picture {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_picture_new_for_file(
                file.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_picture_new_for_filename")]
    #[doc(alias = "new_for_filename")]
    pub fn for_filename(filename: impl AsRef<std::path::Path>) -> Picture {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_picture_new_for_filename(
                filename.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_picture_new_for_paintable")]
    #[doc(alias = "new_for_paintable")]
    pub fn for_paintable(paintable: &impl IsA<gdk::Paintable>) -> Picture {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_picture_new_for_paintable(
                paintable.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_picture_new_for_pixbuf")]
    #[doc(alias = "new_for_pixbuf")]
    pub fn for_pixbuf(pixbuf: &gdk_pixbuf::Pixbuf) -> Picture {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_picture_new_for_pixbuf(pixbuf.to_glib_none().0))
                .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_picture_new_for_resource")]
    #[doc(alias = "new_for_resource")]
    pub fn for_resource(resource_path: &str) -> Picture {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_picture_new_for_resource(
                resource_path.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Picture`] objects.
    ///
    /// This method returns an instance of [`PictureBuilder`](crate::builders::PictureBuilder) which can be used to create [`Picture`] objects.
    pub fn builder() -> PictureBuilder {
        PictureBuilder::new()
    }

    #[doc(alias = "gtk_picture_get_alternative_text")]
    #[doc(alias = "get_alternative_text")]
    pub fn alternative_text(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_picture_get_alternative_text(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_picture_get_can_shrink")]
    #[doc(alias = "get_can_shrink")]
    pub fn can_shrink(&self) -> bool {
        unsafe { from_glib(ffi::gtk_picture_get_can_shrink(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v4_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_8")))]
    #[doc(alias = "gtk_picture_get_content_fit")]
    #[doc(alias = "get_content_fit")]
    pub fn content_fit(&self) -> ContentFit {
        unsafe { from_glib(ffi::gtk_picture_get_content_fit(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_picture_get_file")]
    #[doc(alias = "get_file")]
    pub fn file(&self) -> Option<gio::File> {
        unsafe { from_glib_none(ffi::gtk_picture_get_file(self.to_glib_none().0)) }
    }

    #[cfg_attr(feature = "v4_8", deprecated = "Since 4.8")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_picture_get_keep_aspect_ratio")]
    #[doc(alias = "get_keep_aspect_ratio")]
    pub fn is_keep_aspect_ratio(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_picture_get_keep_aspect_ratio(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_picture_get_paintable")]
    #[doc(alias = "get_paintable")]
    pub fn paintable(&self) -> Option<gdk::Paintable> {
        unsafe { from_glib_none(ffi::gtk_picture_get_paintable(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_picture_set_alternative_text")]
    pub fn set_alternative_text(&self, alternative_text: Option<&str>) {
        unsafe {
            ffi::gtk_picture_set_alternative_text(
                self.to_glib_none().0,
                alternative_text.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_picture_set_can_shrink")]
    pub fn set_can_shrink(&self, can_shrink: bool) {
        unsafe {
            ffi::gtk_picture_set_can_shrink(self.to_glib_none().0, can_shrink.into_glib());
        }
    }

    #[cfg(any(feature = "v4_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_8")))]
    #[doc(alias = "gtk_picture_set_content_fit")]
    pub fn set_content_fit(&self, content_fit: ContentFit) {
        unsafe {
            ffi::gtk_picture_set_content_fit(self.to_glib_none().0, content_fit.into_glib());
        }
    }

    #[doc(alias = "gtk_picture_set_file")]
    pub fn set_file(&self, file: Option<&impl IsA<gio::File>>) {
        unsafe {
            ffi::gtk_picture_set_file(
                self.to_glib_none().0,
                file.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_picture_set_filename")]
    pub fn set_filename(&self, filename: Option<impl AsRef<std::path::Path>>) {
        unsafe {
            ffi::gtk_picture_set_filename(
                self.to_glib_none().0,
                filename.as_ref().map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v4_8", deprecated = "Since 4.8")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_picture_set_keep_aspect_ratio")]
    pub fn set_keep_aspect_ratio(&self, keep_aspect_ratio: bool) {
        unsafe {
            ffi::gtk_picture_set_keep_aspect_ratio(
                self.to_glib_none().0,
                keep_aspect_ratio.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_picture_set_paintable")]
    pub fn set_paintable(&self, paintable: Option<&impl IsA<gdk::Paintable>>) {
        unsafe {
            ffi::gtk_picture_set_paintable(
                self.to_glib_none().0,
                paintable.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_picture_set_pixbuf")]
    pub fn set_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            ffi::gtk_picture_set_pixbuf(self.to_glib_none().0, pixbuf.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_picture_set_resource")]
    pub fn set_resource(&self, resource_path: Option<&str>) {
        unsafe {
            ffi::gtk_picture_set_resource(self.to_glib_none().0, resource_path.to_glib_none().0);
        }
    }

    #[doc(alias = "alternative-text")]
    pub fn connect_alternative_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_alternative_text_trampoline<F: Fn(&Picture) + 'static>(
            this: *mut ffi::GtkPicture,
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
                b"notify::alternative-text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_alternative_text_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "can-shrink")]
    pub fn connect_can_shrink_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_can_shrink_trampoline<F: Fn(&Picture) + 'static>(
            this: *mut ffi::GtkPicture,
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
                b"notify::can-shrink\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_can_shrink_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v4_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_8")))]
    #[doc(alias = "content-fit")]
    pub fn connect_content_fit_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_content_fit_trampoline<F: Fn(&Picture) + 'static>(
            this: *mut ffi::GtkPicture,
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
                b"notify::content-fit\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_content_fit_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "file")]
    pub fn connect_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_file_trampoline<F: Fn(&Picture) + 'static>(
            this: *mut ffi::GtkPicture,
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
                b"notify::file\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_file_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v4_8", deprecated = "Since 4.8")]
    #[doc(alias = "keep-aspect-ratio")]
    pub fn connect_keep_aspect_ratio_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_keep_aspect_ratio_trampoline<F: Fn(&Picture) + 'static>(
            this: *mut ffi::GtkPicture,
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
                b"notify::keep-aspect-ratio\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_keep_aspect_ratio_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "paintable")]
    pub fn connect_paintable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_paintable_trampoline<F: Fn(&Picture) + 'static>(
            this: *mut ffi::GtkPicture,
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
                b"notify::paintable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_paintable_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for Picture {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Picture`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct PictureBuilder {
    builder: glib::object::ObjectBuilder<'static, Picture>,
}

impl PictureBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn alternative_text(self, alternative_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("alternative-text", alternative_text.into()),
        }
    }

    pub fn can_shrink(self, can_shrink: bool) -> Self {
        Self {
            builder: self.builder.property("can-shrink", can_shrink),
        }
    }

    #[cfg(any(feature = "v4_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_8")))]
    pub fn content_fit(self, content_fit: ContentFit) -> Self {
        Self {
            builder: self.builder.property("content-fit", content_fit),
        }
    }

    pub fn file(self, file: &impl IsA<gio::File>) -> Self {
        Self {
            builder: self.builder.property("file", file.clone().upcast()),
        }
    }

    #[cfg_attr(feature = "v4_8", deprecated = "Since 4.8")]
    pub fn keep_aspect_ratio(self, keep_aspect_ratio: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("keep-aspect-ratio", keep_aspect_ratio),
        }
    }

    pub fn paintable(self, paintable: &impl IsA<gdk::Paintable>) -> Self {
        Self {
            builder: self
                .builder
                .property("paintable", paintable.clone().upcast()),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn can_target(self, can_target: bool) -> Self {
        Self {
            builder: self.builder.property("can-target", can_target),
        }
    }

    pub fn css_classes(self, css_classes: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("css-classes", css_classes.into()),
        }
    }

    pub fn css_name(self, css_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("css-name", css_name.into()),
        }
    }

    pub fn cursor(self, cursor: &gdk::Cursor) -> Self {
        Self {
            builder: self.builder.property("cursor", cursor.clone()),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn focusable(self, focusable: bool) -> Self {
        Self {
            builder: self.builder.property("focusable", focusable),
        }
    }

    pub fn halign(self, halign: Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn layout_manager(self, layout_manager: &impl IsA<LayoutManager>) -> Self {
        Self {
            builder: self
                .builder
                .property("layout-manager", layout_manager.clone().upcast()),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn overflow(self, overflow: Overflow) -> Self {
        Self {
            builder: self.builder.property("overflow", overflow),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    pub fn accessible_role(self, accessible_role: AccessibleRole) -> Self {
        Self {
            builder: self.builder.property("accessible-role", accessible_role),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Picture`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Picture {
        self.builder.build()
    }
}

impl fmt::Display for Picture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Picture")
    }
}