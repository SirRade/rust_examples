fn main() {
    let v = vec![1, 2, 3, 4];

    let i: usize = 10;
    let j: i32 = 10;

    // Works
    v[i];
    // Doesn't work
    // v[j];

    let a = vec![1, 2, 3];
    // next line panicks
    // println!("Item 7 is {}", v[7]);

    match a.get(7) {
        Some(x) => println!("Item 7 is {}", x),
        None => println!("Sorry, this vector is too short.")
    }

    let b = vec![1, 2, 3, 4, 5];

    for i in b {
        println!("Take ownership of the vector and its element {}", i);
    }

    /*
    doesn't work
    ownership was already transferred

    for i in b {
        println!("Take ownership of the vector and its element {}", i);
    }
    */

    let c = vec![1, 2, 3, 4, 5];

    for i in &c {
        println!("This is a reference to {}", i);
    }

    for i in &c {
        println!("This is a reference to {}", i);
    }

}
