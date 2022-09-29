type Vector = Vec<f64>;
type Matrix = Vec<Vector>;

trait Test {
    fn add(&mut self, b: Vector) -> Vector;
}

impl Test for Vector {
    fn add(&mut self, b: Vector) -> Vector {
        if self.len() == b.len() {
            for i in 0..self.len() {
                self[i] += b[i];
            }
        }
        panic!("Calculation of vector addition failed!")
    }
}

fn main() {
    let mut test1: Vector = vec![1.0, 2.0, 3.0];
    let test2: Vector = vec![3.0, 2.0, 1.0];
    test1.add(test2);
    println!("{:#?}", test1);
}
