fn main() {
    // const and static have to be explicitely annotated
    // consts have no memory address, as they're inlined
    const N: i32 = 5;
    
    // statics instead have a single memory location
    static M: i32 = 5;
    // Static variablen exist for the whole program
    // So they have a special static lifetime
    static NAME: &'static str = "Steve";

    // Doesn't work
    // Initial value has to be constant expression
    // static mut DANGER: i32 = get_val();
    static mut DANGER: i32 = 123;

    // both reading and writing a static mut is unsafe
    // as the compiler cannot guarantee
    // that it's free of race conditions
    unsafe {
        DANGER = get_val();
        DANGER += 1;

        println!("N: {}", DANGER);
    }
    println!("Hello, world!");
}

fn get_val() -> i32 {
    4
}