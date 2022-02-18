

#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_macros,
    unused_assignments
)]

extern crate syntect;

use std::path::Path;
use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style, Theme};
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};


use std::fs::read_to_string;


fn read_contents_from_file(file_path: &str) -> String {
    read_to_string(file_path).unwrap()
}

fn load_theme_from_file(tm_file: &str) -> Theme {
    let tm_path = Path::new(tm_file);
    ThemeSet::get_theme(tm_path).unwrap()
}

fn syn_highlight_file(filename: &str, _theme: &str) {
    // Load these once at the start of your program
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();


    let one_dark_theme = load_theme_from_file("one_dark.tmTheme");

    // let from_file = Theme::;
    let syntax = ps.find_syntax_by_extension("rs").unwrap();
    // let mut highligther = HighlightLines::new(syntax, &ts.themes[_theme]);
    let mut highligther = HighlightLines::new(syntax, &one_dark_theme);

    let lines = read_contents_from_file(filename);
    let lines = lines.as_str();

    for line in LinesWithEndings::from(lines) {
        let ranges: Vec<(Style, &str)> = highligther.highlight(line, &ps);
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        print!("{}", escaped);
    }
}


fn main() {
    syn_highlight_file("Cargo.toml", "Solarized (dark)");
}
