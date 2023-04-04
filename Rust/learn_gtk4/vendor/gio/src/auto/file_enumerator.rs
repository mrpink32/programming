// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{AsyncResult, Cancellable, File, FileInfo};
use glib::{prelude::*, translate::*};
use std::{boxed::Box as Box_, fmt, pin::Pin, ptr};

glib::wrapper! {
    #[doc(alias = "GFileEnumerator")]
    pub struct FileEnumerator(Object<ffi::GFileEnumerator, ffi::GFileEnumeratorClass>);

    match fn {
        type_ => || ffi::g_file_enumerator_get_type(),
    }
}

impl FileEnumerator {
    pub const NONE: Option<&'static FileEnumerator> = None;
}

pub trait FileEnumeratorExt: 'static {
    #[doc(alias = "g_file_enumerator_close")]
    fn close(&self, cancellable: Option<&impl IsA<Cancellable>>) -> Result<(), glib::Error>;

    #[doc(alias = "g_file_enumerator_close_async")]
    fn close_async<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    );

    fn close_future(
        &self,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    #[doc(alias = "g_file_enumerator_get_child")]
    #[doc(alias = "get_child")]
    fn child(&self, info: &FileInfo) -> File;

    #[doc(alias = "g_file_enumerator_get_container")]
    #[doc(alias = "get_container")]
    fn container(&self) -> File;

    #[doc(alias = "g_file_enumerator_has_pending")]
    fn has_pending(&self) -> bool;

    #[doc(alias = "g_file_enumerator_is_closed")]
    fn is_closed(&self) -> bool;

    #[doc(alias = "g_file_enumerator_next_file")]
    fn next_file(
        &self,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<Option<FileInfo>, glib::Error>;

    #[doc(alias = "g_file_enumerator_next_files_async")]
    fn next_files_async<P: FnOnce(Result<Vec<FileInfo>, glib::Error>) + 'static>(
        &self,
        num_files: i32,
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    );

    fn next_files_future(
        &self,
        num_files: i32,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Vec<FileInfo>, glib::Error>> + 'static>>;

    #[doc(alias = "g_file_enumerator_set_pending")]
    fn set_pending(&self, pending: bool);
}

impl<O: IsA<FileEnumerator>> FileEnumeratorExt for O {
    fn close(&self, cancellable: Option<&impl IsA<Cancellable>>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::g_file_enumerator_close(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn close_async<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn close_async_trampoline<
            P: FnOnce(Result<(), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::g_file_enumerator_close_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = close_async_trampoline::<P>;
        unsafe {
            ffi::g_file_enumerator_close_async(
                self.as_ref().to_glib_none().0,
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn close_future(
        &self,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.close_async(io_priority, Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }

    fn child(&self, info: &FileInfo) -> File {
        unsafe {
            from_glib_full(ffi::g_file_enumerator_get_child(
                self.as_ref().to_glib_none().0,
                info.to_glib_none().0,
            ))
        }
    }

    fn container(&self) -> File {
        unsafe {
            from_glib_none(ffi::g_file_enumerator_get_container(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn has_pending(&self) -> bool {
        unsafe {
            from_glib(ffi::g_file_enumerator_has_pending(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_closed(&self) -> bool {
        unsafe {
            from_glib(ffi::g_file_enumerator_is_closed(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn next_file(
        &self,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<Option<FileInfo>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_file_enumerator_next_file(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn next_files_async<P: FnOnce(Result<Vec<FileInfo>, glib::Error>) + 'static>(
        &self,
        num_files: i32,
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn next_files_async_trampoline<
            P: FnOnce(Result<Vec<FileInfo>, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret =
                ffi::g_file_enumerator_next_files_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(FromGlibPtrContainer::from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = next_files_async_trampoline::<P>;
        unsafe {
            ffi::g_file_enumerator_next_files_async(
                self.as_ref().to_glib_none().0,
                num_files,
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn next_files_future(
        &self,
        num_files: i32,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Vec<FileInfo>, glib::Error>> + 'static>>
    {
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.next_files_async(num_files, io_priority, Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }

    fn set_pending(&self, pending: bool) {
        unsafe {
            ffi::g_file_enumerator_set_pending(self.as_ref().to_glib_none().0, pending.into_glib());
        }
    }
}

impl fmt::Display for FileEnumerator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FileEnumerator")
    }
}