// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GDatagramBased")]
    pub struct DatagramBased(Interface<ffi::GDatagramBased, ffi::GDatagramBasedInterface>);

    match fn {
        type_ => || ffi::g_datagram_based_get_type(),
    }
}

impl DatagramBased {
    pub const NONE: Option<&'static DatagramBased> = None;
}

pub trait DatagramBasedExt: 'static {
    #[doc(alias = "g_datagram_based_condition_check")]
    fn condition_check(&self, condition: glib::IOCondition) -> glib::IOCondition;
}

impl<O: IsA<DatagramBased>> DatagramBasedExt for O {
    fn condition_check(&self, condition: glib::IOCondition) -> glib::IOCondition {
        unsafe {
            from_glib(ffi::g_datagram_based_condition_check(
                self.as_ref().to_glib_none().0,
                condition.into_glib(),
            ))
        }
    }
}

impl fmt::Display for DatagramBased {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DatagramBased")
    }
}
