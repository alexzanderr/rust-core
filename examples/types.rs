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

fn read_contents_from_file(file_path: String) -> String {
    fs::read_to_string(file_path).unwrap()
}

extern crate reqwest;

// use reqwest;

use std::collections::HashMap;


fn get_current_date() -> Result<String, reqwest::Error> {
    let url = "https://postman-echo.com/time/object";
    let result = reqwest::get(url);

    let response = match result.await {
        Ok(res) => res,
        Err(err) => return Err(err),
    };

    let body = response.json::<HashMap<String, i32>>();

    let json = match body {
        Ok(json) => json,
        Err(err) => return Err(err),
    };

    let date = json["years"].to_string();

    Ok(date)
}

use std::fs;
// /home/alexzander/Alexzander__/programming/rust/core/examples/types.rs
fn main() {
    let number: u32 = "42".parse().expect("not a number man ...");

    let content = fs::read_to_string("../Cargo.toml").unwrap_or("3000".to_string());
    _print!(content);

    // let cargo_contents = read_contents_from_file("Cargo.toml".to_string());
    // _print!(cargo_contents);

    // let file = String::from("Cargo.toml");
    // let cargo_contents = read_contents_from_file(file);
    // _print!(cargo_contents);
}
