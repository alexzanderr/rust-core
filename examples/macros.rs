

#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_macros,
    unused_assignments
)]


macro_rules! _print {
    () => {
        println!();
    };
    ($argument: expr) => {
        println!("{}", format!("{}", $argument));
    };
    ($argument: expr, $($arguments: expr), +) => {
        print!("{}", format!("{} ", $argument));
        _print!($($arguments), +);
    };
}

// `find_min!` will calculate the minimum of any number of arguments.
macro_rules! find_min {
    // Base case:
    ($x:expr) => ($x);
    // `$x` followed by at least one `$y,`
    ($x:expr, $($y:expr),+) => (
        // Call `find_min!` on the tail `$y`
        std::cmp::min($x, find_min!($($y),+))
    )
}


// fn print<T: std::fmt::Display>(arg: T) {
//     println!("{}", arg);
// }

// this thing doesnt compile, because
// variadic generics in functions doesnt exist
// fn make_something<...T: i32>(integer: (...T)) {

// }

// #[derive(Serizalize, Deserialize)]
// struct Person {
//     name: String,
//     age: u32,
// }


use std::fmt::Display;
use std::fmt::Result;
use std::fmt::Formatter;

struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn dot_method(&self) {
        _print!("i was run using 'object.metod()'");
    }

    fn how_about_this(&self) {
        _print!("i was NOT run using 'object.metod()'");
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}



fn main() {
    // println!("{}", find_min!(1u32));
    // println!("{}", find_min!(1u32 + 2, 2u32));
    // println!("{}", find_min!(5u32, 2u32 * 3, 4u32));
    let var = 123;
    _print!("asd");
    _print!("asd", "asd");
    _print!("asd", "asd", var);
    let origin = Point { x: 0, y: 0 };
    _print!(origin, origin, origin, origin);
    _print!(123i32, 123.123f64, origin);


    let x = 723;
    let y = 23458;


    // f-string expression doesnt work
    _print!("{x} - {y}");
    _print!();
    _print!("---------");
    origin.dot_method();
    origin.how_about_this();


}