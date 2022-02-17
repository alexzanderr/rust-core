



macro_rules! name {
    ($left:expr, $right:expr) => {
        let left = $left;
        let right = $right;
        println!("{left} {right}")
    };
}

fn print<T: std::fmt::Display>(arg: T) {
    println!("{}", arg);
}

// #[derive(Serizalize, Deserialize)]
// struct Person {
//     name: String,
//     age: u32,
// }

fn main() {
    name!("asd", "123");
    name!("asd", 123);
    name!(12, 123);
}