// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{LayoutChild, Orientation, SizeRequestMode, Widget};
use glib::{prelude::*, translate::*};
use std::{fmt, mem};

glib::wrapper! {
    #[doc(alias = "GtkLayoutManager")]
    pub struct LayoutManager(Object<ffi::GtkLayoutManager, ffi::GtkLayoutManagerClass>);

    match fn {
        type_ => || ffi::gtk_layout_manager_get_type(),
    }
}

impl LayoutManager {
    pub const NONE: Option<&'static LayoutManager> = None;
}

pub trait LayoutManagerExt: 'static {
    #[doc(alias = "gtk_layout_manager_allocate")]
    fn allocate(&self, widget: &impl IsA<Widget>, width: i32, height: i32, baseline: i32);

    #[doc(alias = "gtk_layout_manager_get_layout_child")]
    #[doc(alias = "get_layout_child")]
    fn layout_child(&self, child: &impl IsA<Widget>) -> LayoutChild;

    #[doc(alias = "gtk_layout_manager_get_request_mode")]
    #[doc(alias = "get_request_mode")]
    fn request_mode(&self) -> SizeRequestMode;

    #[doc(alias = "gtk_layout_manager_get_widget")]
    #[doc(alias = "get_widget")]
    fn widget(&self) -> Option<Widget>;

    #[doc(alias = "gtk_layout_manager_layout_changed")]
    fn layout_changed(&self);

    #[doc(alias = "gtk_layout_manager_measure")]
    fn measure(
        &self,
        widget: &impl IsA<Widget>,
        orientation: Orientation,
        for_size: i32,
    ) -> (i32, i32, i32, i32);
}

impl<O: IsA<LayoutManager>> LayoutManagerExt for O {
    fn allocate(&self, widget: &impl IsA<Widget>, width: i32, height: i32, baseline: i32) {
        unsafe {
            ffi::gtk_layout_manager_allocate(
                self.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                width,
                height,
                baseline,
            );
        }
    }

    fn layout_child(&self, child: &impl IsA<Widget>) -> LayoutChild {
        unsafe {
            from_glib_none(ffi::gtk_layout_manager_get_layout_child(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    fn request_mode(&self) -> SizeRequestMode {
        unsafe {
            from_glib(ffi::gtk_layout_manager_get_request_mode(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_layout_manager_get_widget(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn layout_changed(&self) {
        unsafe {
            ffi::gtk_layout_manager_layout_changed(self.as_ref().to_glib_none().0);
        }
    }

    fn measure(
        &self,
        widget: &impl IsA<Widget>,
        orientation: Orientation,
        for_size: i32,
    ) -> (i32, i32, i32, i32) {
        unsafe {
            let mut minimum = mem::MaybeUninit::uninit();
            let mut natural = mem::MaybeUninit::uninit();
            let mut minimum_baseline = mem::MaybeUninit::uninit();
            let mut natural_baseline = mem::MaybeUninit::uninit();
            ffi::gtk_layout_manager_measure(
                self.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                orientation.into_glib(),
                for_size,
                minimum.as_mut_ptr(),
                natural.as_mut_ptr(),
                minimum_baseline.as_mut_ptr(),
                natural_baseline.as_mut_ptr(),
            );
            (
                minimum.assume_init(),
                natural.assume_init(),
                minimum_baseline.assume_init(),
                natural_baseline.assume_init(),
            )
        }
    }
}

impl fmt::Display for LayoutManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("LayoutManager")
    }
}