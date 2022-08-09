use std::{os::{raw::{c_uint, c_int, c_void, c_ushort, c_ulong, c_long}, windows::raw::HANDLE}, ptr::{null_mut, null}};

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
type HMENU = HANDLE;
type LPVOID = *mut c_void;
type LPMSG = *mut MSG;
type ATOM = c_ulong;
type BOOL = c_int;
type DWORD = c_ulong;
type UINT = c_uint;

const WM_COMMAND: u32 = 0x0111;
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
const MB_YESNO: u32 = 4;

const IDOK: c_int = 1;
const IDYES: c_int = 6;

const COLOR_DESKTOP: u32 = 1;

type WNDPROC = Option<
    unsafe extern "system" fn(
        hwnd: HWND,
        uMsg: UINT,
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
    hwnd: HWND,
    message: c_uint,
    wParam: WPARAM,
    lParam: LPARAM,
    time: DWORD,
    pt: POINT,
    lPrivate: DWORD,
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

pub fn wide_null(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}

#[link(name = "Kernel32")]
extern "system" {
    // ['GetLastError'](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
    pub fn GetLastError() -> DWORD;
    // ['GetModuleHandleW'](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)
    pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HINSTANCE;
}

#[link(name = "User32")]
extern "system" {
    // ['ShowWindow'](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)
    pub fn ShowWindow(hWnd: HWND, nCmdShow: c_int) -> BOOL;
    // ['TranslateMessage'](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translatemessage)
    pub fn TranslateMessage(lpMsg: LPMSG) -> BOOL;
    // ['DispatchMessageW'](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dispatchmessagew)
    pub fn DispatchMessageW(lpmsg: LPMSG) -> LRESULT;
    // ['CreateWindowExW'](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw)
    pub fn CreateWindowExW(
        dwExStyle: c_ulong,
        lpClassName: LPCWSTR,
        lpWindowName: LPCWSTR,
        dwStyle: c_ulong,
        x: c_int,
        y: c_int,
        nWidth: c_int,
        nHeight: c_int,
        hWndParent: HWND,
        hMenu: HMENU,
        hInstance: HINSTANCE,
        lpParam: LPVOID,
    ) -> HWND;
    // ['GetMessageW'](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew)
    pub fn GetMessageW(
        lpMsg: LPMSG,
        hWnd: HWND,
        wMsgFilterMin: c_uint,
        wMsgFilterMax: c_uint,
    ) -> c_int;
    // ['RegisterClassW'](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassw)
    pub fn RegisterClassW(lpWndClass: *const WNDCLASSW) -> ATOM;
    // ['DefWindowProcW'](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-defwindowprocw)
    pub fn DefWindowProcW(
        hWnd: HWND,
        Msg: c_uint,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> LRESULT;
    // ['DestroyWindow'](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroywindow)
    pub fn DestroyWindow(hWnd: HWND) -> BOOL;
    // ['PostQuitMessage'](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postquitmessage)
    pub fn PostQuitMessage(nExitCode: c_int);
    // ['LoadCursorW'](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw)
    pub fn LoadCursorW(hInstance: HINSTANCE, lpCursorName: LPCWSTR) -> HCURSOR;
    // ['BeginPaint'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-beginpaint)
    pub fn BeginPaint(hWnd: HWND, lpPaint: *const PAINTSTRUCT) -> HDC;
    // ['FillRect'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-fillrect)
    pub fn FillRect(hDc: HDC, lprc: *const RECT, hbr: HBRUSH) -> c_int;
    // ['EndPaint'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-endpaint)
    pub fn EndPaint(hWnd: HWND, lpPaint: *const PAINTSTRUCT) -> BOOL;
    // ['MessageBox'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messagebox)
    pub fn MessageBox(hWnd: HWND, lpText: LPCWSTR, lpCaption: LPCWSTR, uType: c_uint) -> c_int;
    // ['GetClientRect'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclientrect)
    pub fn GetClientRect(hWnd: HWND, lpRect: *mut RECT) -> BOOL;
    // ['MessageBoxW'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxw)
    pub fn MessageBoxW(hWnd: HWND, lpText: LPCWSTR, lpCaption: LPCWSTR, uType: c_uint) -> c_int;
}

unsafe extern "system" fn window_procedure(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_CLOSE => {
           if MessageBoxW(hwnd, wide_null("Are you sure you want to exit?").as_ptr(), wide_null("Exit prompt").as_ptr(), MB_YESNO) == IDYES {
               DestroyWindow(hwnd);
           }
           return 0;
        } 
        WM_DESTROY => {
            PostQuitMessage(0);
            return 0;
        }
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = PAINTSTRUCT::default();
            let hdc: HDC = BeginPaint(hwnd, &mut ps);
            FillRect(hdc, &ps.rcPaint, COLOR_DESKTOP as HBRUSH);
            EndPaint(hwnd, &ps);
            return 0;
        }
        _ => {
            return DefWindowProcW(hwnd, msg, wparam, lparam);
        }
    }
}

fn main() {
    let hinstance: HINSTANCE = unsafe { GetModuleHandleW(null_mut()) };
    let class_name: Vec<u16> = wide_null("ServerManagerWindowClass");
    let mut window_class: WNDCLASSW = WNDCLASSW::default();
    window_class.lpfnWndProc = Some(window_procedure);
    window_class.hInstance = hinstance;
    window_class.lpszClassName = class_name.as_ptr();
    let atom: ATOM = unsafe { RegisterClassW(&window_class) };
    if atom == 0 {
        let last_error: c_ulong = unsafe { GetLastError() };
        panic!("Could not register the window class, error code: {}", last_error);
    }
    let window_name: Vec<u16> = wide_null("ServerManagerClient");
    let hwnd: HWND = unsafe {
        CreateWindowExW(
            0,
            class_name.as_ptr(),
            window_name.as_ptr(),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            null_mut(),
            null_mut(),
            hinstance,
            null_mut(),
        )
    };
    if hwnd.is_null() {
        let last_error: c_ulong = unsafe { GetLastError() };
        panic!("Could not register the window class, error code: {}", last_error);
    }
    unsafe { ShowWindow(hwnd, SW_SHOW) };
    let mut msg: MSG = MSG::default();
    while unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) } != 0 {
        unsafe { TranslateMessage(&mut msg) };
        unsafe { DispatchMessageW(&mut msg) };
    }
}
