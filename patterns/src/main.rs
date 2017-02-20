fn main() {
    println!("Hello, world!");
    let x = 5;
    // Match is an expression
    let number = match x {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        _ => "something else",
    };


    let x = 1;
    let c = 'c';
    match c {
        // Pitfall: x is shadowed
        x => println!("x: {} c: {}", x, c),
        // prints x: c c: c
    }
    println!("x: {}", x);
    // prints x: 1


    // Multiple matches
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }


    // Destructuring
    struct Point {
        x: i32,
        y: i32,
    }
    let origin = Point { x: 0, y: 0 };
    match origin {
        Point { x, y } => println!("({},{})", x, y),
    }
    // Alias names
    match origin {
        Point { x: swag, y: memes } => println!("({},{})", swag, memes),
    }
    // Ommit names
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
    match origin {
        Point { y, .. } => println!("y is {}", y),
    }

    enum OptionalTuple {
        Value(i32, i32, i32),
        Missing,
    }
    let x = OptionalTuple::Value(5, -2, 3);
    match x {
        OptionalTuple::Value(..) => println!("Got a tuple!"),
        OptionalTuple::Missing => println!("No such luck."),
    }

    fn coordinate() -> (i32, i32, i32) {
        return (1, 2, 3);
    }
    let (x, _, z) = coordinate();

    // Ranges
    let x = '💅';
    match x {
        'a' ... 'j' => println!("early letter"),
        'k' ... 'z' => println!("late letter"),
        _ => println!("something else"),
    }


    // Bindings
    let x = 1;
    match x {
        e @ 1 ... 5 => println!("got a range element {}", e),
        _ => println!("anything"),
    }

    // Complex Shizzle
    #[derive(Debug)]
    struct Person {
        name: Option<String>,
    }

    let name = "Steve".to_string();
    let x: Option<Person> = Some(Person { name: Some(name) });
    match x {
        Some(Person { name: ref a @ Some(_), .. }) => println!("{:?}", a),
        _ => {}
    }

    // Multiple pattern name binds
    let x = 5;
    match x {
        e @ 1 ... 5 | e @ 8 ... 10 => println!("got a range element {}", e),
        _ => println!("anything"),
    }


    // Guards
    enum OptionalInt {
        Value(i32),
        Missing,
    }

    let x = OptionalInt::Value(5);

    match x {
        OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than five!"),
        OptionalInt::Value(..) => println!("Got an int!"),
        OptionalInt::Missing => println!("No such luck."),
    }

    // Guard applies to all patterns in the match
    let x = 4;
    let y = false;

    match x {
        4 | 5 if y => println!("yes"),
        // it's like (4 | 5) if y => ...
        // and not
        // 4 | (5 if y) => ...
        _ => println!("no"),
    }
}
