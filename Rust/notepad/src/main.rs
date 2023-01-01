use core::ffi::*;
use std::ptr::null_mut;
use winapi::*;

static mut TEXT_AREA: i32 = 0;
static mut BUTTON: i32 = 0;

fn get_window_size(hwnd: HWND) -> (i32, i32) {
    let mut rect: RECT = RECT::default();
    let mut window_width: i32 = 0;
    let mut window_height: i32 = 0;
    if unsafe { GetWindowRect(hwnd, &mut rect) } != 0 {
        window_width = rect.right - rect.left;
        window_height = rect.bottom - rect.top;
        println!("width: {}, height: {}", window_width, window_height);
    }
    (window_width, window_height)
}

unsafe extern "system" fn window_procedure(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_CREATE => {
            let menu: HMENU = CreateMenu();
            let file: HMENU = CreateMenu();

            AppendMenuW(menu, MF_POPUP, file as UINT, wide_null("Menu").as_ptr());
            AppendMenuW(menu, MF_STRING, 0, wide_null("File").as_ptr());
            AppendMenuW(menu, MF_STRING, 3, wide_null("Option").as_ptr());

            AppendMenuW(file, MF_STRING, 1, wide_null("connect").as_ptr());
            AppendMenuW(file, MF_SEPARATOR, 0, 0 as LPCWSTR);
            AppendMenuW(file, MF_STRING, 2, wide_null("Exit").as_ptr());

            SetMenu(hwnd, menu);
            return 0;
        }
        WM_SIZE => {
            let (window_width, window_height): (i32, i32) = get_window_size(hwnd);

            // let text_area = get_module_handle_w(wide_null("test").as_ptr());
            // let text_area: HMODULE = unsafe { GetModuleHandleW(wide_null("test").as_ptr()) };
            // if text_area.is_null() {
            //     panic!("Failed to get module handle: {}", unsafe { GetLastError() });
            // }
            drop(SetWindowPos(
                TEXT_AREA as HWND,
                null_mut(),
                50,
                0,
                window_width,
                window_height,
                SWP_NOZORDER,
            ));

            // let button = get_module_handle_w(wide_null("OK").as_ptr());
            // let button: HMODULE = unsafe { GetModuleHandleW(wide_null("OK").as_ptr()) };
            // if button.is_null() {
            //     panic!("Failed to get module handle: {}", unsafe { GetLastError() });
            // }
            drop(SetWindowPos(
                BUTTON as HWND,
                null_mut(),
                0,
                0,
                50,
                50,
                SWP_NOZORDER,
            ));
            return 0;
        }
        // WM_PAINT => {
        //     let mut ps: PAINTSTRUCT = PAINTSTRUCT::default();
        //     let hdc: HDC = BeginPaint(hwnd, &mut ps);
        //     let h_brush: HBRUSH = CreateSolidBrush(rgb(255, 255, 255));
        //     FillRect(hdc, &ps.rcPaint, h_brush);

        //     DeleteObject(h_brush);

        //     EndPaint(hwnd, &ps);
        // }
        WM_COMMAND => match wparam as UINT {
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
                SendMessageW(hwnd, WM_CLOSE, 0, 0);
                return 0;
            }
            3 => {
                SendMessageW(hwnd, WM_CLOSE, 0, 0);
                return 0;
            }
            _ => return DefWindowProcW(hwnd, msg, wparam, lparam),
        },
        WM_CLOSE => {
            MessageBeep(MB_OK);
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
        _ => return DefWindowProcW(hwnd, msg, wparam, lparam),
    };
}

fn main() {
    let hinstance: HINSTANCE = unsafe { GetModuleHandleW(null_mut()) };
    let class_name: Vec<u16> = wide_null("NoteWindowClass");
    let mut window_class: WNDCLASSW = WNDCLASSW::default();
    window_class.style = 0;
    window_class.lpfnWndProc = Some(window_procedure);
    window_class.cbClsExtra = 0;
    window_class.cbWndExtra = 0;
    window_class.hInstance = hinstance;
    window_class.hIcon = unsafe { LoadIconW(hinstance, wide_null("menu_one.ico").as_ptr()) };
    window_class.hCursor = null_mut();
    window_class.hbrBackground = unsafe { CreateSolidBrush(0x00ffffff) };
    window_class.lpszMenuName = null_mut();
    window_class.lpszClassName = class_name.as_ptr();
    let atom: ATOM = unsafe { RegisterClassW(&window_class) };
    if atom == 0 {
        let last_error: c_ulong = unsafe { GetLastError() };
        panic!(
            "Could not register the window class, error code: {}",
            last_error
        );
    }

    let hwnd: HWND = unsafe {
        CreateWindowExW(
            WS_EX_LEFT,
            class_name.as_ptr(),
            wide_null("notepad").as_ptr(),
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

    let (window_width, window_height): (i32, i32) = get_window_size(hwnd);

    unsafe {
        let text_area: HWND = CreateWindowExW(
            WS_EX_CLIENTEDGE,
            wide_null("Edit").as_ptr(),
            null_mut(),
            WS_CHILD | WS_VISIBLE,
            50,
            0,
            window_width,
            window_height,
            hwnd,
            null_mut(),
            hinstance,
            null_mut(),
        );
        if text_area.is_null() {
            let last_error: c_ulong = GetLastError();
            panic!("Could not create the textarea, error code: {}", last_error);
        }
        print!("{:?}", text_area);
        TEXT_AREA = text_area as i32
    }

    unsafe {
        let button: HWND = CreateWindowExW(
            0,
            wide_null("BUTTON").as_ptr(),
            wide_null("OK").as_ptr(),
            WS_TABSTOP | WS_VISIBLE | WS_CHILD | BS_PUSHBUTTON | BS_FLAT,
            0,
            0,
            50,
            50,
            hwnd,
            null_mut(),
            hinstance,
            null_mut(),
        );
        if button.is_null() {
            let last_error: c_ulong = GetLastError();
            panic!("Could not create the textarea, error code: {}", last_error);
        }
        BUTTON = button as i32;
    }

    unsafe { ShowWindow(hwnd, SW_SHOW) };
    unsafe { UpdateWindow(hwnd) };

    let mut msg: MSG = MSG::default();
    while unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) } != 0 {
        unsafe { TranslateMessage(&mut msg) };
        unsafe { DispatchMessageW(&mut msg) };
    }
}
