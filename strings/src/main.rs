fn main() {
    // Type
    let greeting = "Hello there."; // greeting: &'static str

    // Literal forms
    let s = "foo
        bar";
    assert_eq!("foo\n        bar", s);

    let s = "foo\
        bar";
    assert_eq!("foobar", s);

    // String to str
    let mut s = "Hello".to_string(); // mut s: String
    println!("{}", s);

    s.push_str(", world.");
    println!("{}", s);

    // Parameter
    takes_slice(&s);

    // Doesn't work
    // indexing a string is not allowed
    // as [0] is ambiguous - is it the first char or byte?
    // println!("The first letter of s is {}", s[0]);

    let hachiko = "忠犬ハチ公";

    for b in hachiko.as_bytes() {
        print!("{}, ", b);
    }
    println!("");

    for c in hachiko.chars() {
        print!("{}, ", c);
    }
    println!("");

    let dog = hachiko.chars().nth(1); // Kinda like `hachiko[1]`.
    match dog {
        Some(expr) => println!("{}", dog.unwrap()),
        None => panic!(),
    }

    // Slicing    
    let dog = "hachiko";
    // byte 0 to byte 5
    let hachi = &dog[0..5];

    let dog = "忠犬ハチ公";
    // Doesn't work
    // no character boundary on byte at index 2
    // let hachi = &dog[0..2];

    // Concatenation
    // Requires &str on right side
    let hello = "Hello ".to_string();
    let world = "world!";
    let hello_world = hello + world;

    let hello = "Hello ".to_string();
    let world = "world!".to_string();
    // &String can coerce to &str
    let hello_world = hello + &world;
}

fn takes_slice(slice: &str) {
    println!("Got: {}", slice);
}