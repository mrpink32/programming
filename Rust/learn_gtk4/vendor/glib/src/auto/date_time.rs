// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{translate::*, BoolError, TimeSpan, TimeZone};
use std::{cmp, hash, mem};

crate::wrapper! {
    #[derive(Debug)]
    pub struct DateTime(Shared<ffi::GDateTime>);

    match fn {
        ref => |ptr| ffi::g_date_time_ref(ptr),
        unref => |ptr| ffi::g_date_time_unref(ptr),
        type_ => || ffi::g_date_time_get_type(),
    }
}

impl DateTime {
    #[doc(alias = "g_date_time_new")]
    pub fn new(
        tz: &TimeZone,
        year: i32,
        month: i32,
        day: i32,
        hour: i32,
        minute: i32,
        seconds: f64,
    ) -> Result<DateTime, BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::g_date_time_new(
                tz.to_glib_none().0,
                year,
                month,
                day,
                hour,
                minute,
                seconds,
            ))
            .ok_or_else(|| crate::bool_error!("Invalid date"))
        }
    }

    #[doc(alias = "g_date_time_new_from_iso8601")]
    #[doc(alias = "new_from_iso8601")]
    pub fn from_iso8601(text: &str, default_tz: Option<&TimeZone>) -> Result<DateTime, BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::g_date_time_new_from_iso8601(
                text.to_glib_none().0,
                default_tz.to_glib_none().0,
            ))
            .ok_or_else(|| crate::bool_error!("Invalid date"))
        }
    }

    //#[cfg_attr(feature = "v2_62", deprecated = "Since 2.62")]
    //#[allow(deprecated)]
    //#[doc(alias = "g_date_time_new_from_timeval_local")]
    //#[doc(alias = "new_from_timeval_local")]
    //pub fn from_timeval_local(tv: /*Ignored*/&TimeVal) -> Result<DateTime, BoolError> {
    //    unsafe { TODO: call ffi:g_date_time_new_from_timeval_local() }
    //}

    //#[cfg_attr(feature = "v2_62", deprecated = "Since 2.62")]
    //#[allow(deprecated)]
    //#[doc(alias = "g_date_time_new_from_timeval_utc")]
    //#[doc(alias = "new_from_timeval_utc")]
    //pub fn from_timeval_utc(tv: /*Ignored*/&TimeVal) -> Result<DateTime, BoolError> {
    //    unsafe { TODO: call ffi:g_date_time_new_from_timeval_utc() }
    //}

    #[doc(alias = "g_date_time_new_from_unix_local")]
    #[doc(alias = "new_from_unix_local")]
    pub fn from_unix_local(t: i64) -> Result<DateTime, BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::g_date_time_new_from_unix_local(t))
                .ok_or_else(|| crate::bool_error!("Invalid date"))
        }
    }

    #[doc(alias = "g_date_time_new_from_unix_utc")]
    #[doc(alias = "new_from_unix_utc")]
    pub fn from_unix_utc(t: i64) -> Result<DateTime, BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::g_date_time_new_from_unix_utc(t))
                .ok_or_else(|| crate::bool_error!("Invalid date"))
        }
    }

    #[doc(alias = "g_date_time_new_local")]
    #[doc(alias = "new_local")]
    pub fn from_local(
        year: i32,
        month: i32,
        day: i32,
        hour: i32,
        minute: i32,
        seconds: f64,
    ) -> Result<DateTime, BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::g_date_time_new_local(
                year, month, day, hour, minute, seconds,
            ))
            .ok_or_else(|| crate::bool_error!("Invalid date"))
        }
    }

    #[doc(alias = "g_date_time_new_now")]
    #[doc(alias = "new_now")]
    pub fn now(tz: &TimeZone) -> Result<DateTime, BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::g_date_time_new_now(tz.to_glib_none().0))
                .ok_or_else(|| crate::bool_error!("Invalid date"))
        }
    }

    #[doc(alias = "g_date_time_new_now_local")]
    #[doc(alias = "new_now_local")]
    pub fn now_local() -> Result<DateTime, BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::g_date_time_new_now_local())
                .ok_or_else(|| crate::bool_error!("Invalid date"))
        }
    }

    #[doc(alias = "g_date_time_new_now_utc")]
    #[doc(alias = "new_now_utc")]
    pub fn now_utc() -> Result<DateTime, BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::g_date_time_new_now_utc())
                .ok_or_else(|| crate::bool_error!("Invalid date"))
        }
    }

    #[doc(alias = "g_date_time_new_utc")]
    #[doc(alias = "new_utc")]
    pub fn from_utc(
        year: i32,
        month: i32,
        day: i32,
        hour: i32,
        minute: i32,
        seconds: f64,
    ) -> Result<DateTime, BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::g_date_time_new_utc(
                year, month, day, hour, minute, seconds,
            ))
            .ok_or_else(|| crate::bool_error!("Invalid date"))
        }
    }

    #[doc(alias = "g_date_time_add")]
    pub fn add(&self, timespan: TimeSpan) -> Result<DateTime, BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::g_date_time_add(
                self.to_glib_none().0,
                timespan.into_glib(),
            ))
            .ok_or_else(|| crate::bool_error!("Invalid date"))
        }
    }

    #[doc(alias = "g_date_time_add_days")]
    pub fn add_days(&self, days: i32) -> Result<DateTime, BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::g_date_time_add_days(self.to_glib_none().0, days))
                .ok_or_else(|| crate::bool_error!("Invalid date"))
        }
    }

    #[doc(alias = "g_date_time_add_full")]
    pub fn add_full(
        &self,
        years: i32,
        months: i32,
        days: i32,
        hours: i32,
        minutes: i32,
        seconds: f64,
    ) -> Result<DateTime, BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::g_date_time_add_full(
                self.to_glib_none().0,
                years,
                months,
                days,
                hours,
                minutes,
                seconds,
            ))
            .ok_or_else(|| crate::bool_error!("Invalid date"))
        }
    }

    #[doc(alias = "g_date_time_add_hours")]
    pub fn add_hours(&self, hours: i32) -> Result<DateTime, BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::g_date_time_add_hours(self.to_glib_none().0, hours))
                .ok_or_else(|| crate::bool_error!("Invalid date"))
        }
    }

    #[doc(alias = "g_date_time_add_minutes")]
    pub fn add_minutes(&self, minutes: i32) -> Result<DateTime, BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::g_date_time_add_minutes(
                self.to_glib_none().0,
                minutes,
            ))
            .ok_or_else(|| crate::bool_error!("Invalid date"))
        }
    }

    #[doc(alias = "g_date_time_add_months")]
    pub fn add_months(&self, months: i32) -> Result<DateTime, BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::g_date_time_add_months(self.to_glib_none().0, months))
                .ok_or_else(|| crate::bool_error!("Invalid date"))
        }
    }

    #[doc(alias = "g_date_time_add_seconds")]
    pub fn add_seconds(&self, seconds: f64) -> Result<DateTime, BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::g_date_time_add_seconds(
                self.to_glib_none().0,
                seconds,
            ))
            .ok_or_else(|| crate::bool_error!("Invalid date"))
        }
    }

    #[doc(alias = "g_date_time_add_weeks")]
    pub fn add_weeks(&self, weeks: i32) -> Result<DateTime, BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::g_date_time_add_weeks(self.to_glib_none().0, weeks))
                .ok_or_else(|| crate::bool_error!("Invalid date"))
        }
    }

    #[doc(alias = "g_date_time_add_years")]
    pub fn add_years(&self, years: i32) -> Result<DateTime, BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::g_date_time_add_years(self.to_glib_none().0, years))
                .ok_or_else(|| crate::bool_error!("Invalid date"))
        }
    }

    #[doc(alias = "g_date_time_compare")]
    fn compare(&self, dt2: &DateTime) -> i32 {
        unsafe {
            ffi::g_date_time_compare(
                ToGlibPtr::<*mut ffi::GDateTime>::to_glib_none(self).0 as ffi::gconstpointer,
                ToGlibPtr::<*mut ffi::GDateTime>::to_glib_none(dt2).0 as ffi::gconstpointer,
            )
        }
    }

    #[doc(alias = "g_date_time_difference")]
    pub fn difference(&self, begin: &DateTime) -> TimeSpan {
        unsafe {
            from_glib(ffi::g_date_time_difference(
                self.to_glib_none().0,
                begin.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_date_time_equal")]
    fn equal(&self, dt2: &DateTime) -> bool {
        unsafe {
            from_glib(ffi::g_date_time_equal(
                ToGlibPtr::<*mut ffi::GDateTime>::to_glib_none(self).0 as ffi::gconstpointer,
                ToGlibPtr::<*mut ffi::GDateTime>::to_glib_none(dt2).0 as ffi::gconstpointer,
            ))
        }
    }

    #[doc(alias = "g_date_time_format")]
    pub fn format(&self, format: &str) -> Result<crate::GString, BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::g_date_time_format(
                self.to_glib_none().0,
                format.to_glib_none().0,
            ))
            .ok_or_else(|| crate::bool_error!("Invalid date"))
        }
    }

    #[cfg(any(feature = "v2_62", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_62")))]
    #[doc(alias = "g_date_time_format_iso8601")]
    pub fn format_iso8601(&self) -> Result<crate::GString, BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::g_date_time_format_iso8601(self.to_glib_none().0))
                .ok_or_else(|| crate::bool_error!("Invalid date"))
        }
    }

    #[doc(alias = "g_date_time_get_day_of_month")]
    #[doc(alias = "get_day_of_month")]
    pub fn day_of_month(&self) -> i32 {
        unsafe { ffi::g_date_time_get_day_of_month(self.to_glib_none().0) }
    }

    #[doc(alias = "g_date_time_get_day_of_week")]
    #[doc(alias = "get_day_of_week")]
    pub fn day_of_week(&self) -> i32 {
        unsafe { ffi::g_date_time_get_day_of_week(self.to_glib_none().0) }
    }

    #[doc(alias = "g_date_time_get_day_of_year")]
    #[doc(alias = "get_day_of_year")]
    pub fn day_of_year(&self) -> i32 {
        unsafe { ffi::g_date_time_get_day_of_year(self.to_glib_none().0) }
    }

    #[doc(alias = "g_date_time_get_hour")]
    #[doc(alias = "get_hour")]
    pub fn hour(&self) -> i32 {
        unsafe { ffi::g_date_time_get_hour(self.to_glib_none().0) }
    }

    #[doc(alias = "g_date_time_get_microsecond")]
    #[doc(alias = "get_microsecond")]
    pub fn microsecond(&self) -> i32 {
        unsafe { ffi::g_date_time_get_microsecond(self.to_glib_none().0) }
    }

    #[doc(alias = "g_date_time_get_minute")]
    #[doc(alias = "get_minute")]
    pub fn minute(&self) -> i32 {
        unsafe { ffi::g_date_time_get_minute(self.to_glib_none().0) }
    }

    #[doc(alias = "g_date_time_get_month")]
    #[doc(alias = "get_month")]
    pub fn month(&self) -> i32 {
        unsafe { ffi::g_date_time_get_month(self.to_glib_none().0) }
    }

    #[doc(alias = "g_date_time_get_second")]
    #[doc(alias = "get_second")]
    pub fn second(&self) -> i32 {
        unsafe { ffi::g_date_time_get_second(self.to_glib_none().0) }
    }

    #[doc(alias = "g_date_time_get_seconds")]
    #[doc(alias = "get_seconds")]
    pub fn seconds(&self) -> f64 {
        unsafe { ffi::g_date_time_get_seconds(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_58")))]
    #[doc(alias = "g_date_time_get_timezone")]
    #[doc(alias = "get_timezone")]
    pub fn timezone(&self) -> TimeZone {
        unsafe { from_glib_none(ffi::g_date_time_get_timezone(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_date_time_get_timezone_abbreviation")]
    #[doc(alias = "get_timezone_abbreviation")]
    pub fn timezone_abbreviation(&self) -> crate::GString {
        unsafe {
            from_glib_none(ffi::g_date_time_get_timezone_abbreviation(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_date_time_get_utc_offset")]
    #[doc(alias = "get_utc_offset")]
    pub fn utc_offset(&self) -> TimeSpan {
        unsafe { from_glib(ffi::g_date_time_get_utc_offset(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_date_time_get_week_numbering_year")]
    #[doc(alias = "get_week_numbering_year")]
    pub fn week_numbering_year(&self) -> i32 {
        unsafe { ffi::g_date_time_get_week_numbering_year(self.to_glib_none().0) }
    }

    #[doc(alias = "g_date_time_get_week_of_year")]
    #[doc(alias = "get_week_of_year")]
    pub fn week_of_year(&self) -> i32 {
        unsafe { ffi::g_date_time_get_week_of_year(self.to_glib_none().0) }
    }

    #[doc(alias = "g_date_time_get_year")]
    #[doc(alias = "get_year")]
    pub fn year(&self) -> i32 {
        unsafe { ffi::g_date_time_get_year(self.to_glib_none().0) }
    }

    #[doc(alias = "g_date_time_get_ymd")]
    #[doc(alias = "get_ymd")]
    pub fn ymd(&self) -> (i32, i32, i32) {
        unsafe {
            let mut year = mem::MaybeUninit::uninit();
            let mut month = mem::MaybeUninit::uninit();
            let mut day = mem::MaybeUninit::uninit();
            ffi::g_date_time_get_ymd(
                self.to_glib_none().0,
                year.as_mut_ptr(),
                month.as_mut_ptr(),
                day.as_mut_ptr(),
            );
            (year.assume_init(), month.assume_init(), day.assume_init())
        }
    }

    #[doc(alias = "g_date_time_hash")]
    fn hash(&self) -> u32 {
        unsafe {
            ffi::g_date_time_hash(
                ToGlibPtr::<*mut ffi::GDateTime>::to_glib_none(self).0 as ffi::gconstpointer,
            )
        }
    }

    #[doc(alias = "g_date_time_is_daylight_savings")]
    pub fn is_daylight_savings(&self) -> bool {
        unsafe { from_glib(ffi::g_date_time_is_daylight_savings(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_date_time_to_local")]
    pub fn to_local(&self) -> Result<DateTime, BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::g_date_time_to_local(self.to_glib_none().0))
                .ok_or_else(|| crate::bool_error!("Invalid date"))
        }
    }

    //#[cfg_attr(feature = "v2_62", deprecated = "Since 2.62")]
    //#[allow(deprecated)]
    //#[doc(alias = "g_date_time_to_timeval")]
    //pub fn to_timeval(&self, tv: /*Ignored*/&mut TimeVal) -> bool {
    //    unsafe { TODO: call ffi:g_date_time_to_timeval() }
    //}

    #[doc(alias = "g_date_time_to_timezone")]
    pub fn to_timezone(&self, tz: &TimeZone) -> Result<DateTime, BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::g_date_time_to_timezone(
                self.to_glib_none().0,
                tz.to_glib_none().0,
            ))
            .ok_or_else(|| crate::bool_error!("Invalid date"))
        }
    }

    #[doc(alias = "g_date_time_to_unix")]
    pub fn to_unix(&self) -> i64 {
        unsafe { ffi::g_date_time_to_unix(self.to_glib_none().0) }
    }

    #[doc(alias = "g_date_time_to_utc")]
    pub fn to_utc(&self) -> Result<DateTime, BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::g_date_time_to_utc(self.to_glib_none().0))
                .ok_or_else(|| crate::bool_error!("Invalid date"))
        }
    }
}

impl PartialOrd for DateTime {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.compare(other).partial_cmp(&0)
    }
}

impl Ord for DateTime {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.compare(other).cmp(&0)
    }
}

impl PartialEq for DateTime {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for DateTime {}

impl hash::Hash for DateTime {
    #[inline]
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        hash::Hash::hash(&self.hash(), state)
    }
}

unsafe impl Send for DateTime {}
unsafe impl Sync for DateTime {}