// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(any(feature = "v2_66", feature = "dox"))]
use std::ptr;

use glib::prelude::*;
#[cfg(any(feature = "v2_60", feature = "dox"))]
use glib::{translate::*, IntoStrV};

#[cfg(any(feature = "v2_66", feature = "dox"))]
use crate::TlsChannelBindingType;
use crate::TlsConnection;

pub trait TlsConnectionExtManual {
    #[cfg(any(feature = "v2_66", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
    #[doc(alias = "g_tls_connection_get_channel_binding_data")]
    #[doc(alias = "get_channel_binding_data")]
    fn channel_binding_data(
        &self,
        type_: TlsChannelBindingType,
    ) -> Result<glib::ByteArray, glib::Error>;

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    #[doc(alias = "g_tls_connection_set_advertised_protocols")]
    fn set_advertised_protocols(&self, protocols: impl IntoStrV);
}

impl<O: IsA<TlsConnection>> TlsConnectionExtManual for O {
    #[cfg(any(feature = "v2_66", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
    fn channel_binding_data(
        &self,
        type_: TlsChannelBindingType,
    ) -> Result<glib::ByteArray, glib::Error> {
        unsafe {
            let data = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = ffi::g_tls_connection_get_channel_binding_data(
                self.as_ptr() as *mut _,
                type_.into_glib(),
                data,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_none(data))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    fn set_advertised_protocols(&self, protocols: impl IntoStrV) {
        unsafe {
            protocols.run_with_strv(|protocols| {
                ffi::g_tls_connection_set_advertised_protocols(
                    self.as_ref().to_glib_none().0,
                    protocols.as_ptr() as *mut _,
                );
            })
        }
    }
}