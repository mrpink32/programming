use std::{
    io::{stdin, BufRead, BufReader, BufWriter, Result, Write},
    net::TcpStream,
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

fn main() {
    let stream: TcpStream = match TcpStream::connect("localhost:9000") {
        Ok(stream) => {
            println!("Connected to the server!");
            stream
        }
        Err(e) => {
            println!("Couldn't connect to server: {}", e);
            return;
        }
    };

    let mut buf_stream: BufTcpStream = BufTcpStream::new(stream.try_clone().unwrap())
        .expect("Failed to create buffered stream from networkstream!");

    loop {
        // get input from user
        let mut input: String = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        print!("You wrote: {}", input);

        // write input to networkstream
        let bytes_written: usize = buf_stream
            .writer
            .write(input.as_bytes())
            .expect("Failed to write to stream");
        println!("Wrote: {} bytes to stream", bytes_written);
        buf_stream.writer.flush().unwrap();

        // create a buffer to store incomming bytes
        let mut package: Vec<u8> = Vec::new();
        println!("Waiting for server response...");

        // read respons from networkstream and prints to console
        buf_stream
            .reader
            .read_until('\n' as u8, &mut package)
            .expect("Failed to read from stream");
        let message: String =
            String::from_utf8(package.to_vec()).expect("Could not convert package to string");
        print!("Echo of message: {}", message);
    }
}
