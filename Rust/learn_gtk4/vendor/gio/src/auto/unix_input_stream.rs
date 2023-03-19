// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{FileDescriptorBased, InputStream, PollableInputStream};
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GUnixInputStream")]
    pub struct UnixInputStream(Object<ffi::GUnixInputStream, ffi::GUnixInputStreamClass>) @extends InputStream, @implements FileDescriptorBased, PollableInputStream;

    match fn {
        type_ => || ffi::g_unix_input_stream_get_type(),
    }
}

impl UnixInputStream {
    pub const NONE: Option<&'static UnixInputStream> = None;
}

pub trait UnixInputStreamExt: 'static {
    #[doc(alias = "g_unix_input_stream_get_close_fd")]
    #[doc(alias = "get_close_fd")]
    fn closes_fd(&self) -> bool;
}

impl<O: IsA<UnixInputStream>> UnixInputStreamExt for O {
    fn closes_fd(&self) -> bool {
        unsafe {
            from_glib(ffi::g_unix_input_stream_get_close_fd(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for UnixInputStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("UnixInputStream")
    }
}
