use std::{net::TcpStream, io::{Result, BufReader, BufWriter, BufRead, Write}};
use std::io;

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
    // if let Ok(stream) = TcpStream::connect("127.0.0.1:9000") {
    //     println!("Connected to the server!");
    // } else {
    //     println!("Couldn't connect to server...");
    // }
    let mut stream: TcpStream = match TcpStream::connect("localhost:9000") {
        Ok(stream) => {
            println!("Connected to the server!");
            stream
        },
        Err(e) => {
            println!("Couldn't connect to server: {}", e);
            return
        },
    };


    
    loop {
        // get input from user
        let mut input: String = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        print!("You wrote: {}", input);

        // write input to networkstream
        let byes_written = stream.write(input.as_bytes())
            .expect("Failed to write to stream");
        println!("Wrote: {} bytes to stream", byes_written);

        // create a buffer to store incomming bytes
        let package: Vec<u8> = Vec::new();
        println!("buffer created");

        // read respons from networkstream and prints to console
        // stream.read_to_end(&mut package)
        //     .expect("Failed to read from stream");
        // println!("{}", String::from_utf8(package).unwrap());
    }
}