

extern crate color_backtrace;



fn main() {
    let base: i32 = 2;

    color_backtrace::install();
    panic!("just panicked from main")
}