struct fraction {
    upper: f64,
    lower: f64,
}

impl fraction {
    fn from(a: f64) -> fraction {
        return fraction {
            upper: a.fract() * 100.0,
            lower: 100.0,
        };
    }
}

fn pow2(a: f64, b: f64) -> f64 {
    let b = fraction::from(b);
    // x = pow(b.lower.root(a), b.upper)
    println!("{}", pow((b.lower), b.upper));
    todo!("write code for performing tasks")
}

// x = a^b <-> log(x) = b * log(a)
fn pow(a: f64, b: f64) -> f64 {
    let mut result: f64 = a;
    for _ in 1..b.floor() as i32 {
        result = result * b;
    }
    println!("{}", (b - b.floor()));
    return result;
}

fn main() {
    println!("{}", pow(2.0, 2.5));
    assert_eq!(pow(2.0, 2.0), 4.0);
}
