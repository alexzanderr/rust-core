#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_macros,
    unused_assignments,
    unused_mut
)]

fn _main1() {
    let mut s = String::from("salutare si bine team gasit la asod niasd");

    s.push('c');
    s.push('\n');
    s.push('c');
    s.push_str("some string");
    s.push_str("some string");
    println!("{}", s);

    // let x = 5;
    // let mut y = x;
    // y = 123;

    // let mut xx = 56;
    // let mut y = &xx;
    // y = &123;
    // let mut x = 5;
    // let mut y = x;
    // y = 123;
    // x = 123;

    // this doesnt work, why?
    // here is a pointer to the hello arr in mem
    let s1 = String::from("hello");
    // here we move the value from s1 to s2
    let s2 = s1;
    // basically after move s1 is NULL or doesnt exist anymore
    // println!("{}, world!", s1);

    // this works
    let s1 = String::from("hello");
    let s2 = &s1;
    // s2 = String::from("123");
    println!("{}, world!", s1);

    let s1 = String::from("hello");
    // here we move the value from s1 to s2
    let s2 = s1.clone();
    println!("{}, world!", s1);
    println!("{}, world!", s2);

    // let data = vec![1, 2, 3];
    // println!("{:?}", data);
    // let closure = move || println!("captured {:?} by value", data);
    // let sal = closure();
    // println!("{}", sal);
}
fn takes_ownership(_string: &String) -> &String {
    _string
}

fn makes_copy(_x: i32) -> i32 {
    _x
}

fn _main2() {
    let s = String::from("hello"); // s comes into scope

    // s is moved into the function
    // so after the function s doesnt exist anymore
    // takes_ownership(s);

    // to use s after the function
    // pass a reference of s to the function
    // meaning that s will be borrowed in the function
    takes_ownership(&s); // s's value moves into the function...

    println!("{:?}", s);

    let x = 5; // x comes into scope

    // this works by default
    // because i32 has Copy trait
    // and it's value its copied into the func
    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
}

fn _main3() {
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    // s2 is moved into the function and goodbye
    // no matter if the function even returns the s2 back
    // its something else
    let s3 = takes_and_gives_back(s2);

    // compile error
    // s2 its gone because it was moved into the function
    // println!("{} {} {}", s1, s2, s3);

    fn gives_ownership() -> String {
        let some_string = String::from("yours");

        some_string
    }

    // This function takes a String and returns one
    fn takes_and_gives_back(a_string: String) -> String {
        // a_string comes into
        // scope

        a_string // a_string is returned and moves out to the calling function
    }
}


fn append(_string: &mut String, appendee: String) -> () {
    let appendee = appendee.as_str();
    _string.push_str(appendee);
}

async fn foo () {
}

fn _main4() {
    let mut s = String::from("123as dasd asd");
    println!("{}", &s[1..5]);
    println!("{}", &s[5..]);
    println!("{}", &s[..4]);

    println!("{}", &s[..]);


    append(&mut s, String::from("asd asdasdasd"));
    println!("{}", s);
}


fn _main5() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    println!("{} and {}", r1, r2);

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

fn main() {
    // _main1();
    _main5();
}
