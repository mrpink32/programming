use core::ptr::{null, null_mut};
use gtk::{
    prelude::*, subclass::prelude::*, Application, ApplicationWindow, Button, Orientation,
    Settings, Widget, Window,
};
use gtk4 as gtk;
use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::Graphics::Gdi::*,
    Win32::System::LibraryLoader::*,
    Win32::{
        System::Diagnostics::Debug::MessageBeep,
        UI::{Controls::LVITEMINDEX, WindowsAndMessaging::*},
    },
};

const PLAY_BUTTON: i32 = 1;
const APP_ID: &str = "org.app.Taskbar";

fn insert_list_view_items() -> BOOL {
    let test = LVITEMINDEX::default();
    return BOOL::default();
}

fn get_window_size(hwnd: HWND) -> (i32, i32) {
    let mut rect = RECT::default();
    let mut window_width = 0;
    let mut window_height = 0;
    if unsafe { GetWindowRect(hwnd, &mut rect) } != false {
        window_width = rect.right - rect.left;
        window_height = rect.bottom - rect.top;
        println!("width: {}, height: {}", window_width, window_height);
    }
    return (window_width, window_height);
}

fn rgb(r: u8, g: u8, b: u8) -> COLORREF {
    let color = COLORREF(r as u32 | (g as u32) << 8 | (b as u32) << 16);
    return color;
}

unsafe extern "system" fn window_procedure(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let hinstance = GetModuleHandleW(PCWSTR::null()).expect("Failed to get module handle!");
    match msg {
        WM_CREATE => {
            let (window_width, window_height) = get_window_size(hwnd);

            let play_button = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                w!("BUTTON"),
                PCWSTR::null(),
                WS_CHILD
                    | WS_VISIBLE
                    // | WINDOW_STYLE(BS_PUSHBUTTON as u32)
                    | WINDOW_STYLE(BS_FLAT as u32)
                    | WINDOW_STYLE(BS_ICON as u32), // BS_BITMAP
                5,
                5,
                40,
                40,
                hwnd,
                HMENU(PLAY_BUTTON as isize),
                hinstance,
                Some(null()),
            );
            if play_button == HWND::default() {
                let last_error = GetLastError();
                panic!(
                    "Could not create the play button, error code: {:?}",
                    last_error
                );
            }
            let icon = LoadImageW(
                hinstance,
                w!("src/play.ico"),
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

            // let prev_button = CreateWindowExW(
            //     WINDOW_EX_STYLE::default(),
            //     w!("BUTTON"),
            //     PCWSTR::null(),
            //     WS_CHILD
            //         | WS_VISIBLE
            //         // | WINDOW_STYLE(BS_PUSHBUTTON as u32)
            //         | WINDOW_STYLE(BS_FLAT as u32)
            //         | WINDOW_STYLE(BS_ICON as u32),
            //     window_width / 2 - 90,
            //     window_height - 100,
            //     40,
            //     40,
            //     hwnd,
            //     HMENU(PREV_BUTTON as isize),
            //     hinstance,
            //     Some(null()),
            // );
            // if prev_button == HWND::default() {
            //     let last_error: WIN32_ERROR = GetLastError();
            //     panic!(
            //         "Could not create the prev button, error code: {:?}",
            //         last_error
            //     );
            // }

            // let next_button = CreateWindowExW(
            //     WINDOW_EX_STYLE::default(),
            //     w!("BUTTON"),
            //     PCWSTR::null(),
            //     WS_CHILD
            //         | WS_VISIBLE
            //         // | WINDOW_STYLE(BS_PUSHBUTTON as u32)
            //         | WINDOW_STYLE(BS_FLAT as u32)
            //         | WINDOW_STYLE(BS_ICON as u32),
            //     window_width / 2,
            //     window_height - 100,
            //     40,
            //     40,
            //     hwnd,
            //     HMENU(NEXT_BUTTON as isize),
            //     hinstance,
            //     Some(null()),
            // );
            // if next_button == HWND::default() {
            //     let last_error: WIN32_ERROR = GetLastError();
            //     panic!(
            //         "Could not create the next button, error code: {:?}",
            //         last_error
            //     );
            // }

            return LRESULT(0);
        }
        // WM_SIZE => {
        // let (window_width, window_height): (i32, i32) = get_window_size(hwnd);

        // drop(SetWindowPos(
        //     GetDlgItem(hwnd, PLAY_BUTTON),
        //     HWND::default(),
        //     5,
        //     5,
        //     40,
        //     40,
        //     SWP_NOZORDER,
        // ));

        // drop(SetWindowPos(
        //     GetDlgItem(hwnd, PREV_BUTTON),
        //     HWND::default(),
        //     window_width / 2 + 10,
        //     window_height - 100,
        //     40,
        //     40,
        //     SWP_NOZORDER,
        // ));

        // drop(SetWindowPos(
        //     GetDlgItem(hwnd, NEXT_BUTTON),
        //     HWND::default(),
        //     window_width / 2 - 70,
        //     window_height - 100,
        //     40,
        //     40,
        //     SWP_NOZORDER,
        // ));

        // return LRESULT(0);
        // }
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

fn windows() {
    let hinstance =
        unsafe { GetModuleHandleW(PCWSTR::null()).expect("Failed to get module handle!") };
    let class_name = w!("Taskbar");
    let mut window_class = WNDCLASSW::default();
    window_class.style = WNDCLASS_STYLES::default();
    window_class.lpfnWndProc = Some(window_procedure);
    window_class.cbClsExtra = 0;
    window_class.cbWndExtra = 0;
    window_class.hInstance = hinstance;
    window_class.hIcon = HICON(
        unsafe {
            LoadImageW(
                hinstance,
                w!("src/test.ico"),
                IMAGE_ICON,
                0,
                0,
                LR_DEFAULTSIZE | LR_LOADFROMFILE,
            )
        }
        .expect("Failed to load image!")
        .0,
    );
    window_class.hCursor = HCURSOR::default();
    window_class.hbrBackground = unsafe { CreateSolidBrush(rgb(127, 127, 127)) };
    window_class.lpszMenuName = PCWSTR::null();
    window_class.lpszClassName = class_name;
    let atom: u16 = unsafe { RegisterClassW(&window_class) };
    if atom == 0 {
        let last_error = unsafe { GetLastError() };
        panic!(
            "Could not register the window class, error code: {:?}",
            last_error
        );
    }

    let hwnd = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            class_name,
            PCWSTR::null(),
            WINDOW_STYLE::default(),
            0,
            0,
            1920,
            50,
            HWND::default(),
            HMENU::default(),
            hinstance,
            Some(null()),
        )
    };
    if hwnd == HWND::default() {
        let last_error = unsafe { GetLastError() };
        panic!("Failed to create window, error code: {:?}", last_error);
    }

    unsafe {
        SetWindowLongPtrW(hwnd, GWL_STYLE, WS_POPUP.0 as isize);
        ShowWindow(hwnd, SW_SHOW);
        UpdateWindow(hwnd);
    }

    let mut msg = MSG::default();
    while unsafe { GetMessageW(&mut msg, HWND::default(), 0, 0) } != false {
        unsafe { TranslateMessage(&mut msg) };
        unsafe { DispatchMessageW(&mut msg) };
    }
}

fn linux() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(application: &Application) {
    // Create a button
    let button = Button::new();

    // Create a button
    let button1 = Button::new();

    // Add buttons to `gtk_box`
    let gtk_box = gtk4::Box::builder()
        .orientation(Orientation::Horizontal)
        .build();
    gtk_box.append(&button);
    gtk_box.append(&button1);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(application)
        .child(&gtk_box)
        .build();

    // window.set_display(Display::default());
    // window.set_window_position(WindowPosition::None);
    window.set_default_size(1920, 25);
    // window.set_skip_taskbar_hint(true);
    // window.set_skip_pager_hint(true);
    window.set_decorated(false);
    // window.set_keep_above(true);
    // window.move_(0, 0);
    window.set_resizable(false);
    window.set_size_request(1920, 25);

    // Present window
    window.present();
}

fn main() {
    linux();

    // match std::env::consts::OS {
    //     "windows" => windows(),
    //     "linux" => linux(),
    //     _ => {
    //         windows();
    //     }
    // }
}
