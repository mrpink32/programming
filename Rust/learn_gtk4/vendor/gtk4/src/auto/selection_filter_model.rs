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
    #[doc(alias = "GtkSelectionFilterModel")]
    pub struct SelectionFilterModel(Object<ffi::GtkSelectionFilterModel, ffi::GtkSelectionFilterModelClass>) @implements gio::ListModel;

    match fn {
        type_ => || ffi::gtk_selection_filter_model_get_type(),
    }
}

impl SelectionFilterModel {
    #[doc(alias = "gtk_selection_filter_model_new")]
    pub fn new(model: Option<&impl IsA<SelectionModel>>) -> SelectionFilterModel {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_selection_filter_model_new(
                model.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_selection_filter_model_get_model")]
    #[doc(alias = "get_model")]
    pub fn model(&self) -> Option<SelectionModel> {
        unsafe {
            from_glib_none(ffi::gtk_selection_filter_model_get_model(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_selection_filter_model_set_model")]
    pub fn set_model(&self, model: Option<&impl IsA<SelectionModel>>) {
        unsafe {
            ffi::gtk_selection_filter_model_set_model(
                self.to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "model")]
    pub fn connect_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_trampoline<F: Fn(&SelectionFilterModel) + 'static>(
            this: *mut ffi::GtkSelectionFilterModel,
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
}

impl fmt::Display for SelectionFilterModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SelectionFilterModel")
    }
}