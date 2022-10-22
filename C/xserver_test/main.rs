use std::{ffi::*, ptr::*};

#[repr(C)]
pub struct _XDisplay {}

extern "C" {
    pub fn XOpenDisplay(display_name: *const c_char) -> *mut _XDisplay;
}

fn main() {
    let display = XOpenDisplay(null_mut::<*const c_char>);
}
