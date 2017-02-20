use std::mem;

fn main() {
    // 'as' is a type safe cast
    let x: i32 = 5;
    let y = x as i64;

    let one = true as u8;
    let at_sign = 64 as char;
    let two_hundred = -56i8 as u8;

    let nan: f64 = 8.0/0.0;
    println!("8/0 as a float is: {:?}", nan);

    // Silly things like this result in undefined behaviour
    let nan_as_int = nan as i32;
    println!("8/0 as an int is: {:?}", nan_as_int);

    // Pointers can be cast too
    // However, dereferencing any of this is unsafe
    let a = 300 as *const char; // `a` is a pointer to location 300.
    let b = a as u32;


    let a = [0u8, 1u8, 0u8, 0u8];
    // Doesn't work
    // 'non scalar cast'
    // We make assumptions about how underlying types are implemented
    // let b = a as u32; // Four u8s makes a u32.

    unsafe {
        // transmute is a cast that nearly completely ignores the type system
        let b = mem::transmute::<[u8; 4], u32>(a);
        println!("{}", b); // 256
        // Or, more concisely:
        let c: u32 = mem::transmute(a);
        println!("{}", c); // 256
        
        // Doesn't work
        // The only thing that transmute checks are sizes
        // let d: u64 = mem::transmute(a);
        // a doesn't fit in an u64
    }
}
