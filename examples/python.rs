


trait _Object {
    fn __str__(self) where Self: Sized {
    }

    fn __len__(self) -> i32  where Self: Sized {
        123
    }
}

struct Int {
    value: i32
}

impl _Object for Int {
    fn __str__(self) {
        println!("{}", self.value);
    }

    fn __len__(self) -> i32 {
        self.value
    }
}

struct Float {
    value: f32
}

impl _Object for Float {
    fn __str__(self) {
        println!("{}", self.value);
    }

    fn __len__(self) -> i32 {
        self.value as i32
    }
}

// struct List {
//     inner: Vec<dyn _Object>
// }


// impl _Object for List {
//     fn __str__(self) {
//         println!("{:?}", self.inner);
//     }
// }

pub fn len<T>(_object: &T) -> i32 where T: _Object {
    _object.__len__()
}

pub fn str<T>(_object: &T) where T: _Object {
    _object.__str__()
}

// impl _Object for List<Int> {
//     fn __str__(self) {
//         println!("{:?}", self.inner);
//     }
// }


fn main() {
    let integer = Int { value: 123i32};
    let floater = Float { value: 123.123f32};

    println!("{}", len(&integer));
    // println!("{}", len(integer));
}