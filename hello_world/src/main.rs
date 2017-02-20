fn main() {
    println!("Hello, world!");
   
    do_stuff(do_ints);

}

fn do_stuff(func: fn(i32) -> i32) {
    println!("Zahl ist {} 💩", func(69));
}

fn do_ints(number: i32) -> i32{
    number + number + 1
}


