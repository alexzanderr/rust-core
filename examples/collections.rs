

#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_macros,
    unused_assignments,
    unused_mut,
)]

extern crate color_backtrace;

use std::fmt::Display;

fn print<T: Display>(arg: T) {
    println!("{}", arg);
}


fn _main1() {
    let mut v: Vec<i32> = Vec::new(); // empty
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let item = &v[2];
    println!("{}", item);

    let another = item;
    println!("{}", item);
    // let v = vec![1, 2, 3];
}

/// cannot borrow `v` as mutable because it is also borrowed as immutable
/// error code
/// cannot have mutable and immutable borrows at the same time
fn _main2() {

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];
    // the fix is the consume the borrow before using it again
    // this line should be here
    // println!("The first element is: {}", first);

    v.push(6);

    // println!("The first element is: {}", first);
}

fn _main3() {

}

fn _main4() {

}

fn _main5() {

}

fn _main6() {

}

fn main() {
    color_backtrace::install();

    // _main1();
    _main2();
    // _main3();
    // _main4();
    // _main5();
    // _main6();
}
