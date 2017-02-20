// Following traits can be implemented automatically by the compiler:
/*
    Clone
    Copy
    Debug
    Default
    Eq
    Hash
    Ord
    PartialEq
    PartialOrd
*/

#[derive(Debug)]
struct Foo;

#[derive(Debug)]
struct Bar {
    x: i32,
}

fn main() {
    println!("{:?}", Foo);
    let bar = Bar{ x : 123 };
    println!("{:?}", bar);
}
