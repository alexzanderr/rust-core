


extern crate fstrings;
extern crate ansi_term;
extern crate libc;
extern crate console;
// extern crate variadic;


// to use aesthetics you need to
// write mod aesthetics before
// using it like on line 21
mod aesthetics;
use aesthetics::*;

use std::hash::Hash;
use std::io::stdin;
use std::time::Duration;
use console::Term;

// (tmux) [~/Alexzander__/programming/testing_vim] git:(master[-M?--U])

fn print<T: std::fmt::Display>(arg: T) {
    println!("{}", arg);
}

macro_rules! calc {
    ($argument:expr) => {
        println!("{}", $argument);
    };
    // ($argument:expr, $arguments:expr, +) => {
    //     println!("{}", $argument);
    //     calc!()
    // };
}



// fn print<T: std::fmt::Display>(argument: T, argument2: T) {
//     println!("{} {}", argument, argument2);
// }

// fn f<T>(string: T) -> String {
//     return f!(string);
// }

fn read_line() -> String {
    let mut _input = String::new();

    stdin()
        .read_line(&mut _input)
        .expect("Failed to read line");

    return _input;
}

fn read_line_prompt(prompt_message: &str) -> String {
    println!("{}", prompt_message);
    return read_line();
}


extern "C" fn handle_interrupt(sig: libc::c_int) { // 1
    println!("Sorry we didn't get the chance to finish");
}


use std::thread;


fn not_main() {
    let x = 123;
    println!("{x}");
    print_red("asd");
    // let result = read_line();
    // print(result);
    // let result = read_line_prompt(" [~/Alexzander__/programming/rust/core] > ");
    // print(result);



    // All libc functions are unsafe
    unsafe {
        libc::signal(libc::SIGINT, handle_interrupt as libc::sighandler_t); // 2
    }
}


// fn something() {

//     print("asd");

//     let mut var: String = String::from("asdasdasdasdas");
//     print(var);

//     print("asd");
//     test("this is some cool red");
//     print_red("this is a better and real red");
//     print_yellow("this is a better and real red");
//     print_green("this is a better and real red");
//     print_white("this is a better and real red");
//     print_blue("this is a better and real red");
//     print_cyan("this is a better and real red");

//     print(red("somerthing") + "asd");
//     let name = "World";
//     let result = red(name);
//     print("salutare {result}");
//     print(5i64);

//     print(String::from("salutare my name isa ndrew"));

// }



extern crate termion;
use std::io::{stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn not12_main() {
    let stdin = stdin();
    //setting up stdout and going into raw mode
    let mut stdout = stdout().into_raw_mode().unwrap();
    //printing welcoming message, clearing the screen and going to left top corner with the cursor
    write!(stdout, r#"{}{}ctrl + q to exit, ctrl + h to print "Hello world!", alt + t to print "termion is cool""#, termion::cursor::Goto(1, 1), termion::clear::All)
            .unwrap();
    stdout.flush().unwrap();

    //detecting keydown events
    for c in stdin.keys() {
        //clearing the screen and going to top left corner
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )
        .unwrap();

        //i reckon this speaks for itself
        match c.unwrap() {
            Key::Ctrl('h') => println!("Hello world!"),
            Key::Ctrl('q') => break,
            Key::Alt('t') => println!("termion is cool"),
            _ => (),
        }

        stdout.flush().unwrap();
    }
}

extern crate rand;

use rand::Rng;
use std::collections::HashMap;
use std::cmp::Ordering;

fn read_number() -> u32 {
    let mut guess = String::new();

    println!("awaiting number read > ");
    stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    return guess.trim().parse().expect("Please type a number!");
}

fn main() {
    let mut _vector: Vec<i8> = vec![];
    let mut _counter: HashMap<i8, i32> = HashMap::new();

    for i in 1..100000i32 {
        let secret_number =
            rand::thread_rng().gen_range(1..101);

        // match 50.cmp(&secret_number) {
        //     Ordering::Less => println!("Too small!"),
        //     Ordering::Greater => println!("Too big!"),
        //     Ordering::Equal => {
        //         println!("You win!");
        //         println!("You win!");
        //         println!("You win!");
        //         println!("You win!");
        //         println!("You win!");
        //     },
        // }

        *_counter.entry(secret_number).or_insert(0) += 1;

        _vector.push(secret_number);
    }


    for (k, v) in _counter.iter() {
        println!("{}-{}", k, v);
    }


    // for item in _vector {
    //     print!("{} ", item);
    // }
    println!("\ndone");
}