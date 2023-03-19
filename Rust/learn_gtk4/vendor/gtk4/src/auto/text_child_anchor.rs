// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Widget;
use glib::{prelude::*, translate::*};
use std::{fmt, mem};

glib::wrapper! {
    #[doc(alias = "GtkTextChildAnchor")]
    pub struct TextChildAnchor(Object<ffi::GtkTextChildAnchor, ffi::GtkTextChildAnchorClass>);

    match fn {
        type_ => || ffi::gtk_text_child_anchor_get_type(),
    }
}

impl TextChildAnchor {
    pub const NONE: Option<&'static TextChildAnchor> = None;

    #[doc(alias = "gtk_text_child_anchor_new")]
    pub fn new() -> TextChildAnchor {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_text_child_anchor_new()) }
    }

    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    #[doc(alias = "gtk_text_child_anchor_new_with_replacement")]
    #[doc(alias = "new_with_replacement")]
    pub fn with_replacement(character: &str) -> TextChildAnchor {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_text_child_anchor_new_with_replacement(
                character.to_glib_none().0,
            ))
        }
    }
}

impl Default for TextChildAnchor {
    fn default() -> Self {
        Self::new()
    }
}

pub trait TextChildAnchorExt: 'static {
    #[doc(alias = "gtk_text_child_anchor_get_deleted")]
    #[doc(alias = "get_deleted")]
    fn is_deleted(&self) -> bool;

    #[doc(alias = "gtk_text_child_anchor_get_widgets")]
    #[doc(alias = "get_widgets")]
    fn widgets(&self) -> Vec<Widget>;
}

impl<O: IsA<TextChildAnchor>> TextChildAnchorExt for O {
    fn is_deleted(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_child_anchor_get_deleted(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn widgets(&self) -> Vec<Widget> {
        unsafe {
            let mut out_len = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_container_num(
                ffi::gtk_text_child_anchor_get_widgets(
                    self.as_ref().to_glib_none().0,
                    out_len.as_mut_ptr(),
                ),
                out_len.assume_init() as _,
            );
            ret
        }
    }
}

impl fmt::Display for TextChildAnchor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TextChildAnchor")
    }
}
