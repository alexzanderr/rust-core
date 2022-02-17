

#![feature(thread_is_running)]
#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_macros,
    unused_assignments
)]


use std::time::Duration;
use std::thread::sleep;

use std::io::stdout;
use std::io::Write;
// fn animate(frames: [&str, 10], iterations: i32) {
//     for _iter in 1..iterations {
//         print!("{}", frames[_iter]);
//     }
// }

use std::thread;



fn main() {
    let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let mut _iter = 0;
    let message = "random message ...";

    let handle = thread::spawn(move || {
        // println!("started");
        // stdout().flush().expect("some error message");

        // some work here
        // println!("thread 2 started");
        let mut counter = 0;
        for _iter in 1..30 {
            counter += 1;
            // println!("asd");
            sleep(Duration::from_millis(100));
        }
        // println!("thread 2 done");
    });


    while handle.is_running() {
        let frame = frames[_iter % 10];
        print!("{frame} {message}\r", );
        // print!("12345");
        // print!("{}", (8u8 as char));
        stdout().flush().expect("some error message");
        sleep(Duration::from_millis(100));
        _iter += 1;
    }

    handle.join().unwrap();

    // let iterations = 20;
    // for _iter in 1..iterations {
    //     print!("{} {message}\r", frames[_iter % 10]);
    //     // print!("12345");
    //     // print!("{}", (8u8 as char));
    //     stdout().flush().expect("some error message");
    //     sleep(Duration::from_millis(100));
    // }
    // println!("spinner is done");

    // animate(frames, 10);
    // println!("{:?}", frames);
}