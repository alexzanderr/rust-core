struct Foo { x: int, y: int, z: int }

impl Foo {
    fn default() -> Foo {
        Foo{x: 0, y: 0, z: 0}
    }
}

fn bar(f: Foo) {
    println!(f);
}

fn join(s1: String, s2: String, _ joiner: String = " ") -> String {
    return s1 + joiner + s2
}

fn main() {
    bar(Foo{y: 5, ..Foo::default()});
}