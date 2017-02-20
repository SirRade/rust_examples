struct Firework {
    strength: i32,
}

// drop gets automatically called when value goes out of scope
impl Drop for Firework {
    fn drop(&mut self) {
        println!("BOOM times {}!!!", self.strength);
    }
}

// prints 
// BOOM times 100!!!
// BOOM times 1!!!
// Because the variables are stack based (last in - first out)
fn main() {
    let firecracker = Firework { strength: 1 };
    let tnt = Firework { strength: 100 };
}