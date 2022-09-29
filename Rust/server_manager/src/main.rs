#![allow(unused_variables)]
use {
    core::ffi::*,
    std::{
        io::{stdin, stdout, BufRead, BufReader, BufWriter, Error, Result, Write},
        net::{TcpListener, TcpStream},
        process::{Child, Command},
        ptr::{null, null_mut},
        str::{SplitTerminator, SplitWhitespace},
    },
};

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
pub struct DISPLAY {}
unsafe_impl_default_zeroed!(DISPLAY);

// #[link(name = "X11/Xlib")]
extern "system" {
    // pub fn XQueryColors(
    //     display: *mut Display,
    //     colormap: Colormap,
    //     colors_in_out: XcmsColor[],
    //     ncolors: c_uint,
    //     result_format: XcmscolorFormat,
    // );

    // pub fn XOpenDisplay(

    // ) -> mut *DISPLAY;
    // pub fn XWarpPointer(
    //     display: mut *DISPLAY,
    //     src_w: c_void,
    //     dest_w: c_void,
    //     src_x: i32,
    //     src_y: i32,
    //     src_width: u32,
    //     src_height: u32,
    //     dest_x: i32,
    //     dest_y: i32,
    // );
}

const PORT: &str = "9000";
const IP: &str = "192.168.0.14"; // "localhost";
const CURRENT_OS: &str = std::env::consts::OS;
const MAX_CONNECTIONS: u8 = 1;

struct BufferedTcpStream {
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
}

impl BufferedTcpStream {
    fn new(stream: TcpStream) -> Result<Self> {
        let reader: BufReader<TcpStream> = BufReader::new(stream.try_clone()?);
        let writer: BufWriter<TcpStream> = BufWriter::new(stream.try_clone()?);

        Ok(Self { reader, writer })
    }
}

fn main() {
    // start server socket
    let address: String = String::from(format!("{IP}:{PORT}"));
    let listener: TcpListener = TcpListener::bind(address).expect("Could not bind to port");
    println!("Listening on port: {PORT}");

    loop {
        let stream: TcpStream = match listener.accept() {
            Ok((stream, addr)) => {
                println!("new client: {addr:?}");
                stream
            }
            Err(e) => {
                println!("Error: {}", e);
                return;
            }
        };

        // let mut stream_writer: BufWriter<TcpStream> = BufWriter::new(stream.try_clone().unwrap());
        // let mut stream_reader: BufReader<TcpStream> = BufReader::new(stream.try_clone().unwrap());
        let mut buffered_stream: BufferedTcpStream =
            BufferedTcpStream::new(stream) // .try_clone().unwrap()
                .expect("Failed to create buffered stream from networkstream!");

        loop {
            let mut package: Vec<u8> = Vec::new();
            println!("Waiting for client to send data...");
            let bytes_read: usize =
                match buffered_stream.reader.read_until('\n' as u8, &mut package) {
                    Ok(bytes_read) => bytes_read,
                    Err(e) => {
                        println!("Error: {}", e);
                        break;
                    }
                };
            println!("read: {} bytes to buffer", bytes_read);
            let message: String =
                String::from_utf8(package.to_vec()).expect("Could not convert package to string");
            print!("Received message: {}", message);
            // let data = message.split_terminator(",");
            // println!("{}", data.collect());
            // let mut mouse_x;
            // let mut mouse_y;
            // let mut (mouse_x, mouse_y) = data.next().unwrap().parse::<i32>().unwrap();
            // let bytes_written: usize = buf_stream.writer.write(message.as_bytes()).unwrap();
            // // let bytes_written: usize = stream.write(message.as_bytes()).unwrap();
            // println!("Returned: {bytes_written} bytes to client");
            // buf_stream.writer.flush().unwrap();
            // // let write = BufWriter::write_all(&mut bufStream.output, &mut package);
        }
    }

    match CURRENT_OS {
        "linux" => {}
        _ => {}
    }

    // loop {
    //     print!("enter command: ");
    //     stdout().flush().expect("failed to flush stdout");

    //     let mut input: String = String::new();
    //     stdin().read_line(&mut input).expect("Failed to read line");

    //     let mut parts: SplitWhitespace = input.trim().split_whitespace();
    //     let command: &str = parts.next().expect("No command");
    //     let args: SplitWhitespace = parts;

    //     match command {
    //         "cd" => {
    //             todo!("change directory");
    //         },
    //         "exit" => { return; },
    //         _ => {
    //             let child: Result<Child, Error> = Command::new(command)
    //                 .args(args)
    //                 .spawn();
    //             match child {
    //                 Ok(mut child) => {
    //                     drop(child.wait());
    //                     // todo!("handle child exit status");
    //                 },
    //                 Err(e) => eprintln!("{}", e),
    //             };
    //         }
    //     }
    // }
}
