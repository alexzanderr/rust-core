

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

fn main() {

}