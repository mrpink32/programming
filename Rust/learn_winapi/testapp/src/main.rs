#![allow(unused)]

use std::{
    f64::INFINITY,
    io::{BufRead, BufReader, BufWriter, Read, Result, Write},
    net::TcpStream,
    os::{raw::*, windows::raw::HANDLE},
    ptr::{null, null_mut},
    thread,
};
use winapi::*;

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

fn get_window_size(hwnd: HWND) -> (i32, i32) {
    let mut rect: RECT = RECT::default();
    let mut window_width: i32 = 0;
    let mut window_height: i32 = 0;
    if unsafe { GetWindowRect(hwnd, &mut rect) } != 0 {
        window_width = rect.right - rect.left;
        window_height = rect.bottom - rect.top;
        println!("width: {}, height: {}", window_width, window_height);
    }
    return (window_width, window_height);
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
            let (window_width, window_height): (i32, i32) = get_window_size(hwnd);
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
