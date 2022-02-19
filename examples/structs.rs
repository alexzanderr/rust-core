

#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_macros,
    unused_assignments,
    unused_mut,
)]

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

extern crate unindent;
use unindent::unindent;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    fn new() -> User {
        User {
            email: "justin@gmail.com".to_string(),
            username: "justin".to_string(),
            active: true,
            sign_in_count: 1
        }
    }
}


impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let _fmt = unindent(format!("
            active: {}
            username: {}
            email: {}
            sign_in_count: {}",
            self.active,
            self.username,
            self.email,
            self.sign_in_count).as_str());

        write!(f, "{}", _fmt)
    }
}

extern crate color_backtrace;


fn _main1() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{}", user1);
    println!("{}", user1);
    println!("{}", user1);

    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("123123121213\"asd12312");
    println!("{}", user2);

    let mut new_user = User::new();
    println!("{}", new_user);


    // expanded user1
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("{}", user2);
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "tuple_struct=({}, {}, {})", self.0, self.1, self.2)
    }
}

fn print<T: Display>(arg: T) {
     println!("{}", arg);
}

struct User2<'a> {
    username: &'a str,
    email: &'a str,
    sign_in_count: u64,
    active: bool,
}

fn _main2() {


    let black = Color(0, 0, 0);
    print(black);
    let origin = Point(0, 0, 0);
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn _main3() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}



fn main() {
    color_backtrace::install();

    _main3();
    // unimplemented!();
}