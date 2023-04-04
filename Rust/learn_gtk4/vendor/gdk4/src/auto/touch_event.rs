// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GdkTouchEvent")]
    pub struct TouchEvent(Shared<ffi::GdkTouchEvent>);

    match fn {
        ref => |ptr| ffi::gdk_event_ref(ptr as *mut ffi::GdkEvent),
        unref => |ptr| ffi::gdk_event_unref(ptr as *mut ffi::GdkEvent),
    }
}

impl glib::StaticType for TouchEvent {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gdk_touch_event_get_type()) }
    }
}

impl TouchEvent {
    #[doc(alias = "gdk_touch_event_get_emulating_pointer")]
    #[doc(alias = "get_emulating_pointer")]
    pub fn is_emulating_pointer(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_touch_event_get_emulating_pointer(
                self.to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for TouchEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TouchEvent")
    }
}