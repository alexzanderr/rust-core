

#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_macros,
    unused_assignments,
    unused_mut,
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

fn main() {
    color_backtrace::install();
    let exa_command = "exa --all --icons --colour=always";
    // let out_and_err = Exec::shell(exa_command)
    let errorneous_command = "not_found";
    let out_and_err = Exec::shell(errorneous_command)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Pipe)
        .capture();

    println!("{:?}", out_and_err.as_ref().unwrap().stderr_str());

    if let Err(ref err) = out_and_err {
        println!("{:?}", err);
    }

    if let Ok(ref out) = out_and_err {
        print("its working");
        // println!("{:?}", out);
        let exit_code = out.exit_status;
        if exit_code.success() {
            println!("{}", "nu merge frate");

        }
        println!("{:?}", exit_code);
    }

    match out_and_err {
        Ok(out) => {
            let out = out.stdout_str();
            println!("{:?}", out);

            for line in out.split("\n") {
                print(line);
            }
        },
        Err(err) => {
            // let err = err.stderr_str();
            println!("{:?}", err);
        }
    }
    print("tot ce este dupa match nu mai este rulat");
    print("ba da, lol");
    print(exa_command);


    // println!("{:?}", out);
}