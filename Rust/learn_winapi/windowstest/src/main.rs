use core::ptr::{null, null_mut};
use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::Graphics::Gdi::*,
    Win32::Storage::FileSystem::*,
    Win32::System::LibraryLoader::*,
    Win32::System::Memory::*,
    Win32::{System::Diagnostics::Debug::MessageBeep, UI::WindowsAndMessaging::*},
};

const LIST_VIEW: i32 = 1;
const PLAY_BUTTON: i32 = 2;
const PREV_BUTTON: i32 = 3;
const NEXT_BUTTON: i32 = 4;

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

fn rgb(r: u8, g: u8, b: u8) -> COLORREF {
    let color: COLORREF = COLORREF(r as u32 | (g as u32) << 8 | (b as u32) << 16);
    return color;
}

unsafe extern "system" fn window_procedure(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let hinstance: HINSTANCE =
        GetModuleHandleW(PCWSTR::null()).expect("Failed to get module handle!");
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

            let (window_width, window_height): (i32, i32) = get_window_size(hwnd);

            let list_view: HWND = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                w!("SysListView32"),
                PCWSTR::null(),
                WS_CHILD
                    | WS_VISIBLE
                    | WS_VSCROLL
                    | WINDOW_STYLE(ES_MULTILINE as u32)
                    | WINDOW_STYLE(ES_AUTOVSCROLL as u32),
                0,
                0,
                window_width,
                window_height - 90,
                hwnd,
                HMENU(LIST_VIEW as isize),
                hinstance,
                Some(null()),
            );
            if list_view == HWND::default() {
                let last_error: WIN32_ERROR = GetLastError();
                panic!(
                    "Could not create the list view, error code: {:?}",
                    last_error
                );
            }

            let play_button: HWND = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                w!("BUTTON"),
                PCWSTR::null(),
                WS_CHILD
                    | WS_VISIBLE
                    // | WINDOW_STYLE(BS_PUSHBUTTON as u32)
                    | WINDOW_STYLE(BS_FLAT as u32)
                    | WINDOW_STYLE(BS_ICON as u32), // BS_BITMAP
                window_width / 2 - 50,
                window_height - 100,
                40,
                40,
                hwnd,
                HMENU(PLAY_BUTTON as isize),
                hinstance,
                Some(null()),
            );
            if play_button == HWND::default() {
                let last_error: WIN32_ERROR = GetLastError();
                panic!(
                    "Could not create the play button, error code: {:?}",
                    last_error
                );
            }
            let icon: HANDLE = LoadImageW(
                hinstance,
                w!("D:/programming/Rust/learn_winapi/windowstest/src/play.ico"),
                IMAGE_ICON,
                0,
                0,
                LR_DEFAULTSIZE | LR_LOADFROMFILE,
            )
            .expect("Failed to load image!");
            SendMessageW(
                play_button,
                BM_SETIMAGE,
                WPARAM(IMAGE_ICON.0 as usize),
                LPARAM(icon.0),
            );

            let prev_button: HWND = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                w!("BUTTON"),
                PCWSTR::null(),
                WS_CHILD
                        | WS_VISIBLE
                        // | WINDOW_STYLE(BS_PUSHBUTTON as u32)
                        | WINDOW_STYLE(BS_FLAT as u32)
                        | WINDOW_STYLE(BS_ICON as u32),
                window_width / 2 - 90,
                window_height - 100,
                40,
                40,
                hwnd,
                HMENU(PREV_BUTTON as isize),
                hinstance,
                Some(null()),
            );
            if prev_button == HWND::default() {
                let last_error: WIN32_ERROR = GetLastError();
                panic!(
                    "Could not create the prev button, error code: {:?}",
                    last_error
                );
            }

            let next_button: HWND = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                w!("BUTTON"),
                PCWSTR::null(),
                WS_CHILD
                    | WS_VISIBLE
                    // | WINDOW_STYLE(BS_PUSHBUTTON as u32)
                    | WINDOW_STYLE(BS_FLAT as u32)
                    | WINDOW_STYLE(BS_ICON as u32),
                window_width / 2,
                window_height - 100,
                40,
                40,
                hwnd,
                HMENU(NEXT_BUTTON as isize),
                hinstance,
                Some(null()),
            );
            if next_button == HWND::default() {
                let last_error: WIN32_ERROR = GetLastError();
                panic!(
                    "Could not create the next button, error code: {:?}",
                    last_error
                );
            }

            return LRESULT(0);
        }
        WM_SIZE => {
            let (window_width, window_height): (i32, i32) = get_window_size(hwnd);

            drop(SetWindowPos(
                GetDlgItem(hwnd, LIST_VIEW),
                HWND::default(),
                0,
                0,
                window_width,
                window_height - 100,
                SWP_NOZORDER,
            ));

            drop(SetWindowPos(
                GetDlgItem(hwnd, PLAY_BUTTON),
                HWND::default(),
                window_width / 2 - 30,
                window_height - 100,
                40,
                40,
                SWP_NOZORDER,
            ));

            drop(SetWindowPos(
                GetDlgItem(hwnd, PREV_BUTTON),
                HWND::default(),
                window_width / 2 + 10,
                window_height - 100,
                40,
                40,
                SWP_NOZORDER,
            ));

            drop(SetWindowPos(
                GetDlgItem(hwnd, NEXT_BUTTON),
                HWND::default(),
                window_width / 2 - 70,
                window_height - 100,
                40,
                40,
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

    unsafe { ShowWindow(hwnd, SW_SHOW) };
    unsafe { UpdateWindow(hwnd) };

    let mut msg: MSG = MSG::default();
    while unsafe { GetMessageW(&mut msg, HWND::default(), 0, 0) } != false {
        unsafe { TranslateMessage(&mut msg) };
        unsafe { DispatchMessageW(&mut msg) };
    }
}
