fn main() {
    // Type declaration
    let x: Option<i32> = Some(5);
    // Doesn't work
    // types don't match
    // let x: Option<f32> = Some(5);
    let x: Option<f32> = Some(5.0);
    let x = Some(5);
    let x = Some(5.0);
    
    let mut int_origin = Point{x: 1, y: -54};
    let mut float_origin = Point{x: 1.9, y: 23.12};
    int_origin.swap();
    float_origin.swap();
}

fn takes_anything<T>(x: T) {
    // Do something with `x`.
}

fn takes_two_of_the_same_things<T>(x: T, y: T) {
    // ...
}

fn takes_two_things<T, U>(x: T, y: U) {
    // ...
}

struct Point<T> {
    x: T,
    y: T,
}

// Write the type twice in implementations!
impl<T> Point<T> {
    fn swap (&mut self) {
        std::mem::swap(&mut self.x, &mut self.y);
    }
}