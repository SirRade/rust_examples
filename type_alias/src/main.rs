use std::result;

enum ConcreteError {
    Foo,
    Bar,
}

// aliases are useful for giving certain generics a name
type Result<T> = result::Result<T, ConcreteError>;

fn main() {
    // type denotes an alias that is equivalent to the original type
    type Num = i32;
    let x: i32 = 234;
    let y: Num = 234;

    if x == y {
        println!("i32 and Num are interchangeable");
    }

    // Use tuple struct if the types should be different
    struct DifferentNum(i32);
    let z = DifferentNum(23);
}
