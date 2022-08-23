#![allow(unused)]

use std::{
    f64::INFINITY,
    io::{BufRead, BufReader, BufWriter, Read, Result, Write},
    net::TcpStream,
    os::{
        raw::{c_int, c_long, c_uint, c_ulong, c_ushort, c_void},
        windows::raw::HANDLE,
    },
    ptr::{null, null_mut},
    thread,
};

struct BufTcpStream {
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
}

impl BufTcpStream {
    fn new(stream: TcpStream) -> Result<Self> {
        let reader: BufReader<TcpStream> = BufReader::new(stream.try_clone()?);
        let writer: BufWriter<TcpStream> = BufWriter::new(stream.try_clone()?);

        Ok(Self { reader, writer })
    }
}

mod win_api {}

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
type BOOL = bool;
type DWORD = c_ulong;
type UINT = c_uint;
type COLORREF = DWORD;
type LPCOLORREF = *mut COLORREF;
type BYTE = u8;
type HPEN = HANDLE;
type HGDIOBJ = HANDLE;
type LPPOINT = *mut POINT;

const WM_COMMAND: u32 = 0x0111;
const WM_CLOSE: u32 = 0x0010;
const WM_DESTROY: u32 = 0x0002;
const WM_PAINT: u32 = 0x000F;
const WM_SIZE: u32 = 0x0005;
const WM_NCCREATE: u32 = 0x0081;
const WM_CREATE: u32 = 0x0001;

const WS_BORDER: DWORD = 0x00800000;
const WS_CAPTION: DWORD = 0x00C00000;
const WS_OVERLAPPED: DWORD = 0x00000000;
const WS_MAXIMIZEBOX: DWORD = 0x00010000;
const WS_MINIMIZEBOX: DWORD = 0x00020000;
const WS_OVERLAPPEDWINDOW: DWORD =
    WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;
const WS_SYSMENU: DWORD = 0x00080000;
const WS_THICKFRAME: DWORD = 0x00040000;

const WS_EX_LEFT: DWORD = 0x00000000;
const WS_EX_OVERLAPPEDWINDOW: DWORD = 0x00000100;
const WS_EX_WINDOWEDGE: DWORD = 0x00000100;

const CW_USEDEFAULT: c_int = 0x80000000_u32 as c_int;

const SW_HIDE: c_int = 0;
const SW_SHOWNORMAL: c_int = 1;
const SW_SHOWMINIMIZED: c_int = 2;
const SW_SHOWMAXIMIZED: c_int = 3;
const SW_SHOWNOACTIVATE: c_int = 4;
const SW_SHOW: c_int = 5;
const SW_MINIMIZE: c_int = 6;
const SW_SHOWMINNOACTIVE: c_int = 7;
const SW_SHOWNA: c_int = 8;
const SW_RESTORE: c_int = 9;
const SW_SHOWDEFAULT: c_int = 10;
const SW_FORCEMINIMIZE: c_int = 11;

const MB_OK: u32 = 0x00000000;
const MB_OKCANCEL: u32 = 1;
const MB_YESNO: u32 = 4;

const MF_BITMAP: u32 = 0x00000004;
const MF_CHECKED: u32 = 0x00000008;
const MF_DISABLED: u32 = 0x00000010;
const MF_ENABLED: u32 = 0x00000000;
const MF_GRAYED: u32 = 0x00000001;
const MF_MENUBARBREAK: u32 = 0x00000020;
const MF_MENUBREAK: u32 = 0x00000040;
const MF_OWNERDRAW: u32 = 0x00000100;
const MF_POPUP: u32 = 0x00000010;
const MF_SEPARATOR: u32 = 0x00000800;
const MF_STRING: u32 = 0x00000000;
const MF_UNCHECKED: u32 = 0x00000000;

const IDOK: c_int = 1;
const IDYES: c_int = 6;

const COLOR_BACKGROUND: u32 = 1;
const COLOR_DESKTOP: u32 = 1;
const COLOR_WINDOW: u32 = 5;

const rgbRed: COLORREF = 0x000000FF;
const rgbGreen: COLORREF = 0x0000FF00;
const rgbBlue: COLORREF = 0x00FF0000;
const rgbBlack: COLORREF = 0x00000000;
const rgbWhite: COLORREF = 0x00FFFFFF;

const PS_SOLID: i32 = 0;
const PS_DASH: i32 = 1;
const PS_DOT: i32 = 2;
const PS_DASHDOT: i32 = 3;
const PS_DASHDOTDOT: i32 = 4;
const PS_NULL: i32 = 5;
const PS_INSIDEFRAME: i32 = 6;

type WNDPROC = Option<
    unsafe extern "system" fn(hwnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT,
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
    rgbReserved: [u8; 32],
}
unsafe_impl_default_zeroed!(PAINTSTRUCT);

pub fn wide_null(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}

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
    pub fn GetMessageW(
        lpMsg: LPMSG,
        hWnd: HWND,
        wMsgFilterMin: c_uint,
        wMsgFilterMax: c_uint,
    ) -> c_int;
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
}

unsafe fn draw_line(hdc: HDC, x1: i32, y1: i32, x2: i32, y2: i32) {
    let a: f64 = (y2 - y1) as f64 / (x2 - x1) as f64;
    let b: f64 = y1 as f64 - a * x1 as f64;

    // println!("a: {}", a);
    if a != INFINITY {
        for x in x1..=x2 {
            let mut y: i32;
            y = (a * x as f64 + b).floor() as i32;
            // println!("x: {}, y: {}", x, y);
            Rectangle(hdc, x, y, x + 2, y + 2);
        }
    } else {
        for y in y1..=y2 {
            let x: i32 = x1;
            // println!("x: {}, y: {}", x, y);
            Rectangle(hdc, x, y, x + 2, y + 2);
        }
    }
}

fn draw_circle(hdc: HDC, x: i32, y: i32, r: i32) {
    // TODO
}

unsafe extern "system" fn window_procedure(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let mut window_width: i32 = 0;
    let mut window_height: i32 = 0;
    match msg {
        WM_CREATE => {
            // todo!("Creastruct");

            let menu: HMENU = CreateMenu();
            let file: HMENU = CreateMenu();

            AppendMenuW(menu, MF_POPUP, file as UINT, wide_null("Menu").as_ptr());
            AppendMenuW(menu, MF_STRING, 0, wide_null("File").as_ptr());

            AppendMenuW(file, MF_STRING, 1, wide_null("connect").as_ptr());
            AppendMenuW(file, MF_SEPARATOR, 0, 0 as LPCWSTR);
            AppendMenuW(file, MF_STRING, 2, wide_null("Exit").as_ptr());

            SetMenu(hwnd, menu);

            thread::Builder::new()
                .name("thread1".to_string())
                .spawn(move || {
                    let stream: TcpStream = match TcpStream::connect("localhost:9000") {
                        Ok(stream) => {
                            println!("Connected to the server!");
                            stream
                        }
                        Err(e) => {
                            println!("Couldn't connect to server: {}", e);
                            return;
                        }
                    };
                    let mut buf_stream: BufTcpStream =
                        BufTcpStream::new(stream.try_clone().unwrap())
                            .expect("Failed to create buffered stream from networkstream!");
                    loop {
                        // get cursor position
                        let mut mouse_pos: POINT = POINT::default();
                        GetCursorPos(&mut mouse_pos);

                        let bytes_written: usize = buf_stream
                            .writer
                            .write(
                                String::from(format!("{},{}\n", mouse_pos.x, mouse_pos.y))
                                    .as_bytes(),
                            )
                            .expect("Failed to write to stream!");
                        println!("Wrote: {} bytes to stream", bytes_written);
                        buf_stream.writer.flush().unwrap();
                    }
                });

            return 0;
        }
        // WM_NCCREATE => {
        //     // todo!("NCCreate");
        //     return 1;
        // }
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = PAINTSTRUCT::default();
            let hdc: HDC = BeginPaint(hwnd, &mut ps);
            let h_brush: HBRUSH = CreateSolidBrush(rgbBlack);
            FillRect(hdc, &ps.rcPaint, h_brush);

            let pen: HPEN = CreatePen(PS_DASH, 1, rgbGreen);

            // SelectObject(hdc, pen);
            // SetBkColor(hdc, rgbBlue);
            SelectObject(hdc, pen);
            draw_line(hdc, window_width / 2, window_height / 2, 400, 400);

            draw_line(hdc, 400, 400, 500, 500);
            draw_line(hdc, 500, 500, 600, 400);
            draw_line(hdc, 400, 400, 500, 300);
            draw_line(hdc, 500, 300, 600, 400);
            draw_line(hdc, 400, 400, 400, 500);
            draw_line(hdc, 400, 500, 500, 600);
            draw_line(hdc, 500, 600, 600, 500);
            draw_line(hdc, 600, 400, 600, 500);
            draw_line(hdc, 500, 500, 500, 600);

            // SelectObject(hdc, pen);
            // SetBkColor(hdc, rgbBlue);
            // SelectObject(hdc, pen);
            // Rectangle(hdc, 100, 100, 100 + 2, 100 + 2);

            DeleteObject(h_brush);
            DeleteObject(pen);

            EndPaint(hwnd, &ps);
            return 0;
        }
        WM_SIZE => {
            let mut rect: RECT = RECT::default();

            if (GetWindowRect(hwnd, &mut rect)) {
                window_width = rect.right - rect.left;
                window_height = rect.bottom - rect.top;
                println!("width: {}, height: {}", window_width, window_height);
            }
            return 0;
        }
        WM_COMMAND => {
            match wparam as UINT {
                1 => {
                    MessageBoxW(
                        hwnd,
                        wide_null("Menu").as_ptr(),
                        wide_null("Menu").as_ptr(),
                        MB_OK,
                    );
                    return 0;
                }
                2 => {
                    // DestroyWindow(hwnd);
                    SendMessageW(hwnd, WM_CLOSE, 0, 0);
                    return 0;
                }
                _ => {
                    return DefWindowProcW(hwnd, msg, wparam, lparam);
                }
            }
        }
        WM_CLOSE => {
            if MessageBoxW(
                hwnd,
                wide_null("Are you sure you want to exit?").as_ptr(),
                wide_null("Exit prompt").as_ptr(),
                MB_YESNO,
            ) == IDYES
            {
                DestroyWindow(hwnd);
            }
            return 0;
        }
        WM_DESTROY => {
            PostQuitMessage(0);
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
        panic!(
            "Could not register the window class, error code: {}",
            last_error
        );
    }
    let window_name: Vec<u16> = wide_null("ServerManagerClient");
    let hwnd: HWND = unsafe {
        CreateWindowExW(
            WS_EX_LEFT,
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
        panic!(
            "Could not register the window class, error code: {}",
            last_error
        );
    }
    unsafe { ShowWindow(hwnd, SW_SHOW) };
    let mut msg: MSG = MSG::default();
    while unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) } != 0 {
        unsafe { TranslateMessage(&mut msg) };
        unsafe { DispatchMessageW(&mut msg) };
    }
}
