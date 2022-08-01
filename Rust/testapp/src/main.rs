use std::{os::{raw::{c_uint, c_int, c_void, c_ushort, c_ulong, c_long}, windows::raw::HANDLE}, ptr::null_mut};

type HINSTANCE = HANDLE;
type HICON = HANDLE;
type HDC = HANDLE;
type HCURSOR = HICON;
type HBRUSH = HANDLE;
type LPCWSTR = *const u16;
type HWND = HANDLE;
type WPARAM = usize;
type LPARAM = isize;
type LRESULT = isize;

const WM_CLOSE: u32 = 0x0010;
const WM_DESTROY: u32 = 0x0002;
const WM_PAINT: u32 = 0x000F;
const WM_NCCREATE: u32 = 0x0081;
const WM_CREATE: u32 = 0x0001;

const WS_OVERLAPPED: u32 = 0x00000000;
const WS_OVERLAPPEDWINDOW: u32 = WS_OVERLAPPED;
const CW_USEDEFAULT: c_int = 0x80000000_u32 as c_int;
const COLOR_WINDOW: u32 = 5;
const SW_SHOW: c_int = 5;
const MB_OKCANCEL: u32 = 1;
const IDOK: c_int = 1;

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

#[repr(C)]
pub struct RECT {
    left: c_long,
    top: c_long,
    right: c_long,
    bottom: c_long,
}
unsafe_impl_default_zeroed!(RECT);

#[repr(C)]
pub struct PAINTSTRUCT {
    hdc: HDC,
    fErase: bool,
    rcPaint: RECT,
    fRestore: bool,
    fIncUpdate: bool,
    rgbReserved: [u8; 32]
}
unsafe_impl_default_zeroed!(PAINTSTRUCT);

pub unsafe extern "system" fn window_procedure(
    hWnd: HWND, uMsg: c_uint, wParam: WPARAM, lParam: LPARAM,
) -> LRESULT {
    match uMsg {
        WM_CLOSE => drop(DestroyWindow(hWnd)),
        WM_DESTROY => PostQuitMessage(0),
        WM_PAINT => {
            let mut ps:PAINTSTRUCT = PAINTSTRUCT::default();
            let hdc: *mut c_void = BeginPaint(hWnd, &mut ps);
            let _success = FillRect(hdc, &ps.rcPaint, (COLOR_WINDOW + 1) as HBRUSH);
            EndPaint(hWnd, &ps);
        }
        // WM_NCCREATE => {
        //     println!("NC Create");
        //     return 0;
        // }
        // WM_CREATE => println!("Create"),
        _ => return DefWindowProcW(hWnd, uMsg, wParam, lParam),
    }
    0
}
  
pub fn wide_null(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}

#[link(name = "Kernel32")]
extern "system" {
  /// [`GetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
  pub fn GetLastError() -> c_ulong;
  /// [`GetModuleHandleW`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)
  pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HINSTANCE;
}

#[link(name = "User32")]
extern "system" {
    // [`ShowWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)
    pub fn ShowWindow(hWnd: HWND, nCmdShow: c_int) -> c_int;
    // [`TranslateMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translatemessage)
    pub fn TranslateMessage(lspMsg: *const MSG) -> bool;
    // [`DispatchMessageW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dispatchmessagew)
    pub fn DispatchMessageW(lspMsg: *const MSG) -> LRESULT;
    // [`CreateWindowExW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw)
    pub fn CreateWindowExW(
        dwExStyle: c_ulong, lpClassName: LPCWSTR, lpWindowName: LPCWSTR,
        dwStyle: c_ulong, X: c_int, Y: c_int, nWidth: c_int, nHeight: c_int,
        hWndParent: HWND, hMenu: HANDLE, hInstance: HINSTANCE, lpParam: *mut c_void,
    ) -> HWND;
    // [`GetMessageW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew)
    pub fn GetMessageW(
        lpMsg: &MSG, hWnd: HWND, wMsgFilterMin: c_uint, wMsgFilterMax: c_uint,
    ) -> c_int;
    // [`RegisterClassW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassw)
    pub fn RegisterClassW(lpWndClass: *const WNDCLASSW) -> c_ushort;
    // [`DefWindowProcW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-defwindowprocw)
    pub fn DefWindowProcW(
        hWnd: HWND, Msg: c_uint, wParam: WPARAM, lParam: LPARAM,
    ) -> LRESULT;
    // [`DestroyWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroywindow)
    pub fn DestroyWindow(hwnd: HWND) -> bool;
    // [`PostQuitMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postquitmessage)
    pub fn PostQuitMessage(nExtCode: c_int);
    // [`LoadCursorW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw)
    pub fn LoadCursorW(hInstance: HINSTANCE, lpCursorName: LPCWSTR) -> HCURSOR;
    // ['BeginPaint'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-beginpaint)
    pub fn BeginPaint(hWnd: HWND, lpPaint: *const PAINTSTRUCT) -> HDC;
    // ['FillRect'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-fillrect)
    pub fn FillRect(hDc: HDC, lprc: *const RECT, hbr: HBRUSH) -> c_int;
    // ['EndPaint'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-endpaint)
    pub fn EndPaint(hWnd: HWND, lpPaint: *const PAINTSTRUCT) -> bool;
    // ['MessageBox'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messagebox)
    pub fn MessageBox(hWnd: HWND, lpText: LPCWSTR, lpCaption: LPCWSTR, uType: c_uint) -> c_int;
}

fn main() {
    let h_instance: *mut c_void = unsafe { GetModuleHandleW(core::ptr::null()) };
    let sample_window_class_wn: Vec<u16> = wide_null("Sample Window Class");
    let mut window_class: WNDCLASSW = WNDCLASSW::default();
    window_class.lpfnWndProc = Some(window_procedure);
    window_class.hInstance = h_instance;
    window_class.lpszClassName = sample_window_class_wn.as_ptr();
    let atom: u16 = unsafe { RegisterClassW(&window_class) };
    if atom == 0 {
        let last_error: u32 = unsafe { GetLastError() };
        panic!("Could not register the window class, error code: {}", last_error);
    }
    let sample_window_name_wn: Vec<u16> = wide_null("Testapp name placeholder");
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
    let mut msg: MSG = MSG::default();
    loop {
        let message_return: i32 = unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) };
        if message_return == 0 {
            break;
        } else if message_return == -1 {
            let last_error: u32 = unsafe { GetLastError() };
            panic!("Error with 'GetMessageW', error code: {}", last_error)     
        } else {
            unsafe {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    }
}
