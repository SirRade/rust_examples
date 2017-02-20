fn main() {
    let x = 4;
    // let y: i32;
    // illegal because y wasn't initialized
    let y: i32 = 8;
    let y = 0;
    let y = "A sudden type change appears";
    // y = 3;
    // illegal because y is immutable
    let (foo, bar) = (2, 4);
    let mut some_mutable = 8;
    some_mutable = 7;
    
    println!("x: {}", x);
    println!("y: {}", y);
    println!("foo: {}", foo);
    println!("bar: {}", bar);
    println!("some_mutable: {}", some_mutable);
}
