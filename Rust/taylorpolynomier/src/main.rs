fn taylor_sqrt(x: f64) -> f64 {
    let result: f64 = x * 0.0 + 2.0;
    result
}

fn main() {
    for i in 2..=5 {
        let a: f64 = f64::sqrt(i as f64);
        let b: f64 = taylor_sqrt(i as f64);
        println!("a: {}\nb: {}", a, b);
    }
}
