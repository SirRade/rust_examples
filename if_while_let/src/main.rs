fn main() {
    let option = Option::Some(12);

    // If let
    // All three do the same:
    // 1.
    match option {
        Some(x) => { foo(x) },
        None => {},
    }

    // 2.
    if option.is_some() {
        let x = option.unwrap();
        foo(x);
    }

    // 3.
    if let Some(x) = option {
        foo(x);
    }

    // Else can be used too
    if let Some(x) = option {
        foo(x);
    } else {
        bar();
    }


    // While let
    // Both do the same
    // 1.
    let mut v = vec![1, 3, 5, 7, 11];
    loop {
        match v.pop() {
            Some(x) =>  println!("{}", x),
            None => break,
        }
    }
    // 2.
    let mut v = vec![1, 3, 5, 7, 11];
    while let Some(x) = v.pop() {
        println!("{}", x);
    }
}

fn foo(x: i32) {
    println!{"{:?}", x};
}

fn bar() {
    println!("hi :>");
}