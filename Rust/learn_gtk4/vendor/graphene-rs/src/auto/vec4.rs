// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Vec2, Vec3};
use glib::translate::*;

glib::wrapper! {
    pub struct Vec4(BoxedInline<ffi::graphene_vec4_t>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::graphene_vec4_get_type(), ptr as *mut _) as *mut ffi::graphene_vec4_t,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::graphene_vec4_get_type(), ptr as *mut _),
        type_ => || ffi::graphene_vec4_get_type(),
    }
}

impl Vec4 {
    #[doc(alias = "graphene_vec4_add")]
    #[must_use]
    pub fn add(&self, b: &Vec4) -> Vec4 {
        unsafe {
            let mut res = Vec4::uninitialized();
            ffi::graphene_vec4_add(
                self.to_glib_none().0,
                b.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_vec4_divide")]
    #[must_use]
    pub fn divide(&self, b: &Vec4) -> Vec4 {
        unsafe {
            let mut res = Vec4::uninitialized();
            ffi::graphene_vec4_divide(
                self.to_glib_none().0,
                b.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_vec4_dot")]
    pub fn dot(&self, b: &Vec4) -> f32 {
        unsafe { ffi::graphene_vec4_dot(self.to_glib_none().0, b.to_glib_none().0) }
    }

    #[doc(alias = "graphene_vec4_equal")]
    fn equal(&self, v2: &Vec4) -> bool {
        unsafe { ffi::graphene_vec4_equal(self.to_glib_none().0, v2.to_glib_none().0) }
    }

    #[doc(alias = "graphene_vec4_get_w")]
    #[doc(alias = "get_w")]
    pub fn w(&self) -> f32 {
        unsafe { ffi::graphene_vec4_get_w(self.to_glib_none().0) }
    }

    #[doc(alias = "graphene_vec4_get_x")]
    #[doc(alias = "get_x")]
    pub fn x(&self) -> f32 {
        unsafe { ffi::graphene_vec4_get_x(self.to_glib_none().0) }
    }

    #[doc(alias = "graphene_vec4_get_xy")]
    #[doc(alias = "get_xy")]
    pub fn xy(&self) -> Vec2 {
        unsafe {
            let mut res = Vec2::uninitialized();
            ffi::graphene_vec4_get_xy(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    #[doc(alias = "graphene_vec4_get_xyz")]
    #[doc(alias = "get_xyz")]
    pub fn xyz(&self) -> Vec3 {
        unsafe {
            let mut res = Vec3::uninitialized();
            ffi::graphene_vec4_get_xyz(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    #[doc(alias = "graphene_vec4_get_y")]
    #[doc(alias = "get_y")]
    pub fn y(&self) -> f32 {
        unsafe { ffi::graphene_vec4_get_y(self.to_glib_none().0) }
    }

    #[doc(alias = "graphene_vec4_get_z")]
    #[doc(alias = "get_z")]
    pub fn z(&self) -> f32 {
        unsafe { ffi::graphene_vec4_get_z(self.to_glib_none().0) }
    }

    #[doc(alias = "graphene_vec4_interpolate")]
    #[must_use]
    pub fn interpolate(&self, v2: &Vec4, factor: f64) -> Vec4 {
        unsafe {
            let mut res = Vec4::uninitialized();
            ffi::graphene_vec4_interpolate(
                self.to_glib_none().0,
                v2.to_glib_none().0,
                factor,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_vec4_length")]
    pub fn length(&self) -> f32 {
        unsafe { ffi::graphene_vec4_length(self.to_glib_none().0) }
    }

    #[doc(alias = "graphene_vec4_max")]
    #[must_use]
    pub fn max(&self, b: &Vec4) -> Vec4 {
        unsafe {
            let mut res = Vec4::uninitialized();
            ffi::graphene_vec4_max(
                self.to_glib_none().0,
                b.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_vec4_min")]
    #[must_use]
    pub fn min(&self, b: &Vec4) -> Vec4 {
        unsafe {
            let mut res = Vec4::uninitialized();
            ffi::graphene_vec4_min(
                self.to_glib_none().0,
                b.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_vec4_multiply")]
    #[must_use]
    pub fn multiply(&self, b: &Vec4) -> Vec4 {
        unsafe {
            let mut res = Vec4::uninitialized();
            ffi::graphene_vec4_multiply(
                self.to_glib_none().0,
                b.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_vec4_near")]
    pub fn near(&self, v2: &Vec4, epsilon: f32) -> bool {
        unsafe { ffi::graphene_vec4_near(self.to_glib_none().0, v2.to_glib_none().0, epsilon) }
    }

    #[doc(alias = "graphene_vec4_negate")]
    #[must_use]
    pub fn negate(&self) -> Vec4 {
        unsafe {
            let mut res = Vec4::uninitialized();
            ffi::graphene_vec4_negate(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    #[doc(alias = "graphene_vec4_normalize")]
    #[must_use]
    pub fn normalize(&self) -> Vec4 {
        unsafe {
            let mut res = Vec4::uninitialized();
            ffi::graphene_vec4_normalize(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    #[doc(alias = "graphene_vec4_scale")]
    #[must_use]
    pub fn scale(&self, factor: f32) -> Vec4 {
        unsafe {
            let mut res = Vec4::uninitialized();
            ffi::graphene_vec4_scale(self.to_glib_none().0, factor, res.to_glib_none_mut().0);
            res
        }
    }

    #[doc(alias = "graphene_vec4_subtract")]
    #[must_use]
    pub fn subtract(&self, b: &Vec4) -> Vec4 {
        unsafe {
            let mut res = Vec4::uninitialized();
            ffi::graphene_vec4_subtract(
                self.to_glib_none().0,
                b.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    #[doc(alias = "graphene_vec4_one")]
    pub fn one() -> Vec4 {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::graphene_vec4_one()) }
    }

    #[doc(alias = "graphene_vec4_w_axis")]
    pub fn w_axis() -> Vec4 {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::graphene_vec4_w_axis()) }
    }

    #[doc(alias = "graphene_vec4_x_axis")]
    pub fn x_axis() -> Vec4 {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::graphene_vec4_x_axis()) }
    }

    #[doc(alias = "graphene_vec4_y_axis")]
    pub fn y_axis() -> Vec4 {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::graphene_vec4_y_axis()) }
    }

    #[doc(alias = "graphene_vec4_z_axis")]
    pub fn z_axis() -> Vec4 {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::graphene_vec4_z_axis()) }
    }

    #[doc(alias = "graphene_vec4_zero")]
    pub fn zero() -> Vec4 {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::graphene_vec4_zero()) }
    }
}

impl PartialEq for Vec4 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for Vec4 {}