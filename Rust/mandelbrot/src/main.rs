use num::Complex;
use std::env;
use std::str::FromStr;

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z: Complex<f64> = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<T, T> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

fn main() {
    println!("Hello, world!");
    let mut numbers: Vec<f64> = Vec::new();
    for arg in env::args().skip(1) {
        numbers.push(arg.parse::<f64>().expect("Failed conversion!"));
    }

    if numbers.len() == 0 {
        eprintln!("No arguments given...");
        std::process::exit(1);
    }

    let mut d: f64 = numbers[0];
    for m in &mut numbers[1..] {
        d = d + *m;
    }
    println!("{}", d);
}
