use image::{codecs::png::PngEncoder, ColorType, ImageEncoder};
use std::{env, fs::File};

trait Factorial {
    fn factorial(&self) -> f64;
}

impl Factorial for i32 {
    fn factorial(&self) -> f64 {
        let mut result: f64 = 1.0;
        for i in 1..=*self {
            result *= f64::from(i);
        }
        result
    }
}

fn taylor_sqrt(x: f64) -> f64 {
    let a0: f64 = 1.0; // sqrt(1) = 1.0
                       // sqrt(4) = 2.0
    let a1: f64 = 0.5 * ((x - 1_f64).powi(1) / 1.factorial()); // 1 / (2 * sqrt(1)) = 1/2 = 0.5
                                                               // let a1: f64 = 0.25 * ((x - 4_f64).powi(1) / 1.factorial()); // 1 / (2 * sqrt(4)) = 1/4 = 0.25
    let a2: f64 = -0.25 * ((x - 1_f64).powi(2) / 2.factorial()); // -1 / (4 * sqrt(1)^3) = -1/4 = -0.25
                                                                 // let a2: f64 = -0.03125 * ((x - 4_f64).powi(2) / 2.factorial()); // -1 / (4 * sqrt(4)^3) = -0.03125
    let a3: f64 = 0.375 * ((x - 1_f64).powi(3) / 3.factorial()); // 3 / (8 * sqrt(1)^5) = 3/8 = 0.375
                                                                 // let a3: f64 =  // 3 / (8 * sqrt(4)^5) = 0.375

    let result: f64 = a0 + a1 + a2 + a3;
    result
}

fn taylor_cos(x: f64, n: Option<i32>) -> f64 {
    let a0: f64 = 1.0; // cos(0) = 1
    let a1: f64 = 0.0 * (x.powi(1) / 1.factorial()); // -sin(0) = 0
    let a2: f64 = -1.0 * (x.powi(2) / 2.factorial()); // -0.5 * x.powf(2_f64); // -cos(0) = -1
    let a3: f64 = 0.0 * (x.powi(3) / 3.factorial()); // sin(0) = 0
    let a4: f64 = 1.0 * (x.powi(4) / 4.factorial()); // cos(0) = 1
    let a5: f64 = 0.0 * (x.powi(5) / 5.factorial()); // -sin(0) = 0
    let a6: f64 = -1.0 * (x.powi(6) / 6.factorial()); // -cos(0) = -1
    let a7: f64 = 0.0 * (x.powi(7) / 7.factorial()); // sin(0) = 0
    let a8: f64 = 1.0 * (x.powi(8) / 8.factorial()); // cos(0) = 1
    let a9: f64 = 0.0 * (x.powi(9) / 9.factorial()); // -sin(0) = 0
    let a10: f64 = -1.0 * (x.powi(10) / 10.factorial()); // -cos(0) = -1
    let a11: f64 = 0.0 * (x.powi(11) / 11.factorial()); // sin(0) = 0
    let a12: f64 = 1.0 * (x.powi(12) / 12.factorial()); // cos(0) = 1
    let a13: f64 = 0.0 * (x.powi(13) / 13.factorial()); // -sin(0) = 0
    let a14: f64 = -1.0 * (x.powi(14) / 14.factorial()); // -cos(0) = -1
    let a15: f64 = 0.0 * (x.powi(15) / 15.factorial()); // sin(0) = 0
    let result: f64 =
        a0 + a1 + a2 + a3 + a4 + a5 + a6 + a7 + a8 + a9 + a10 + a11 + a12 + a13 + a14 + a15;
    result
}

fn main() {
    println!("Taylor approx for cosine:");
    for i in 0..=20 {
        let x: f64 = i as f64 / 10_f64;
        println!("For x = {}", x);
        let a: f64 = x.cos();
        let b: f64 = taylor_cos(x, None);
        println!("cos(x): {}\napprox_cos(x): {}", a, b);
    }
    println!("Taylor approx for square root:");
    for i in 0..=20 {
        let x: f64 = i as f64 / 10_f64;
        println!("For x = {}", x);
        let a: f64 = x.sqrt();
        let b: f64 = taylor_sqrt(x);
        println!("sqrt(x): {}\napprox_sqrt(x): {}", a, b);
    }

    let args: Vec<String> = env::args().collect::<Vec<String>>();

    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS", args[0]);
        eprintln!("Example: {} mandel.png 1000x750", args[0]);
        std::process::exit(1);
    }

    // let bounds = parse_pair(&args[2], 'x').expect("Error parsing image dimensions!");
    // let upper_left: Complex<f64> =
    //     parse_complex(&args[3]).expect("Error parsing upper left corner point!");
    // let lower_right: Complex<f64> =
    //     parse_complex(&args[4]).expect("Error parsing lower right corner point!");

    // let mut pixels: Vec<u8> = vec![0; bounds.0 * bounds.1];

    // render(&mut pixels, bounds, upper_left, lower_right);

    // write_image(&args[1], &pixels, bounds).expect("Error writing PNG file!");
}

// fn pixel_to_point(
//     bounds: (usize, usize),
//     pixel: (usize, usize),
//     upper_left: Complex<f64>,
//     lower_right: Complex<f64>,
// ) -> Complex<f64> {
//     let (width, height) = (
//         lower_right.re - upper_left.re,
//         upper_left.im - lower_right.im,
//     );
//     Complex {
//         re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
//         im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
//     }
// }

// fn render(
//     pixels: &mut [u8],
//     bounds: (usize, usize),
//     upper_left: Complex<f64>,
//     lower_right: Complex<f64>,
// ) {
//     assert_eq!(pixels.len(), bounds.0 * bounds.1);
//     for row in 0..bounds.1 {
//         for column in 0..bounds.0 {
//             let point: Complex<f64> =
//                 pixel_to_point(bounds, (column, row), upper_left, lower_right);
//
//             pixels[row * bounds.0 + column] = match escape_time(point, 255) {
//                 None => 0,
//                 Some(count) => 255 - count as u8,
//             };
//         }
//     }
// }

fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let output: File = File::create(filename)?;

    let encoder: PngEncoder<File> = PngEncoder::new(output);
    encoder
        .write_image(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::L8)
        .expect("TODO: panic message");

    Ok(())
}
