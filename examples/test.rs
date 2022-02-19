

#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_macros,
    unused_assignments,
    unreachable_code
)]

use std::env;

fn __exit() -> ! {
    println!("exited with code");
    std::process::exit(1);
}

fn usage_and_exit() -> ! {
    println!("USAGE: gendata synpack source-dir \
              newlines.packdump nonewlines.packdump \
              [metadata.packdump] [metadata extra-source-dir]\n       \
              gendata themepack source-dir themepack.themedump");

    std::process::exit(2);
}


fn main() {
    // usage_and_exit();
    __exit();
    let args = env::args();
    println!("{:?}", args);
    print!("asdasdasdasdasdasd\n");

}