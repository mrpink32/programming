use std::os::{raw::*, windows::raw::HANDLE};

// #![allow(unused)]

pub type HINSTANCE = HANDLE;
pub type HICON = HANDLE;
pub type HDC = HANDLE;
pub type HCURSOR = HICON;
pub type HBRUSH = HANDLE;
pub type LPCWSTR = *const u16;
pub type HWND = HANDLE;
pub type WPARAM = usize;
pub type LPARAM = isize;
pub type LRESULT = isize;
pub type HMENU = HANDLE;
pub type LPVOID = *mut c_void;
pub type LPMSG = *mut MSG;
pub type ATOM = c_ulong;
pub type BOOL = bool;
pub type DWORD = c_ulong;
pub type UINT = c_uint;
pub type COLORREF = DWORD;
pub type LPCOLORREF = *mut COLORREF;
pub type BYTE = c_uchar;
pub type PBYTE = *mut BYTE;
pub type HPEN = HANDLE;
pub type HGDIOBJ = HANDLE;
pub type LPPOINT = *mut POINT;

pub const WM_COMMAND: u32 = 0x0111;
pub const WM_CLOSE: u32 = 0x0010;
pub const WM_DESTROY: u32 = 0x0002;
pub const WM_PAINT: u32 = 0x000F;
pub const WM_SIZE: u32 = 0x0005;
pub const WM_NCCREATE: u32 = 0x0081;
pub const WM_CREATE: u32 = 0x0001;

pub const WS_BORDER: DWORD = 0x00800000;
pub const WS_CAPTION: DWORD = 0x00C00000;
pub const WS_OVERLAPPED: DWORD = 0x00000000;
pub const WS_MAXIMIZEBOX: DWORD = 0x00010000;
pub const WS_MINIMIZEBOX: DWORD = 0x00020000;
pub const WS_OVERLAPPEDWINDOW: DWORD =
    WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;
pub const WS_SYSMENU: DWORD = 0x00080000;
pub const WS_THICKFRAME: DWORD = 0x00040000;

pub const WS_EX_LEFT: DWORD = 0x00000000;
pub const WS_EX_OVERLAPPEDWINDOW: DWORD = 0x00000100;
pub const WS_EX_WINDOWEDGE: DWORD = 0x00000100;

pub const CW_USEDEFAULT: c_int = 0x80000000_u32 as c_int;

pub const SW_HIDE: c_int = 0;
pub const SW_SHOWNORMAL: c_int = 1;
pub const SW_SHOWMINIMIZED: c_int = 2;
pub const SW_SHOWMAXIMIZED: c_int = 3;
pub const SW_SHOWNOACTIVATE: c_int = 4;
pub const SW_SHOW: c_int = 5;
pub const SW_MINIMIZE: c_int = 6;
pub const SW_SHOWMINNOACTIVE: c_int = 7;
pub const SW_SHOWNA: c_int = 8;
pub const SW_RESTORE: c_int = 9;
pub const SW_SHOWDEFAULT: c_int = 10;
pub const SW_FORCEMINIMIZE: c_int = 11;

pub const MB_OK: u32 = 0x00000000;
pub const MB_OKCANCEL: u32 = 1;
pub const MB_YESNO: u32 = 4;

pub const MF_BITMAP: u32 = 0x00000004;
pub const MF_CHECKED: u32 = 0x00000008;
pub const MF_DISABLED: u32 = 0x00000010;
pub const MF_ENABLED: u32 = 0x00000000;
pub const MF_GRAYED: u32 = 0x00000001;
pub const MF_MENUBARBREAK: u32 = 0x00000020;
pub const MF_MENUBREAK: u32 = 0x00000040;
pub const MF_OWNERDRAW: u32 = 0x00000100;
pub const MF_POPUP: u32 = 0x00000010;
pub const MF_SEPARATOR: u32 = 0x00000800;
pub const MF_STRING: u32 = 0x00000000;
pub const MF_UNCHECKED: u32 = 0x00000000;

pub const IDOK: c_int = 1;
pub const IDYES: c_int = 6;

pub const COLOR_BACKGROUND: u32 = 1;
pub const COLOR_DESKTOP: u32 = 1;
pub const COLOR_WINDOW: u32 = 5;

pub const rgbRed: COLORREF = 0x000000FF;
pub const rgbGreen: COLORREF = 0x0000FF00;
pub const rgbBlue: COLORREF = 0x00FF0000;
pub const rgbBlack: COLORREF = 0x00000000;
pub const rgbWhite: COLORREF = 0x00FFFFFF;

pub const PM_NOREMOVE: u32 = 0x0000;
pub const PM_REMOVE: u32 = 0x0001;
pub const PM_NOYIELD: u32 = 0x0002;

pub const PS_SOLID: i32 = 0;
pub const PS_DASH: i32 = 1;
pub const PS_DOT: i32 = 2;
pub const PS_DASHDOT: i32 = 3;
pub const PS_DASHDOTDOT: i32 = 4;
pub const PS_NULL: i32 = 5;
pub const PS_INSIDEFRAME: i32 = 6;

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

pub type WNDPROC = Option<
    unsafe extern "system" fn(hwnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT,
>;

#[repr(C)]
pub struct WNDCLASSW {
    pub style: c_uint,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: c_int,
    pub cbWndExtra: c_int,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: LPCWSTR,
    pub lpszClassName: LPCWSTR,
}
unsafe_impl_default_zeroed!(WNDCLASSW);

#[repr(C)]
pub struct POINT {
    pub x: c_long,
    pub y: c_long,
}
unsafe_impl_default_zeroed!(POINT);

#[repr(C)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: c_uint,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: DWORD,
    pub pt: POINT,
    pub lPrivate: DWORD,
}
unsafe_impl_default_zeroed!(MSG);

#[repr(C)]
pub struct RECT {
    pub left: c_long,
    pub top: c_long,
    pub right: c_long,
    pub bottom: c_long,
}
unsafe_impl_default_zeroed!(RECT);

#[repr(C)]
pub struct PAINTSTRUCT {
    pub hdc: HDC,
    pub fErase: bool,
    pub rcPaint: RECT,
    pub fRestore: bool,
    pub fIncUpdate: bool,
    pub rgbReserved: [u8; 32],
}
unsafe_impl_default_zeroed!(PAINTSTRUCT);

#[link(name = "Gdi32")]
extern "system" {
    // ['CreateSolidBrush'] (https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createsolidbrush)
    pub fn CreateSolidBrush(color: COLORREF) -> HBRUSH;
    // ['Rectangle'] (https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-rectangle)
    pub fn Rectangle(
        hdc: HDC,
        nLeftRect: c_int,
        nTopRect: c_int,
        nRightRect: c_int,
        nBottomRect: c_int,
    ) -> BOOL;
    // ['CreatePen'] (https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createpen)
    pub fn CreatePen(nPenStyle: c_int, nWidth: c_int, color: COLORREF) -> HPEN;
    // ['SelectObject'] (https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject)
    pub fn SelectObject(hdc: HDC, hgdiobj: HGDIOBJ) -> HGDIOBJ;
    // ['SetBkColor'] (https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setbkcolor)
    pub fn SetBkColor(hdc: HDC, color: COLORREF) -> COLORREF;
    // ['DeleteObject'] (https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-deleteobject)
    pub fn DeleteObject(hObject: HGDIOBJ) -> BOOL;
    // ['RGB'] (https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-rgb)
    pub fn RGB(r: BYTE, g: BYTE, b: BYTE) -> COLORREF;
}

#[link(name = "Kernel32")]
extern "system" {
    // ['GetLastError'](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
    pub fn GetLastError() -> DWORD;
    // ['GetModuleHandleExW'](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)
    pub fn GetModuleHandleExW(dwFlags: DWORD, lpModuleName: LPCWSTR) -> HINSTANCE;
    // ['GetModuleHandleExW'](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)
    pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HINSTANCE;
}

#[link(name = "User32")]
extern "system" {
    // ['ShowWindow'](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)
    pub fn ShowWindow(hWnd: HWND, nCmdShow: c_int) -> BOOL;
    // ['GetWindowRect'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowrect)
    pub fn GetWindowRect(hWnd: HWND, lpRect: *mut RECT) -> BOOL;
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
    // ['CreateWindowW'](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw)
    pub fn CreateWindowW(
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
    pub fn GetMessageW(lpMsg: LPMSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT) -> BOOL;
    // ['PeekMessageW'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-peekmessagew)
    pub fn PeekMessageW(
        lpMsg: LPMSG,
        hWnd: HWND,
        wMsgFilterMin: UINT,
        wMsgFilterMax: UINT,
        wRemoveMsg: UINT,
    ) -> BOOL;
    // ['SendMessageW'](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessagew)
    pub fn SendMessageW(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    // ['RegisterClassW'](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassw)
    pub fn RegisterClassW(lpWndClass: *const WNDCLASSW) -> ATOM;
    // ['DefWindowProcW'](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-defwindowprocw)
    pub fn DefWindowProcW(hWnd: HWND, Msg: c_uint, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    // ['CloseWindow'](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-closewindow)
    pub fn CloseWindow(hWnd: HWND) -> BOOL;
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
    // ['CreateMenu'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createmenu)
    pub fn CreateMenu() -> HMENU;
    // ['AppendMenuW'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-appendmenuw)
    pub fn AppendMenuW(hMenu: HMENU, uFlags: UINT, uIDNewItem: UINT, lpNewItem: LPCWSTR) -> BOOL;
    // ['SetMenu'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setmenu)
    pub fn SetMenu(hWnd: HWND, hMenu: HMENU) -> BOOL;
    // ['GetCursorPos'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getcursorpos)
    pub fn GetCursorPos(lpPoint: LPPOINT) -> BOOL;
    // ['ScreenToClient'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-screentoclient)
    pub fn ScreenToClient(hwnd: HWND, lpPoint: LPPOINT) -> BOOL;
    // ['GetKeyState'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getkeystate)
    pub fn GetKeyState(nVirtKey: c_int) -> c_short;
    // ['GetKeyboardState'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getkeyboardstate)
    pub fn GetKeyboardState(lpKeyState: PBYTE) -> BOOL;
}
