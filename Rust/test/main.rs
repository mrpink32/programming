use std::io;

fn main() {
    let unsigned_value: u8 = 255;
    let signed_value: i8 = 127;
    let float32_value: f32 = 32.00000;
    let float64_value: f64 = 64.00000;
    let bool_value: bool = true;
    let mut sentence = String::from("Hello world!");
    if bool_value {
        println!("Unsigned value: {}", unsigned_value);
        println!("Signed value: {}", signed_value);
        println!("float32 value: {float32_value:.5}");
        println!("float64 value: {float64_value:.5}");
        io::stdin()
            .read_line(&mut sentence)
            .expect("Failed to read line");
        println!("String: {sentence}");
    }
}