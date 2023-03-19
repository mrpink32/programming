// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
use crate::AttrIterator;
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Item(Boxed<ffi::PangoItem>);

    match fn {
        copy => |ptr| ffi::pango_item_copy(mut_override(ptr)),
        free => |ptr| ffi::pango_item_free(ptr),
        type_ => || ffi::pango_item_get_type(),
    }
}

impl Item {
    #[doc(alias = "pango_item_new")]
    pub fn new() -> Item {
        unsafe { from_glib_full(ffi::pango_item_new()) }
    }

    #[cfg(any(feature = "v1_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
    #[doc(alias = "pango_item_apply_attrs")]
    pub fn apply_attrs(&mut self, iter: &mut AttrIterator) {
        unsafe {
            ffi::pango_item_apply_attrs(self.to_glib_none_mut().0, iter.to_glib_none_mut().0);
        }
    }

    #[doc(alias = "pango_item_split")]
    #[must_use]
    pub fn split(&mut self, split_index: i32, split_offset: i32) -> Item {
        unsafe {
            from_glib_full(ffi::pango_item_split(
                self.to_glib_none_mut().0,
                split_index,
                split_offset,
            ))
        }
    }
}

impl Default for Item {
    fn default() -> Self {
        Self::new()
    }
}
