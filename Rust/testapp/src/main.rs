#![allow(unused)]

pub mod win_api {
    use std::os::{raw::*, windows::raw::HANDLE};

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
    pub type BYTE = u8;
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
        unsafe extern "system" fn(
            hwnd: HWND,
            uMsg: UINT,
            wParam: WPARAM,
            lParam: LPARAM,
        ) -> LRESULT,
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
        /// ['GetMessageW'](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew)
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
        pub fn MessageBoxW(hWnd: HWND, lpText: LPCWSTR, lpCaption: LPCWSTR, uType: c_uint)
            -> c_int;
        // ['CreateMenu'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createmenu)
        pub fn CreateMenu() -> HMENU;
        // ['AppendMenuW'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-appendmenuw)
        pub fn AppendMenuW(
            hMenu: HMENU,
            uFlags: UINT,
            uIDNewItem: UINT,
            lpNewItem: LPCWSTR,
        ) -> BOOL;
        // ['SetMenu'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setmenu)
        pub fn SetMenu(hWnd: HWND, hMenu: HMENU) -> BOOL;
        // ['GetCursorPos'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getcursorpos)
        pub fn GetCursorPos(lpPoint: LPPOINT) -> BOOL;
        // ['ScreenToClient'] (https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-screentoclient)
        pub fn ScreenToClient(hwnd: HWND, lpPoint: LPPOINT) -> BOOL;
    }
}

use {
    std::{
        f64::INFINITY,
        io::{BufRead, BufReader, BufWriter, Read, Result, Write},
        net::TcpStream,
        os::{raw::*, windows::raw::HANDLE},
        ptr::{null, null_mut},
        thread,
    },
    win_api::*,
    // winapi::*
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

pub fn wide_null(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}

pub unsafe fn draw_line(hdc: HDC, x1: i32, y1: i32, x2: i32, y2: i32) {
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
                    'connection_loop: loop {
                        let stream: TcpStream = match TcpStream::connect("localhost:9000") {
                            Ok(stream) => {
                                println!("Connected to the server!");
                                stream
                            }
                            Err(e) => {
                                println!("Couldn't connect to server: {}", e);
                                continue 'connection_loop;
                            }
                        };
                        let mut buf_stream: BufTcpStream =
                            BufTcpStream::new(stream.try_clone().unwrap())
                                .expect("Failed to create buffered stream from networkstream!");
                        loop {
                            // get cursor position
                            let mut mouse_pos: POINT = POINT::default();
                            GetCursorPos(&mut mouse_pos);
                            // convert screen position to window position
                            // drop(ScreenToClient(hwnd, &mut mouse_pos));

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
            // draw_line(hdc, window_width / 2, window_height / 2, 400, 400);

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
            let mut window_width: i32 = 0;
            let mut window_height: i32 = 0;

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
