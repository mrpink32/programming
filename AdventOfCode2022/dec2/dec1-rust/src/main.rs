use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    println!("-----part1-----");
    let data = File::open(Path::new("../data")).unwrap();
    let mut bufreader = BufReader::new(data);

    let mut line = String::new();
    bufreader.read_line(&mut line);
    println!("{}", line);
}
