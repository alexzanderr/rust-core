

#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_macros,
    unused_assignments
)]

fn main() {
    let age: i32 = 123;

    if age > 23 {
        println!("explicit is better than implicit")
    } else if age == 34 {
        println!("explicit is better than implicit; again")
    } else {
        println!("just something else")
    }


    // if with output capture
    let old_enough: bool = if age > 21 {
        true
    } else {
        false
    };


    // the best you can get close to ternary operator from C and C++
    let old_enough = if age > 21 { true } else { false };


    // let age: Option<i32> = Some(123123;
    match age {
        21 => println!("age is 21"),
        22 => println!("age is 22"),
        23 | 24 => println!("age is 23 or 24"),
        25..=28 => println!("asd"),
        n if n < 5 => {
            println!("very cool")
        }
        n if n > 50 => {
            println!("very cool again")
        }
        _ => {
            println!("this is the default case");
        }
    }

    // infinite loop
    let mut i = 0;
    loop {
        println!("asd");
        if i == 10 {
            break
        }
        i += 1
    }

    // while loop
    let mut i = 0;
    while i < 10 {
        i += 1
    }


    for j in 0..100 {

    }

    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 190];
    for num in nums {

    }

}