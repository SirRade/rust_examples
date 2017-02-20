fn take(v: Vec<i32>) {
    // What happens here isn’t important.
}

fn main() {
    let v = vec![1, 2, 3];
    let v2 = v;
    // Doesn't work: Memory got moved out of v
    // println!("v[0] is: {}", v[0]);

    let w = vec![23; 123];
    take(w);
    // Doesn't work: Memory got moved out of v
    // println!("w[0] is: {}", w[0]);

    let b = true;
    println!("Old truth: {}", b);
    // Works: primitive types such as bool implement the Copy trait
    change_truth(b);
}

fn change_truth(x: bool) -> bool {
    !x
}