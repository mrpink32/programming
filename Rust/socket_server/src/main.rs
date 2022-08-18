use std::{net::{TcpStream, TcpListener}, io::{Result, BufReader, BufWriter, BufRead, Write}};

struct BufTcpStream {
    input: BufReader<TcpStream>,
    output: BufWriter<TcpStream>,
}

impl BufTcpStream {
    fn new(stream: TcpStream) -> Result<Self> {
        let input: BufReader<TcpStream> = BufReader::new(stream.try_clone()?);
        let output: BufWriter<TcpStream> = BufWriter::new(stream.try_clone()?);

        Ok(Self { input, output })
    }
}

fn main() {
    // start server socket
    let listener = TcpListener::bind("localhost:9000").expect("Could not bind to port");
    println!("Listening on port 9000");
    loop {
        let stream: TcpStream = match listener.accept() {
            Ok((stream, addr)) => {
                println!("new client: {addr:?}");
                stream
            },
            Err(e) => {
                println!("Error: {}", e);
                return
            },
        };


        let mut buf_stream = BufTcpStream::new(stream).unwrap();
        // let mut stream_reader: BufReader<TcpStream> = BufReader::new(stream.try_clone().unwrap());
        // let mut stream_writer: BufWriter<TcpStream> = BufWriter::new(stream.try_clone().unwrap());

        loop {
            let mut package: Vec<u8> = Vec::new();
            println!("Waiting for client to send data...");
            let bytes_read = BufReader::read_until(&mut buf_stream.input, '\n' as u8, &mut package)
                .expect("Failed to read from stream");
            println!("read: {} bytes to buffer", bytes_read);
            let message: String = String::from_utf8(package.to_vec())
                .expect("Could not convert package to string");
            print!("Received message: {}", message);
            buf_stream.output.write(message.as_bytes()).unwrap();
            // let write = BufWriter::write_all(&mut bufStream.output, &mut package);
        }
    }
}
