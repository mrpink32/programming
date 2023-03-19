// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkDirectoryList")]
    pub struct DirectoryList(Object<ffi::GtkDirectoryList, ffi::GtkDirectoryListClass>) @implements gio::ListModel;

    match fn {
        type_ => || ffi::gtk_directory_list_get_type(),
    }
}

impl DirectoryList {
    #[doc(alias = "gtk_directory_list_new")]
    pub fn new(attributes: Option<&str>, file: Option<&impl IsA<gio::File>>) -> DirectoryList {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_directory_list_new(
                attributes.to_glib_none().0,
                file.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_directory_list_get_attributes")]
    #[doc(alias = "get_attributes")]
    pub fn attributes(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_directory_list_get_attributes(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_directory_list_get_error")]
    #[doc(alias = "get_error")]
    pub fn error(&self) -> Option<glib::Error> {
        unsafe { from_glib_none(ffi::gtk_directory_list_get_error(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_directory_list_get_file")]
    #[doc(alias = "get_file")]
    pub fn file(&self) -> Option<gio::File> {
        unsafe { from_glib_none(ffi::gtk_directory_list_get_file(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_directory_list_get_monitored")]
    #[doc(alias = "get_monitored")]
    pub fn is_monitored(&self) -> bool {
        unsafe { from_glib(ffi::gtk_directory_list_get_monitored(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_directory_list_is_loading")]
    pub fn is_loading(&self) -> bool {
        unsafe { from_glib(ffi::gtk_directory_list_is_loading(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_directory_list_set_attributes")]
    pub fn set_attributes(&self, attributes: Option<&str>) {
        unsafe {
            ffi::gtk_directory_list_set_attributes(
                self.to_glib_none().0,
                attributes.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_directory_list_set_file")]
    pub fn set_file(&self, file: Option<&impl IsA<gio::File>>) {
        unsafe {
            ffi::gtk_directory_list_set_file(
                self.to_glib_none().0,
                file.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_directory_list_set_monitored")]
    pub fn set_monitored(&self, monitored: bool) {
        unsafe {
            ffi::gtk_directory_list_set_monitored(self.to_glib_none().0, monitored.into_glib());
        }
    }

    #[doc(alias = "attributes")]
    pub fn connect_attributes_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_attributes_trampoline<F: Fn(&DirectoryList) + 'static>(
            this: *mut ffi::GtkDirectoryList,
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
                b"notify::attributes\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_attributes_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "error")]
    pub fn connect_error_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_error_trampoline<F: Fn(&DirectoryList) + 'static>(
            this: *mut ffi::GtkDirectoryList,
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
                b"notify::error\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_error_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "file")]
    pub fn connect_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_file_trampoline<F: Fn(&DirectoryList) + 'static>(
            this: *mut ffi::GtkDirectoryList,
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

    #[doc(alias = "io-priority")]
    pub fn connect_io_priority_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_io_priority_trampoline<F: Fn(&DirectoryList) + 'static>(
            this: *mut ffi::GtkDirectoryList,
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
                b"notify::io-priority\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_io_priority_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "loading")]
    pub fn connect_loading_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_loading_trampoline<F: Fn(&DirectoryList) + 'static>(
            this: *mut ffi::GtkDirectoryList,
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
                b"notify::loading\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_loading_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "monitored")]
    pub fn connect_monitored_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_monitored_trampoline<F: Fn(&DirectoryList) + 'static>(
            this: *mut ffi::GtkDirectoryList,
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
                b"notify::monitored\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_monitored_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DirectoryList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DirectoryList")
    }
}
