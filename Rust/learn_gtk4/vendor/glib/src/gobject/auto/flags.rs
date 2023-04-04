// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{translate::*, value::FromValue, value::ToValue, StaticType, Type};
use bitflags::bitflags;
use std::fmt;

bitflags! {
    #[doc(alias = "GBindingFlags")]
    pub struct BindingFlags: u32 {
        #[doc(alias = "G_BINDING_DEFAULT")]
        const DEFAULT = gobject_ffi::G_BINDING_DEFAULT as _;
        #[doc(alias = "G_BINDING_BIDIRECTIONAL")]
        const BIDIRECTIONAL = gobject_ffi::G_BINDING_BIDIRECTIONAL as _;
        #[doc(alias = "G_BINDING_SYNC_CREATE")]
        const SYNC_CREATE = gobject_ffi::G_BINDING_SYNC_CREATE as _;
        #[doc(alias = "G_BINDING_INVERT_BOOLEAN")]
        const INVERT_BOOLEAN = gobject_ffi::G_BINDING_INVERT_BOOLEAN as _;
    }
}

impl fmt::Display for BindingFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for BindingFlags {
    type GlibType = gobject_ffi::GBindingFlags;

    #[inline]
    fn into_glib(self) -> gobject_ffi::GBindingFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gobject_ffi::GBindingFlags> for BindingFlags {
    #[inline]
    unsafe fn from_glib(value: gobject_ffi::GBindingFlags) -> Self {
        Self::from_bits_truncate(value)
    }
}

impl StaticType for BindingFlags {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(gobject_ffi::g_binding_flags_get_type()) }
    }
}

impl crate::HasParamSpec for BindingFlags {
    type ParamSpec = crate::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> crate::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        |name| Self::ParamSpec::builder(name)
    }
}

impl crate::value::ValueType for BindingFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for BindingFlags {
    type Checker = crate::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a crate::Value) -> Self {
        from_glib(crate::gobject_ffi::g_value_get_flags(
            value.to_glib_none().0,
        ))
    }
}

impl ToValue for BindingFlags {
    #[inline]
    fn to_value(&self) -> crate::Value {
        let mut value = crate::Value::for_value_type::<Self>();
        unsafe {
            crate::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> crate::Type {
        Self::static_type()
    }
}

impl From<BindingFlags> for crate::Value {
    #[inline]
    fn from(v: BindingFlags) -> Self {
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[doc(alias = "GSignalFlags")]
    pub struct SignalFlags: u32 {
        #[doc(alias = "G_SIGNAL_RUN_FIRST")]
        const RUN_FIRST = gobject_ffi::G_SIGNAL_RUN_FIRST as _;
        #[doc(alias = "G_SIGNAL_RUN_LAST")]
        const RUN_LAST = gobject_ffi::G_SIGNAL_RUN_LAST as _;
        #[doc(alias = "G_SIGNAL_RUN_CLEANUP")]
        const RUN_CLEANUP = gobject_ffi::G_SIGNAL_RUN_CLEANUP as _;
        #[doc(alias = "G_SIGNAL_NO_RECURSE")]
        const NO_RECURSE = gobject_ffi::G_SIGNAL_NO_RECURSE as _;
        #[doc(alias = "G_SIGNAL_DETAILED")]
        const DETAILED = gobject_ffi::G_SIGNAL_DETAILED as _;
        #[doc(alias = "G_SIGNAL_ACTION")]
        const ACTION = gobject_ffi::G_SIGNAL_ACTION as _;
        #[doc(alias = "G_SIGNAL_NO_HOOKS")]
        const NO_HOOKS = gobject_ffi::G_SIGNAL_NO_HOOKS as _;
        #[doc(alias = "G_SIGNAL_MUST_COLLECT")]
        const MUST_COLLECT = gobject_ffi::G_SIGNAL_MUST_COLLECT as _;
        #[doc(alias = "G_SIGNAL_DEPRECATED")]
        const DEPRECATED = gobject_ffi::G_SIGNAL_DEPRECATED as _;
        #[doc(alias = "G_SIGNAL_ACCUMULATOR_FIRST_RUN")]
        const ACCUMULATOR_FIRST_RUN = gobject_ffi::G_SIGNAL_ACCUMULATOR_FIRST_RUN as _;
    }
}

impl fmt::Display for SignalFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for SignalFlags {
    type GlibType = gobject_ffi::GSignalFlags;

    #[inline]
    fn into_glib(self) -> gobject_ffi::GSignalFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gobject_ffi::GSignalFlags> for SignalFlags {
    #[inline]
    unsafe fn from_glib(value: gobject_ffi::GSignalFlags) -> Self {
        Self::from_bits_truncate(value)
    }
}