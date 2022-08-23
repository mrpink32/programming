use std::env;

fn main() {
    let test_args: Vec<String> = env::args().collect();
    dbg!(test_args);
}
