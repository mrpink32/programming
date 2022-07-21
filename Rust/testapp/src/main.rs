use std::{os::{raw::{c_uint, c_int, c_void, c_ushort, c_ulong, c_long}, windows::raw::HANDLE}, ptr::null_mut};

type HINSTANCE = HANDLE;
type HICON = HANDLE;
type HCURSOR = HICON;
type HBRUSH = HANDLE;
type LPCWSTR = *const u16;
type HWND = HANDLE;
type WPARAM = usize;
type LPARAM = isize;
type LRESULT = isize;

const WS_OVERLAPPED: u32 = 0x00000000;
const WS_OVERLAPPEDWINDOW: u32 = WS_OVERLAPPED;
const CW_USEDEFAULT: c_int = 0x80000000_u32 as c_int;

const SW_SHOW: c_int = 5;

type WNDPROC = Option<
    unsafe extern "system" fn(
        hwnd: HWND,
        uMsg: c_uint,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> LRESULT,
>;

macro_rules! unsafe_impl_default_zeroed {
    ($t:ty) => {
        impl Default for $t {
            #[inline]
            #[must_use]
            fn default() -> Self {
                unsafe { core::mem::zeroed() }
            }
        }        
    };
}

#[repr(C)]
pub struct WNDCLASSW {
    style: c_uint,
    lpfnWndProc: WNDPROC,
    cbClsExtra: c_int,
    cbWndExtra: c_int,
    hInstance: HINSTANCE,
    hIcon: HICON,
    hCursor: HCURSOR,
    hbrBackground: HBRUSH,
    lpszMenuName: LPCWSTR,
    lpszClassName: LPCWSTR,
}
unsafe_impl_default_zeroed!(WNDCLASSW);

#[repr(C)]
pub struct POINT {
    x: c_long,
    y: c_long,
}
unsafe_impl_default_zeroed!(POINT);

#[repr(C)]
pub struct MSG {
    hwnd: HANDLE,
    message: c_uint,
    wParam: WPARAM,
    lParam: LPARAM,
    time: c_uint,
    pt: POINT,
    lPrivate: c_uint,
}
unsafe_impl_default_zeroed!(MSG);

unsafe extern "system" fn dummy_window_procedure(
  hwnd: HWND, uMsg: c_uint, wParam: WPARAM, lParam: LPARAM,
) -> LRESULT {
  unimplemented!()
}

#[link(name = "User32")]
extern "system" {
    /// [`DefWindowProcW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-defwindowprocw)
    pub fn DefWindowProcW(
        hWnd: HWND, Msg: c_uint, wParam: WPARAM, lParam: LPARAM,
    ) -> LRESULT;
}

#[link(name = "Kernel32")]
extern "system" {
    /// [`GetModuleHandleW`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)
    pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HINSTANCE;
}

#[link(name = "User32")]
extern "system" {
  /// [`RegisterClassW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassw)
  pub fn RegisterClassW(lpWndClass: *const WNDCLASSW) -> c_ushort;
}

#[link(name = "Kernel32")]
extern "system" {
  /// [`GetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
  pub fn GetLastError() -> c_ulong;
}

#[link(name = "User32")]
extern "system" {
    /// [`GetMessageW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew)
    pub fn GetMessageW(
        lpMsg: &MSG, hWnd: HWND, wMsgFilterMin: c_uint, wMsgFilterMax: c_uint,
    ) -> c_int;
}

#[link(name = "User32")]
extern "system" {
    /// [`CreateWindowExW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw)
    pub fn CreateWindowExW(
        dwExStyle: c_ulong, lpClassName: LPCWSTR, lpWindowName: LPCWSTR,
        dwStyle: c_ulong, X: c_int, Y: c_int, nWidth: c_int, nHeight: c_int,
        hWndParent: HWND, hMenu: HANDLE, hInstance: HINSTANCE, lpParam: *mut c_void,
    ) -> HWND;
}

pub fn wide_null(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}


#[link(name = "User32")]
extern "system" {
    /// [`ShowWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)
    pub fn ShowWindow(hWnd: HWND, nCmdShow: c_int) -> c_int;
}

fn create_window() {
    let h_instance: *mut c_void = unsafe { GetModuleHandleW(core::ptr::null()) };
    let sample_window_class_wn: Vec<u16> = wide_null("Sample Window Class");
    let mut wc: WNDCLASSW = WNDCLASSW::default();
    wc.lpfnWndProc = Some(DefWindowProcW);
    wc.hInstance = h_instance;
    wc.lpszClassName = sample_window_class_wn.as_ptr();

    let atom: u16 = unsafe { RegisterClassW(&wc) };
    if atom == 0 {
        let last_error: u32 = unsafe { GetLastError() };
        panic!("Could not register the window class, error code: {}", last_error);
    }

    let sample_window_name_wn: Vec<u16> = wide_null("Sample Window Name");
    let hwnd: *mut c_void = unsafe {
        CreateWindowExW(
            0,
            sample_window_class_wn.as_ptr(),
            sample_window_name_wn.as_ptr(),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            h_instance,
            core::ptr::null_mut(),
        )
    };
    if hwnd.is_null() {
        panic!("Failed to create a window.");
    }

    let _previously_visible: i32 = unsafe { ShowWindow(hwnd, SW_SHOW) };
}

fn main() {
    create_window();
    let mut msg: MSG = MSG::default();
    loop {
        let message_return: i32 = unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) };
        if message_return == 0 {
            break;
        } else if message_return == -1 {
            let last_error: u32 = unsafe { GetLastError() };
            panic!("Error with 'GetMessageW', error code: {}", last_error)     
        }
        
        // match message_return {
        //     0 =>    break,
        //     -1 =>   let last_error = unsafe { GetLastError() };
        //             panic!("Error with `GetMessageW`, error code: {}", last_error),
        //     _ =>    println!("Grim"),
        // };
    }
}
