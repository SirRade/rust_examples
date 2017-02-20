fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];

    let answer = borrow_vec(&v1, &v2);

    // We can use `v1` and `v2` here!
    for i in v1 {
        println!("{}", i);
    }
    for i in v2 {
        println!("{}", i);
    }


    let mut x = 5;
    // Scope is needed
    {
        let y = &mut x;
        *y += 1;
    }
    println!("{}", x);

    // Doesn't work: y would live longer than x
    //let y: &i32;
    let x = 5;
    let y: &i32;
    y = &x;

    println!("{}", y);

    let mut x = &5;
    let mut y = x;
    println!("x: {:?}, y: {:?}", x, y);
}

fn borrow_vec(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    // Do stuff with `v1` and `v2`.

    // Return the answer.
    42
}

fn foo(v: &Vec<i32>) {
     //v.push(5);
     // Doesn't work: references are immutable by default
}

