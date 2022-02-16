use std::ops::Index;

trait Foo<T>: Index<usize, Output = T>
where
    T: std::fmt::Display,
{
    fn foo(&self) {
        println!("Generic: {}", self[0]);
    }
}

impl Foo<f32> for Vec<f32> {}

impl Foo<f64> for Vec<f64> {
    fn foo(&self) {
        println!("Specialized: {}", self[0]);
    }
}

fn main() {
    vec![1.2_f32, 2.3].foo();
    vec![3.4_f64, 4.5].foo();
    println!("asd");
}