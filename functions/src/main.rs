fn main() {
    print_result(2, 3, get_sum);
}

fn print_result(lhs: i32, rhs: i32, f: fn(i32, i32) -> i32) {
    println!("sum is: {}", f(lhs, rhs));
}

fn get_sum(lhs: i32, rhs: i32) -> i32{
    lhs + rhs
    // No semicolon because this is a statement!
}
