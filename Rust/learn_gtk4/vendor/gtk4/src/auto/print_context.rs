// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::PageSetup;
use glib::translate::*;
use std::{fmt, mem};

glib::wrapper! {
    #[doc(alias = "GtkPrintContext")]
    pub struct PrintContext(Object<ffi::GtkPrintContext>);

    match fn {
        type_ => || ffi::gtk_print_context_get_type(),
    }
}

impl PrintContext {
    #[doc(alias = "gtk_print_context_create_pango_context")]
    pub fn create_pango_context(&self) -> pango::Context {
        unsafe {
            from_glib_full(ffi::gtk_print_context_create_pango_context(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_print_context_create_pango_layout")]
    pub fn create_pango_layout(&self) -> pango::Layout {
        unsafe {
            from_glib_full(ffi::gtk_print_context_create_pango_layout(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_print_context_get_cairo_context")]
    #[doc(alias = "get_cairo_context")]
    pub fn cairo_context(&self) -> cairo::Context {
        unsafe {
            from_glib_none(ffi::gtk_print_context_get_cairo_context(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_print_context_get_dpi_x")]
    #[doc(alias = "get_dpi_x")]
    pub fn dpi_x(&self) -> f64 {
        unsafe { ffi::gtk_print_context_get_dpi_x(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_print_context_get_dpi_y")]
    #[doc(alias = "get_dpi_y")]
    pub fn dpi_y(&self) -> f64 {
        unsafe { ffi::gtk_print_context_get_dpi_y(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_print_context_get_hard_margins")]
    #[doc(alias = "get_hard_margins")]
    pub fn hard_margins(&self) -> Option<(f64, f64, f64, f64)> {
        unsafe {
            let mut top = mem::MaybeUninit::uninit();
            let mut bottom = mem::MaybeUninit::uninit();
            let mut left = mem::MaybeUninit::uninit();
            let mut right = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtk_print_context_get_hard_margins(
                self.to_glib_none().0,
                top.as_mut_ptr(),
                bottom.as_mut_ptr(),
                left.as_mut_ptr(),
                right.as_mut_ptr(),
            ));
            if ret {
                Some((
                    top.assume_init(),
                    bottom.assume_init(),
                    left.assume_init(),
                    right.assume_init(),
                ))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gtk_print_context_get_height")]
    #[doc(alias = "get_height")]
    pub fn height(&self) -> f64 {
        unsafe { ffi::gtk_print_context_get_height(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_print_context_get_page_setup")]
    #[doc(alias = "get_page_setup")]
    pub fn page_setup(&self) -> PageSetup {
        unsafe { from_glib_none(ffi::gtk_print_context_get_page_setup(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_print_context_get_pango_fontmap")]
    #[doc(alias = "get_pango_fontmap")]
    pub fn pango_fontmap(&self) -> pango::FontMap {
        unsafe {
            from_glib_none(ffi::gtk_print_context_get_pango_fontmap(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_print_context_get_width")]
    #[doc(alias = "get_width")]
    pub fn width(&self) -> f64 {
        unsafe { ffi::gtk_print_context_get_width(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_print_context_set_cairo_context")]
    pub fn set_cairo_context(&self, cr: &cairo::Context, dpi_x: f64, dpi_y: f64) {
        unsafe {
            ffi::gtk_print_context_set_cairo_context(
                self.to_glib_none().0,
                mut_override(cr.to_glib_none().0),
                dpi_x,
                dpi_y,
            );
        }
    }
}

impl fmt::Display for PrintContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PrintContext")
    }
}