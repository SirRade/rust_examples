use std::ops::{Add, Mul};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// Implements the + operator for Point
impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

// Mix and match types
impl Add<i32> for Point {
    type Output = i64;
    fn add(self, rhs: i32) -> i64 {
        (self.x + self.y + rhs) as i64
    }
}


// Generic example
trait HasArea<T> {
    fn area(&self) -> T;
}

struct Square<T> {
    x: T,
    y: T,
    side: T,
}

impl<T> HasArea<T> for Square<T> 
        where T: Mul<Output = T> + Copy {
    fn area(&self) -> T {
        self.side * self.side
    }
}

fn main() {
    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };

    let p3 = p1 + p2;

    println!("{:?}", p3);

    let foo = p3 + 123;
    println!("{:?}", foo);

    let some_square = Square{x: 3, y: 5, side: 4};
    println!("{:?}", some_square.area());
}
