fn main() {
    let x = 9;
    let raw = &x as *const i32;

    let mut y = 34;
    let raw_mut = &mut y as *mut i32;

    // Doens't work
    // Dereferencing a pointer is unsafe
    // println!("raw points at {}", *raw);
    unsafe {
        println!("raw points at {}", *raw);
    }

    // Explicit cast: ref to ptr
    let i: u32 = 1;
    let p_imm: *const u32 = &i as *const u32;

    // Implicit coercion: ref to ptr
    let mut m: u32 = 2;
    let p_mut: *mut u32 = &mut m;

    // Explicit cast: ptr to ref 
    // transmute could have been used, but shouldn't be 
    // it's too powerfull for our needs and gives less guarantees
    unsafe {
        let ref_imm: &u32 = &*p_imm;
        let ref_mut: &mut u32 = &mut *p_mut;
    }
}
