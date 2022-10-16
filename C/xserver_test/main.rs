use std::{ffi::*, ptr::*};

#[repr(C)]
pub struct _XDisplay {}

extern "C" {
    pub fn XOpendisplay(display_name: *const c_char) -> *mut _XDisplay;
}

fn main() {
    let display = XOpendisplay(null_mut::<*const c_char>);
}
