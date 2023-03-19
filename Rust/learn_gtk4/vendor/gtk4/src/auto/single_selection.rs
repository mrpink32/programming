// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::SelectionModel;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkSingleSelection")]
    pub struct SingleSelection(Object<ffi::GtkSingleSelection, ffi::GtkSingleSelectionClass>) @implements gio::ListModel, SelectionModel;

    match fn {
        type_ => || ffi::gtk_single_selection_get_type(),
    }
}

impl SingleSelection {
    #[doc(alias = "gtk_single_selection_new")]
    pub fn new(model: Option<impl IsA<gio::ListModel>>) -> SingleSelection {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_single_selection_new(
                model.map(|p| p.upcast()).into_glib_ptr(),
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`SingleSelection`] objects.
    ///
    /// This method returns an instance of [`SingleSelectionBuilder`](crate::builders::SingleSelectionBuilder) which can be used to create [`SingleSelection`] objects.
    pub fn builder() -> SingleSelectionBuilder {
        SingleSelectionBuilder::new()
    }

    #[doc(alias = "gtk_single_selection_get_autoselect")]
    #[doc(alias = "get_autoselect")]
    pub fn is_autoselect(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_single_selection_get_autoselect(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_single_selection_get_can_unselect")]
    #[doc(alias = "get_can_unselect")]
    pub fn can_unselect(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_single_selection_get_can_unselect(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_single_selection_get_model")]
    #[doc(alias = "get_model")]
    pub fn model(&self) -> Option<gio::ListModel> {
        unsafe { from_glib_none(ffi::gtk_single_selection_get_model(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_single_selection_get_selected")]
    #[doc(alias = "get_selected")]
    pub fn selected(&self) -> u32 {
        unsafe { ffi::gtk_single_selection_get_selected(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_single_selection_get_selected_item")]
    #[doc(alias = "get_selected_item")]
    pub fn selected_item(&self) -> Option<glib::Object> {
        unsafe {
            from_glib_none(ffi::gtk_single_selection_get_selected_item(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_single_selection_set_autoselect")]
    pub fn set_autoselect(&self, autoselect: bool) {
        unsafe {
            ffi::gtk_single_selection_set_autoselect(self.to_glib_none().0, autoselect.into_glib());
        }
    }

    #[doc(alias = "gtk_single_selection_set_can_unselect")]
    pub fn set_can_unselect(&self, can_unselect: bool) {
        unsafe {
            ffi::gtk_single_selection_set_can_unselect(
                self.to_glib_none().0,
                can_unselect.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_single_selection_set_model")]
    pub fn set_model(&self, model: Option<&impl IsA<gio::ListModel>>) {
        unsafe {
            ffi::gtk_single_selection_set_model(
                self.to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_single_selection_set_selected")]
    pub fn set_selected(&self, position: u32) {
        unsafe {
            ffi::gtk_single_selection_set_selected(self.to_glib_none().0, position);
        }
    }

    #[doc(alias = "autoselect")]
    pub fn connect_autoselect_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_autoselect_trampoline<F: Fn(&SingleSelection) + 'static>(
            this: *mut ffi::GtkSingleSelection,
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
                b"notify::autoselect\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_autoselect_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "can-unselect")]
    pub fn connect_can_unselect_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_can_unselect_trampoline<F: Fn(&SingleSelection) + 'static>(
            this: *mut ffi::GtkSingleSelection,
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
                b"notify::can-unselect\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_can_unselect_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "model")]
    pub fn connect_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_trampoline<F: Fn(&SingleSelection) + 'static>(
            this: *mut ffi::GtkSingleSelection,
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
                b"notify::model\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_model_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "selected")]
    pub fn connect_selected_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_selected_trampoline<F: Fn(&SingleSelection) + 'static>(
            this: *mut ffi::GtkSingleSelection,
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
                b"notify::selected\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_selected_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "selected-item")]
    pub fn connect_selected_item_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_selected_item_trampoline<F: Fn(&SingleSelection) + 'static>(
            this: *mut ffi::GtkSingleSelection,
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
                b"notify::selected-item\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_selected_item_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for SingleSelection {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`SingleSelection`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct SingleSelectionBuilder {
    builder: glib::object::ObjectBuilder<'static, SingleSelection>,
}

impl SingleSelectionBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn autoselect(self, autoselect: bool) -> Self {
        Self {
            builder: self.builder.property("autoselect", autoselect),
        }
    }

    pub fn can_unselect(self, can_unselect: bool) -> Self {
        Self {
            builder: self.builder.property("can-unselect", can_unselect),
        }
    }

    pub fn model(self, model: &impl IsA<gio::ListModel>) -> Self {
        Self {
            builder: self.builder.property("model", model.clone().upcast()),
        }
    }

    pub fn selected(self, selected: u32) -> Self {
        Self {
            builder: self.builder.property("selected", selected),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`SingleSelection`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> SingleSelection {
        self.builder.build()
    }
}

impl fmt::Display for SingleSelection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SingleSelection")
    }
}
