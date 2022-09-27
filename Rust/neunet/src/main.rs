type vector = Vec<f64>;
type matrix = Vec<vector>;

trait Test {
    fn add(&mut self, b: vector) -> vector;
}

impl Test for vector {
    fn add(&mut self, b: vector) -> vector {
        if self.len() == b.len() {
            for i in 0..self.len() {
                self[i] += b[i];
            }
        }
        panic!("Calculation of vector addition failed!")
    }
}

fn main() {
    let mut test1: vector = vec![1.0, 2.0, 3.0];
    let test2: vector = vec![3.0, 2.0, 1.0];
    test1.add(test2);
    println!("{:#?}", test1);
}
