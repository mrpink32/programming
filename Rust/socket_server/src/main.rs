use std::{
    io::{BufRead, BufReader, BufWriter, Read, Result, Write},
    net::{TcpListener, TcpStream},
};

// constants
const PORT: &str = "9000";
const IP: &str = "localhost";

// define buffered TcpStreamW
struct BufTcpStream {
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
}

// implement new for buffered TcpStream
impl BufTcpStream {
    fn new(stream: TcpStream) -> Result<Self> {
        let reader: BufReader<TcpStream> = BufReader::new(stream.try_clone()?);
        let writer: BufWriter<TcpStream> = BufWriter::new(stream.try_clone()?);

        Ok(Self { reader, writer })
    }
}

fn main() {
    // start server socket and bind to address
    let address: String = String::from(format!("{IP}:{PORT}"));
    let listener = TcpListener::bind(address).expect("Could not bind to port");
    println!("Listening on port: {PORT}");
    // loop until a connection is made
    loop {
        // wait for connection to accept or run the loop again
        let stream: TcpStream = match listener.accept() {
            Ok((stream, addr)) => {
                println!("new client: {addr:?}");
                stream
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        };

        // create buffered TcpStream
        let mut buf_stream: BufTcpStream = BufTcpStream::new(stream)
            .expect("Failed to create buffered stream from networkstream!");

        // echo loop
        loop {
            // create vector to hold incoming packet
            let mut packet: Vec<u8> = Vec::new();
            println!("Waiting for client to send data...");
            // read buffer until newline
            let bytes_read: usize = {
                buf_stream
                    .reader
                    .read_until('\n' as u8, &mut packet)
                    .expect("test")
            };
            // exit loop if
            if bytes_read < 1 {
                break;
            }
            println!("read: {} bytes from buffer", bytes_read);

            // create a string from packet
            let message: String =
                String::from_utf8(packet).expect("Could not convert package to string");
            print!("Received message: {}", message);
            // write the received message to client and flush writer
            let bytes_written: usize = buf_stream.writer.write(message.as_bytes()).unwrap();
            println!("Returned: {bytes_written} bytes to client");
            buf_stream.writer.flush().unwrap();
        }
    }
}
