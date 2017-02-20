fn main() {
    // Array
    let a = [0, 1, 2, 3, 4];


    // Slice    
    let complete = &a[..]; // a slice containing the whole a
    let middle = &a[1..4]; // a slice containing 1, 2, 3
    // First is inclusive, second is not

    println!("First element of slice middle is: {}", middle[0]);


    // Tuple
    let tuple = (1, "hello");
    let explicit_tuple: (i32, &str) = (1, "hello");

    let mut arityOfTwo = (1, 2);
    let anotherArityOfTwo = (2, 1);
    let arityOfThree = (4, 6, 1);

    arityOfTwo = anotherArityOfTwo;
    // arityOfTwo = arityOfThree; 
    // Compiler error: different arity

    println!("First element in tuple: {}", arityOfTwo.0);
    println!("Second element in tuple: {}", arityOfTwo.1);
    println!("Whole tuple: {:?}", arityOfTwo);

    let (x, y, z) = (1, 2, 3); // destructuring let
    println!("x is {}", x);
    println!("y is {}", y);
    println!("z is {}", z);
    // arityOfTwo = arityOfThree

    // Function pointers
    fn foo(x: i32) -> i32 { x }
    let pointerToFoo = foo;
    let explicitPointerToFoo: fn(i32) -> i32 = foo;
}
