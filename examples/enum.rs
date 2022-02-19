





#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_macros,
    unused_assignments,
    unused_mut
)]

extern crate color_backtrace;
extern crate subprocess;

use subprocess::Exec;
use subprocess::Redirection;
use subprocess::ExitStatus;

use std::fmt::Display;

fn print<T: Display>(arg: T) {
    println!("{}", arg);
}


#[derive(Debug)]
enum IpAddrKind {
    V4(i32),
    V6(i32),
}


enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn return_tuple() -> (i32, i32, i32) {
    (123, 1123, 123)
}

fn _main2() {
    let config_max = 123u8;
    
    if let max = config_max {
        println!("The maximum is configured to be {:?}", max);
    } else {
        println!("its {:?}", config_max);
    }
    // print(max); // errorneous code, max its dealocatted
}

fn main() {
    color_backtrace::install();

    _main2();

    // let home = IpAddr::V4(127, 0, 0, 1);


    // let loopback = IpAddr::V6(String::from("::1"));

    // let v4 = IpAddrKind::V4;

    // println!("{:?}", v4);
    // print(IpAddrKind::V6);
}