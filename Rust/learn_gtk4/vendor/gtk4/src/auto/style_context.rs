// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{Border, StateFlags, StyleContextPrintFlags, StyleProvider};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkStyleContext")]
    pub struct StyleContext(Object<ffi::GtkStyleContext, ffi::GtkStyleContextClass>);

    match fn {
        type_ => || ffi::gtk_style_context_get_type(),
    }
}

impl StyleContext {
    pub const NONE: Option<&'static StyleContext> = None;

    #[doc(alias = "gtk_style_context_add_provider_for_display")]
    pub fn add_provider_for_display(
        display: &impl IsA<gdk::Display>,
        provider: &impl IsA<StyleProvider>,
        priority: u32,
    ) {
        skip_assert_initialized!();
        unsafe {
            ffi::gtk_style_context_add_provider_for_display(
                display.as_ref().to_glib_none().0,
                provider.as_ref().to_glib_none().0,
                priority,
            );
        }
    }

    #[doc(alias = "gtk_style_context_remove_provider_for_display")]
    pub fn remove_provider_for_display(
        display: &impl IsA<gdk::Display>,
        provider: &impl IsA<StyleProvider>,
    ) {
        skip_assert_initialized!();
        unsafe {
            ffi::gtk_style_context_remove_provider_for_display(
                display.as_ref().to_glib_none().0,
                provider.as_ref().to_glib_none().0,
            );
        }
    }
}

pub trait StyleContextExt: 'static {
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_style_context_add_class")]
    fn add_class(&self, class_name: &str);

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_style_context_add_provider")]
    fn add_provider(&self, provider: &impl IsA<StyleProvider>, priority: u32);

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_style_context_get_border")]
    #[doc(alias = "get_border")]
    fn border(&self) -> Border;

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_style_context_get_color")]
    #[doc(alias = "get_color")]
    fn color(&self) -> gdk::RGBA;

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_style_context_get_display")]
    #[doc(alias = "get_display")]
    fn display(&self) -> gdk::Display;

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_style_context_get_margin")]
    #[doc(alias = "get_margin")]
    fn margin(&self) -> Border;

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_style_context_get_padding")]
    #[doc(alias = "get_padding")]
    fn padding(&self) -> Border;

    #[doc(alias = "gtk_style_context_get_scale")]
    #[doc(alias = "get_scale")]
    fn scale(&self) -> i32;

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_style_context_get_state")]
    #[doc(alias = "get_state")]
    fn state(&self) -> StateFlags;

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_style_context_has_class")]
    fn has_class(&self, class_name: &str) -> bool;

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_style_context_lookup_color")]
    fn lookup_color(&self, color_name: &str) -> Option<gdk::RGBA>;

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_style_context_remove_class")]
    fn remove_class(&self, class_name: &str);

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_style_context_remove_provider")]
    fn remove_provider(&self, provider: &impl IsA<StyleProvider>);

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_style_context_restore")]
    fn restore(&self);

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_style_context_save")]
    fn save(&self);

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_style_context_set_display")]
    fn set_display(&self, display: &impl IsA<gdk::Display>);

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_style_context_set_scale")]
    fn set_scale(&self, scale: i32);

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_style_context_set_state")]
    fn set_state(&self, flags: StateFlags);

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_style_context_to_string")]
    fn to_string(&self, flags: StyleContextPrintFlags) -> glib::GString;

    #[doc(alias = "display")]
    fn connect_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<StyleContext>> StyleContextExt for O {
    #[allow(deprecated)]
    fn add_class(&self, class_name: &str) {
        unsafe {
            ffi::gtk_style_context_add_class(
                self.as_ref().to_glib_none().0,
                class_name.to_glib_none().0,
            );
        }
    }

    #[allow(deprecated)]
    fn add_provider(&self, provider: &impl IsA<StyleProvider>, priority: u32) {
        unsafe {
            ffi::gtk_style_context_add_provider(
                self.as_ref().to_glib_none().0,
                provider.as_ref().to_glib_none().0,
                priority,
            );
        }
    }

    #[allow(deprecated)]
    fn border(&self) -> Border {
        unsafe {
            let mut border = Border::uninitialized();
            ffi::gtk_style_context_get_border(
                self.as_ref().to_glib_none().0,
                border.to_glib_none_mut().0,
            );
            border
        }
    }

    #[allow(deprecated)]
    fn color(&self) -> gdk::RGBA {
        unsafe {
            let mut color = gdk::RGBA::uninitialized();
            ffi::gtk_style_context_get_color(
                self.as_ref().to_glib_none().0,
                color.to_glib_none_mut().0,
            );
            color
        }
    }

    #[allow(deprecated)]
    fn display(&self) -> gdk::Display {
        unsafe {
            from_glib_none(ffi::gtk_style_context_get_display(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[allow(deprecated)]
    fn margin(&self) -> Border {
        unsafe {
            let mut margin = Border::uninitialized();
            ffi::gtk_style_context_get_margin(
                self.as_ref().to_glib_none().0,
                margin.to_glib_none_mut().0,
            );
            margin
        }
    }

    #[allow(deprecated)]
    fn padding(&self) -> Border {
        unsafe {
            let mut padding = Border::uninitialized();
            ffi::gtk_style_context_get_padding(
                self.as_ref().to_glib_none().0,
                padding.to_glib_none_mut().0,
            );
            padding
        }
    }

    fn scale(&self) -> i32 {
        unsafe { ffi::gtk_style_context_get_scale(self.as_ref().to_glib_none().0) }
    }

    #[allow(deprecated)]
    fn state(&self) -> StateFlags {
        unsafe {
            from_glib(ffi::gtk_style_context_get_state(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[allow(deprecated)]
    fn has_class(&self, class_name: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_style_context_has_class(
                self.as_ref().to_glib_none().0,
                class_name.to_glib_none().0,
            ))
        }
    }

    #[allow(deprecated)]
    fn lookup_color(&self, color_name: &str) -> Option<gdk::RGBA> {
        unsafe {
            let mut color = gdk::RGBA::uninitialized();
            let ret = from_glib(ffi::gtk_style_context_lookup_color(
                self.as_ref().to_glib_none().0,
                color_name.to_glib_none().0,
                color.to_glib_none_mut().0,
            ));
            if ret {
                Some(color)
            } else {
                None
            }
        }
    }

    #[allow(deprecated)]
    fn remove_class(&self, class_name: &str) {
        unsafe {
            ffi::gtk_style_context_remove_class(
                self.as_ref().to_glib_none().0,
                class_name.to_glib_none().0,
            );
        }
    }

    #[allow(deprecated)]
    fn remove_provider(&self, provider: &impl IsA<StyleProvider>) {
        unsafe {
            ffi::gtk_style_context_remove_provider(
                self.as_ref().to_glib_none().0,
                provider.as_ref().to_glib_none().0,
            );
        }
    }

    #[allow(deprecated)]
    fn restore(&self) {
        unsafe {
            ffi::gtk_style_context_restore(self.as_ref().to_glib_none().0);
        }
    }

    #[allow(deprecated)]
    fn save(&self) {
        unsafe {
            ffi::gtk_style_context_save(self.as_ref().to_glib_none().0);
        }
    }

    #[allow(deprecated)]
    fn set_display(&self, display: &impl IsA<gdk::Display>) {
        unsafe {
            ffi::gtk_style_context_set_display(
                self.as_ref().to_glib_none().0,
                display.as_ref().to_glib_none().0,
            );
        }
    }

    #[allow(deprecated)]
    fn set_scale(&self, scale: i32) {
        unsafe {
            ffi::gtk_style_context_set_scale(self.as_ref().to_glib_none().0, scale);
        }
    }

    #[allow(deprecated)]
    fn set_state(&self, flags: StateFlags) {
        unsafe {
            ffi::gtk_style_context_set_state(self.as_ref().to_glib_none().0, flags.into_glib());
        }
    }

    #[allow(deprecated)]
    fn to_string(&self, flags: StyleContextPrintFlags) -> glib::GString {
        unsafe {
            from_glib_full(ffi::gtk_style_context_to_string(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
            ))
        }
    }

    fn connect_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_display_trampoline<
            P: IsA<StyleContext>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkStyleContext,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(StyleContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::display\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_display_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for StyleContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("StyleContext")
    }
}
