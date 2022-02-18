#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_macros,
    unused_assignments
)]

use std::fmt;

use cairo::glib::Pointer;

#[derive(Debug)]
pub enum MyCustomError {
    HttpError,
    ParseError,
}

impl std::error::Error for MyCustomError {}

impl fmt::Display for MyCustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyCustomError::HttpError => write!(f, "HTTP Error"),
            MyCustomError::ParseError => write!(f, "Parse Error"),
        }
    }
}


fn other_case() {
    let decimal = 123_123;
    println!("{}", decimal);
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("{}", tup.0);

    let n:Vec<i32> = (1..10)
        .map(|i| i * 2)
        .filter(|&i| i > 5).collect();
        // .fold(0, |acc, i| acc + i);

    println!("{:?}", n);

    let arr = [1, 2, 3, 4, 5];
    let first   = arr[0];
    let second  = arr[1];
    println!("{:?}", arr);

    let y = {
        let xx = 3;
        xx + 1
    };

    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);
}


struct Point2D {
    x: f64,
    y: f64,
}

impl Point2D {
    // if you want to modify the state of the items within the struct
    // you need to set '&mut' before self
    fn move_right(&mut self, distance: f64) {
        self.x += distance;
    }

    // here returning a new object of the same class
    fn move_right_new(self, distance: f64) -> Point2D {
        Point2D {
            x: self.x + distance,
            y: self.y
        }
    }
}

// __str__ for POint2D
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x={}, y={})", self.x, self.y)
    }
}

fn main() {
    // this needs to be mut in order to
    // have methods that modify the state of the attributes of the class
    let mut p = Point2D {
        x: 123.123,
        y: 123.123
    };
    println!("{}", p);
    // cannot borrow as mutable
    // if the p is not declared with mut before
    p.move_right(123.123);
    println!("{}", p);




    let p2 = p.move_right_new(123.123);
    println!("{}", p2);
    // p2.x

    let mut count = 0;

    'loop_name: loop {
        'counting_up: loop {
            println!("count = {}", count);
            let mut remaining = 10;

            loop {
                println!("remaining = {}", remaining);
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'loop_name;
                }
                remaining -= 1;
            }

            count += 1;
        }
        println!("donme");
        break;

    }

    println!("End count = {}", count);

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2 + 123 + 123  + 123 + 123 ;
        }
    };

    println!("The result is {}", result);


}
