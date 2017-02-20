fn main() {
    for x in 0..10 {
        println!("{}", x); // x: i32
    }
    
    for (index, value) in (5..10).enumerate() {
        println!("index = {} and value = {}", index, value);
    }

    let lines = "hello\nworld".lines();

    for (linenumber, line) in lines.enumerate() {
        println!("{}: {}", linenumber, line);
    }

    'outer: for x in 0..10 {
        for y in 0..10 {
            if x * y == 25 {
                break 'outer;
            }
        }
    }
}
