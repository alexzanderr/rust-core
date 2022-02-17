use ansi_term::Colour::*;
// use ansi_term::ANSIGenericString;

pub fn print_red(string: &str) {
    println!("{}", Red.paint(string));
}

pub fn red(string: &str) -> String {
    Red.paint(string).to_string()
}

pub fn print_yellow(string: &str) {
    println!("{}", Yellow.paint(string));
}

pub fn yellow(string: &str) -> String {
    return Yellow.paint(string).to_string();
}

pub fn print_blue(string: &str) {
    println!("{}", Blue.paint(string));
}

pub fn blue(string: &str) -> String {
    return Blue.paint(string).to_string();
}

pub fn print_green(string: &str) {
    println!("{}", Green.paint(string));
}

pub fn green(string: &str) -> String {
    return Green.paint(string).to_string();
}

pub fn print_white(string: &str) {
    println!("{}", White.paint(string));
}

pub fn white(string: &str) -> String {
    return White.paint(string).to_string();
}

pub fn print_cyan(string: &str) {
    println!("{}", Cyan.paint(string));
}

pub fn cyan(string: &str) -> String {
    return Cyan.paint(string).to_string();
}

// this is working so far
pub fn test(string: &str) {
    println!("{}", string);
}
