use std::{
    io::{BufRead, BufReader, BufWriter, Read, Result, Write},
    net::{TcpListener, TcpStream},
};

const PORT: &str = "9000";
const IP: &str = "localhost";

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

fn main() {
    // start server socket
    let address: String = String::from(format!("{IP}:{PORT}"));
    let listener = TcpListener::bind(address).expect("Could not bind to port");
    println!("Listening on port {PORT}");
    loop {
        let mut stream: TcpStream = match listener.accept() {
            Ok((stream, addr)) => {
                println!("new client: {addr:?}");
                stream
            }
            Err(e) => {
                println!("Error: {}", e);
                return;
            }
        };

        let mut buf_stream: BufTcpStream = BufTcpStream::new(stream.try_clone().unwrap())
            .expect("Failed to create buffered stream from networkstream!");

        loop {
            let mut package: Vec<u8> = Vec::new();
            println!("Waiting for client to send data...");
            // let mut buf_stream: BufReader<&mut TcpStream> = BufReader::new(&mut stream);
            let bytes_read: usize = {
                buf_stream
                    .reader
                    .read_until('\n' as u8, &mut package)
                    .expect("test")
            };
            println!("read: {} bytes from buffer", bytes_read);

            let message: String =
                String::from_utf8(package.to_vec()).expect("Could not convert package to string");
            print!("Received message: {}", message);
            let bytes_written: usize = buf_stream.writer.write(message.as_bytes()).unwrap();
            // let bytes_written: usize = stream.write(message.as_bytes()).unwrap();
            println!("Returned: {bytes_written} bytes to client");
            buf_stream.writer.flush().unwrap();
            // let write = BufWriter::write_all(&mut bufStream.output, &mut package);
        }
    }
}
