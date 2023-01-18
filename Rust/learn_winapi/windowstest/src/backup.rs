use core::ptr::null;
use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::Graphics::Gdi::*,
    Win32::System::LibraryLoader::*,
    Win32::{System::Diagnostics::Debug::MessageBeep, UI::WindowsAndMessaging::*},
};

static mut TEXT_AREA: isize = 0;
static mut BUTTON: isize = 0;

fn get_window_size(hwnd: HWND) -> (i32, i32) {
    let mut rect: RECT = RECT::default();
    let mut window_width: i32 = 0;
    let mut window_height: i32 = 0;
    if unsafe { GetWindowRect(hwnd, &mut rect) } != false {
        window_width = rect.right - rect.left;
        window_height = rect.bottom - rect.top;
        println!("width: {}, height: {}", window_width, window_height);
    }
    return (window_width, window_height);
}

pub fn rgb(r: u8, g: u8, b: u8) -> COLORREF {
    let color: COLORREF = COLORREF(r as u32 | (g as u32) << 8 | (b as u32) << 16);
    return color;
}

unsafe extern "system" fn window_procedure(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_CREATE => {
            let menu: HMENU = CreateMenu().expect("Failed to create menu!");
            let file: HMENU = CreateMenu().expect("Failed to create menu!");

            AppendMenuW(menu, MF_POPUP, file.0 as usize, w!("Menu"));
            AppendMenuW(menu, MF_STRING, 0, w!("File"));
            AppendMenuW(menu, MF_STRING, 3, w!("Option"));

            AppendMenuW(file, MF_STRING, 1, w!("connect"));
            AppendMenuW(file, MF_SEPARATOR, 0, PCWSTR::null());
            AppendMenuW(file, MF_STRING, 2, w!("Exit"));

            SetMenu(hwnd, menu);
            return LRESULT(0);
        }
        WM_SIZE => {
            let (window_width, window_height): (i32, i32) = get_window_size(hwnd);

            drop(SetWindowPos(
                HWND(TEXT_AREA),
                HWND::default(),
                50,
                0,
                window_width,
                window_height,
                SWP_NOZORDER,
            ));

            drop(SetWindowPos(
                HWND(BUTTON),
                HWND::default(),
                0,
                0,
                50,
                50,
                SWP_NOZORDER,
            ));
            return LRESULT(0);
        }
        // WM_PAINT => {
        //     let mut ps: PAINTSTRUCT = PAINTSTRUCT::default();
        //     let hdc: HDC = BeginPaint(hwnd, &mut ps);
        //     let h_brush: HBRUSH = CreateSolidBrush(rgb(255, 255, 255));
        //     FillRect(hdc, &ps.rcPaint, h_brush);

        //     DeleteObject(h_brush);

        //     EndPaint(hwnd, &ps);
        // }
        WM_COMMAND => match wparam.0 {
            1 => {
                MessageBoxW(hwnd, w!("Menu"), w!("Menu"), MB_OK);
                return LRESULT(0);
            }
            2 => {
                SendMessageW(hwnd, WM_CLOSE, wparam, lparam);
                return LRESULT(0);
            }
            3 => {
                SendMessageW(hwnd, WM_CLOSE, wparam, lparam);
                return LRESULT(0);
            }
            _ => return DefWindowProcW(hwnd, msg, wparam, lparam),
        },
        WM_CLOSE => {
            MessageBeep(MB_OK);
            if MessageBoxW(
                hwnd,
                w!("Are you sure you want to exit?"),
                w!("Exit prompt"),
                MB_YESNO,
            ) == IDYES
            {
                DestroyWindow(hwnd);
            }
            return LRESULT(0);
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            return LRESULT(0);
        }
        _ => return DefWindowProcW(hwnd, msg, wparam, lparam),
    };
}

fn main() {
    let hinstance: HINSTANCE =
        unsafe { GetModuleHandleW(PCWSTR::null()).expect("Failed to get module handle!") };
    let class_name: PCWSTR = w!("ServerManagerWindowClass");
    let mut window_class: WNDCLASSW = WNDCLASSW::default();
    window_class.style = WNDCLASS_STYLES::default();
    window_class.lpfnWndProc = Some(window_procedure);
    window_class.cbClsExtra = 0;
    window_class.cbWndExtra = 0;
    window_class.hInstance = hinstance;
    window_class.hIcon = HICON::default();
    window_class.hCursor = HCURSOR::default();
    window_class.hbrBackground = unsafe { CreateSolidBrush(rgb(255, 255, 255)) };
    window_class.lpszMenuName = PCWSTR::null();
    window_class.lpszClassName = class_name;
    let atom: u16 = unsafe { RegisterClassW(&window_class) };
    if atom == 0 {
        let last_error: WIN32_ERROR = unsafe { GetLastError() };
        panic!(
            "Could not register the window class, error code: {:?}",
            last_error
        );
    }

    let hwnd: HWND = unsafe {
        CreateWindowExW(
            WS_EX_LEFT,
            class_name,
            w!("Test"),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            HWND::default(),
            HMENU::default(),
            hinstance,
            Some(null()),
        )
    };
    if hwnd == HWND::default() {
        let last_error: WIN32_ERROR = unsafe { GetLastError() };
        panic!("Failed to create window, error code: {:?}", last_error);
    }

    let (window_width, window_height): (i32, i32) = get_window_size(hwnd);
    unsafe {
        let text_area: HWND = CreateWindowExW(
            WS_EX_CLIENTEDGE,
            w!("EDIT"),
            PCWSTR::null(),
            WS_CHILD | WS_VISIBLE | WINDOW_STYLE(ES_MULTILINE as u32),
            50,
            0,
            window_width,
            window_height,
            hwnd,
            HMENU::default(),
            hinstance,
            Some(null()),
        );
        if text_area == HWND::default() {
            let last_error: WIN32_ERROR = GetLastError();
            panic!(
                "Could not create the textarea, error code: {:?}",
                last_error
            );
        }
        TEXT_AREA = text_area.0;
    }

    unsafe {
        let button: HWND = CreateWindowExW(
            WS_EX_CLIENTEDGE,
            w!("BUTTON"),
            w!("OK"),
            WS_TABSTOP
                | WS_VISIBLE
                | WS_CHILD
                | WINDOW_STYLE(BS_PUSHBUTTON as u32)
                | WINDOW_STYLE(BS_FLAT as u32),
            0,
            0,
            50,
            50,
            hwnd,
            HMENU::default(),
            hinstance,
            Some(null()),
        );
        if button == HWND::default() {
            let last_error: WIN32_ERROR = GetLastError();
            panic!(
                "Could not create the textarea, error code: {:?}",
                last_error
            );
        }
        BUTTON = button.0;
    }

    unsafe { ShowWindow(hwnd, SW_SHOW) };
    unsafe { UpdateWindow(hwnd) };

    let mut msg: MSG = MSG::default();
    while unsafe { GetMessageW(&mut msg, HWND::default(), 0, 0) } != false {
        unsafe { TranslateMessage(&mut msg) };
        unsafe { DispatchMessageW(&mut msg) };
    }
}
