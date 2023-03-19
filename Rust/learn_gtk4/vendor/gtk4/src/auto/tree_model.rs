// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{TreeIter, TreeModelFlags, TreePath};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkTreeModel")]
    pub struct TreeModel(Interface<ffi::GtkTreeModel, ffi::GtkTreeModelIface>);

    match fn {
        type_ => || ffi::gtk_tree_model_get_type(),
    }
}

impl TreeModel {
    pub const NONE: Option<&'static TreeModel> = None;
}

pub trait TreeModelExt: 'static {
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_model_foreach")]
    fn foreach<P: FnMut(&TreeModel, &TreePath, &TreeIter) -> bool>(&self, func: P);

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_model_get_column_type")]
    #[doc(alias = "get_column_type")]
    fn column_type(&self, index_: i32) -> glib::types::Type;

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_model_get_flags")]
    #[doc(alias = "get_flags")]
    fn flags(&self) -> TreeModelFlags;

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_model_get_iter")]
    #[doc(alias = "get_iter")]
    fn iter(&self, path: &TreePath) -> Option<TreeIter>;

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_model_get_iter_first")]
    #[doc(alias = "get_iter_first")]
    fn iter_first(&self) -> Option<TreeIter>;

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_model_get_iter_from_string")]
    #[doc(alias = "get_iter_from_string")]
    fn iter_from_string(&self, path_string: &str) -> Option<TreeIter>;

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_model_get_n_columns")]
    #[doc(alias = "get_n_columns")]
    fn n_columns(&self) -> i32;

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_model_get_path")]
    #[doc(alias = "get_path")]
    fn path(&self, iter: &TreeIter) -> TreePath;

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_model_get_string_from_iter")]
    #[doc(alias = "get_string_from_iter")]
    fn string_from_iter(&self, iter: &TreeIter) -> Option<glib::GString>;

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_model_iter_children")]
    fn iter_children(&self, parent: Option<&TreeIter>) -> Option<TreeIter>;

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_model_iter_has_child")]
    fn iter_has_child(&self, iter: &TreeIter) -> bool;

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_model_iter_n_children")]
    fn iter_n_children(&self, iter: Option<&TreeIter>) -> i32;

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_model_iter_next")]
    fn iter_next(&self, iter: &TreeIter) -> bool;

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_model_iter_nth_child")]
    fn iter_nth_child(&self, parent: Option<&TreeIter>, n: i32) -> Option<TreeIter>;

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_model_iter_parent")]
    fn iter_parent(&self, child: &TreeIter) -> Option<TreeIter>;

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_model_iter_previous")]
    fn iter_previous(&self, iter: &TreeIter) -> bool;

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_model_row_changed")]
    fn row_changed(&self, path: &TreePath, iter: &TreeIter);

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_model_row_deleted")]
    fn row_deleted(&self, path: &TreePath);

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_model_row_has_child_toggled")]
    fn row_has_child_toggled(&self, path: &TreePath, iter: &TreeIter);

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_tree_model_row_inserted")]
    fn row_inserted(&self, path: &TreePath, iter: &TreeIter);

    #[doc(alias = "row-changed")]
    fn connect_row_changed<F: Fn(&Self, &TreePath, &TreeIter) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "row-deleted")]
    fn connect_row_deleted<F: Fn(&Self, &TreePath) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "row-has-child-toggled")]
    fn connect_row_has_child_toggled<F: Fn(&Self, &TreePath, &TreeIter) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "row-inserted")]
    fn connect_row_inserted<F: Fn(&Self, &TreePath, &TreeIter) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    //#[doc(alias = "rows-reordered")]
    //fn connect_rows_reordered<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TreeModel>> TreeModelExt for O {
    #[allow(deprecated)]
    fn foreach<P: FnMut(&TreeModel, &TreePath, &TreeIter) -> bool>(&self, func: P) {
        let func_data: P = func;
        unsafe extern "C" fn func_func<P: FnMut(&TreeModel, &TreePath, &TreeIter) -> bool>(
            model: *mut ffi::GtkTreeModel,
            path: *mut ffi::GtkTreePath,
            iter: *mut ffi::GtkTreeIter,
            data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let model = from_glib_borrow(model);
            let path = from_glib_borrow(path);
            let iter = from_glib_borrow(iter);
            let callback: *mut P = data as *const _ as usize as *mut P;
            (*callback)(&model, &path, &iter).into_glib()
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &P = &func_data;
        unsafe {
            ffi::gtk_tree_model_foreach(
                self.as_ref().to_glib_none().0,
                func,
                super_callback0 as *const _ as usize as *mut _,
            );
        }
    }

    #[allow(deprecated)]
    fn column_type(&self, index_: i32) -> glib::types::Type {
        unsafe {
            from_glib(ffi::gtk_tree_model_get_column_type(
                self.as_ref().to_glib_none().0,
                index_,
            ))
        }
    }

    #[allow(deprecated)]
    fn flags(&self) -> TreeModelFlags {
        unsafe {
            from_glib(ffi::gtk_tree_model_get_flags(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[allow(deprecated)]
    fn iter(&self, path: &TreePath) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_tree_model_get_iter(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none_mut().0,
                mut_override(path.to_glib_none().0),
            ));
            if ret {
                Some(iter)
            } else {
                None
            }
        }
    }

    #[allow(deprecated)]
    fn iter_first(&self) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_tree_model_get_iter_first(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none_mut().0,
            ));
            if ret {
                Some(iter)
            } else {
                None
            }
        }
    }

    #[allow(deprecated)]
    fn iter_from_string(&self, path_string: &str) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_tree_model_get_iter_from_string(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none_mut().0,
                path_string.to_glib_none().0,
            ));
            if ret {
                Some(iter)
            } else {
                None
            }
        }
    }

    #[allow(deprecated)]
    fn n_columns(&self) -> i32 {
        unsafe { ffi::gtk_tree_model_get_n_columns(self.as_ref().to_glib_none().0) }
    }

    #[allow(deprecated)]
    fn path(&self, iter: &TreeIter) -> TreePath {
        unsafe {
            from_glib_full(ffi::gtk_tree_model_get_path(
                self.as_ref().to_glib_none().0,
                mut_override(iter.to_glib_none().0),
            ))
        }
    }

    #[allow(deprecated)]
    fn string_from_iter(&self, iter: &TreeIter) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gtk_tree_model_get_string_from_iter(
                self.as_ref().to_glib_none().0,
                mut_override(iter.to_glib_none().0),
            ))
        }
    }

    #[allow(deprecated)]
    fn iter_children(&self, parent: Option<&TreeIter>) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_tree_model_iter_children(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none_mut().0,
                mut_override(parent.to_glib_none().0),
            ));
            if ret {
                Some(iter)
            } else {
                None
            }
        }
    }

    #[allow(deprecated)]
    fn iter_has_child(&self, iter: &TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_model_iter_has_child(
                self.as_ref().to_glib_none().0,
                mut_override(iter.to_glib_none().0),
            ))
        }
    }

    #[allow(deprecated)]
    fn iter_n_children(&self, iter: Option<&TreeIter>) -> i32 {
        unsafe {
            ffi::gtk_tree_model_iter_n_children(
                self.as_ref().to_glib_none().0,
                mut_override(iter.to_glib_none().0),
            )
        }
    }

    #[allow(deprecated)]
    fn iter_next(&self, iter: &TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_model_iter_next(
                self.as_ref().to_glib_none().0,
                mut_override(iter.to_glib_none().0),
            ))
        }
    }

    #[allow(deprecated)]
    fn iter_nth_child(&self, parent: Option<&TreeIter>, n: i32) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_tree_model_iter_nth_child(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none_mut().0,
                mut_override(parent.to_glib_none().0),
                n,
            ));
            if ret {
                Some(iter)
            } else {
                None
            }
        }
    }

    #[allow(deprecated)]
    fn iter_parent(&self, child: &TreeIter) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_tree_model_iter_parent(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none_mut().0,
                mut_override(child.to_glib_none().0),
            ));
            if ret {
                Some(iter)
            } else {
                None
            }
        }
    }

    #[allow(deprecated)]
    fn iter_previous(&self, iter: &TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_model_iter_previous(
                self.as_ref().to_glib_none().0,
                mut_override(iter.to_glib_none().0),
            ))
        }
    }

    #[allow(deprecated)]
    fn row_changed(&self, path: &TreePath, iter: &TreeIter) {
        unsafe {
            ffi::gtk_tree_model_row_changed(
                self.as_ref().to_glib_none().0,
                mut_override(path.to_glib_none().0),
                mut_override(iter.to_glib_none().0),
            );
        }
    }

    #[allow(deprecated)]
    fn row_deleted(&self, path: &TreePath) {
        unsafe {
            ffi::gtk_tree_model_row_deleted(
                self.as_ref().to_glib_none().0,
                mut_override(path.to_glib_none().0),
            );
        }
    }

    #[allow(deprecated)]
    fn row_has_child_toggled(&self, path: &TreePath, iter: &TreeIter) {
        unsafe {
            ffi::gtk_tree_model_row_has_child_toggled(
                self.as_ref().to_glib_none().0,
                mut_override(path.to_glib_none().0),
                mut_override(iter.to_glib_none().0),
            );
        }
    }

    #[allow(deprecated)]
    fn row_inserted(&self, path: &TreePath, iter: &TreeIter) {
        unsafe {
            ffi::gtk_tree_model_row_inserted(
                self.as_ref().to_glib_none().0,
                mut_override(path.to_glib_none().0),
                mut_override(iter.to_glib_none().0),
            );
        }
    }

    fn connect_row_changed<F: Fn(&Self, &TreePath, &TreeIter) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn row_changed_trampoline<
            P: IsA<TreeModel>,
            F: Fn(&P, &TreePath, &TreeIter) + 'static,
        >(
            this: *mut ffi::GtkTreeModel,
            path: *mut ffi::GtkTreePath,
            iter: *mut ffi::GtkTreeIter,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                TreeModel::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(path),
                &from_glib_borrow(iter),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"row-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    row_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_row_deleted<F: Fn(&Self, &TreePath) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn row_deleted_trampoline<
            P: IsA<TreeModel>,
            F: Fn(&P, &TreePath) + 'static,
        >(
            this: *mut ffi::GtkTreeModel,
            path: *mut ffi::GtkTreePath,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                TreeModel::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(path),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"row-deleted\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    row_deleted_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_row_has_child_toggled<F: Fn(&Self, &TreePath, &TreeIter) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn row_has_child_toggled_trampoline<
            P: IsA<TreeModel>,
            F: Fn(&P, &TreePath, &TreeIter) + 'static,
        >(
            this: *mut ffi::GtkTreeModel,
            path: *mut ffi::GtkTreePath,
            iter: *mut ffi::GtkTreeIter,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                TreeModel::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(path),
                &from_glib_borrow(iter),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"row-has-child-toggled\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    row_has_child_toggled_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_row_inserted<F: Fn(&Self, &TreePath, &TreeIter) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn row_inserted_trampoline<
            P: IsA<TreeModel>,
            F: Fn(&P, &TreePath, &TreeIter) + 'static,
        >(
            this: *mut ffi::GtkTreeModel,
            path: *mut ffi::GtkTreePath,
            iter: *mut ffi::GtkTreeIter,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                TreeModel::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(path),
                &from_glib_borrow(iter),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"row-inserted\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    row_inserted_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    //fn connect_rows_reordered<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Unimplemented new_order: *.Pointer
    //}
}

impl fmt::Display for TreeModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TreeModel")
    }
}
