// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Expression, Filter};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkBoolFilter")]
    pub struct BoolFilter(Object<ffi::GtkBoolFilter, ffi::GtkBoolFilterClass>) @extends Filter;

    match fn {
        type_ => || ffi::gtk_bool_filter_get_type(),
    }
}

impl BoolFilter {
    #[doc(alias = "gtk_bool_filter_new")]
    pub fn new(expression: Option<impl AsRef<Expression>>) -> BoolFilter {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_bool_filter_new(
                expression
                    .map(|p| p.as_ref().clone().upcast())
                    .into_glib_ptr(),
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`BoolFilter`] objects.
    ///
    /// This method returns an instance of [`BoolFilterBuilder`](crate::builders::BoolFilterBuilder) which can be used to create [`BoolFilter`] objects.
    pub fn builder() -> BoolFilterBuilder {
        BoolFilterBuilder::new()
    }

    #[doc(alias = "gtk_bool_filter_get_expression")]
    #[doc(alias = "get_expression")]
    pub fn expression(&self) -> Option<Expression> {
        unsafe { from_glib_none(ffi::gtk_bool_filter_get_expression(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_bool_filter_get_invert")]
    #[doc(alias = "get_invert")]
    pub fn inverts(&self) -> bool {
        unsafe { from_glib(ffi::gtk_bool_filter_get_invert(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_bool_filter_set_expression")]
    pub fn set_expression(&self, expression: Option<impl AsRef<Expression>>) {
        unsafe {
            ffi::gtk_bool_filter_set_expression(
                self.to_glib_none().0,
                expression.as_ref().map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_bool_filter_set_invert")]
    pub fn set_invert(&self, invert: bool) {
        unsafe {
            ffi::gtk_bool_filter_set_invert(self.to_glib_none().0, invert.into_glib());
        }
    }

    #[doc(alias = "expression")]
    pub fn connect_expression_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_expression_trampoline<F: Fn(&BoolFilter) + 'static>(
            this: *mut ffi::GtkBoolFilter,
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
                b"notify::expression\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_expression_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "invert")]
    pub fn connect_invert_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_invert_trampoline<F: Fn(&BoolFilter) + 'static>(
            this: *mut ffi::GtkBoolFilter,
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
                b"notify::invert\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_invert_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for BoolFilter {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`BoolFilter`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct BoolFilterBuilder {
    builder: glib::object::ObjectBuilder<'static, BoolFilter>,
}

impl BoolFilterBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn expression(self, expression: impl AsRef<Expression>) -> Self {
        Self {
            builder: self
                .builder
                .property("expression", expression.as_ref().clone()),
        }
    }

    pub fn invert(self, invert: bool) -> Self {
        Self {
            builder: self.builder.property("invert", invert),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`BoolFilter`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> BoolFilter {
        self.builder.build()
    }
}

impl fmt::Display for BoolFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("BoolFilter")
    }
}
