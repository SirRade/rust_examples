use std::fmt::Debug;

struct Circle {
    x: f32,
    y: f32,
    radius: f32,
}

trait HasArea {
    fn area(&self) -> f32;
    fn is_larger_than(&self, other: &Self) -> bool;
}

impl HasArea for Circle {
    fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius.powi(2)
    }

    fn is_larger_than(&self, other: &Self) -> bool {
        self.radius > other.radius
    }
}


struct Rectangle<T> {
    x: T,
    y: T,
    width: T,
    height: T,
}

// Doesn't work
// T is not guaranteed to be comparable
// impl<T> Rectangle<T> {
impl<T: PartialEq> Rectangle<T> {
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}



trait ConvertTo<Output> {
    fn convert(&self) -> Output;
}

impl ConvertTo<i64> for i32 {
    fn convert(&self) -> i64 { *self as i64 }
}

// Can be called with T == i32.
fn normal<T: ConvertTo<i64>>(x: &T) -> i64 {
    x.convert()
}

// Can be called with T == i64.
fn inverse<T>(x: i32) -> T
        // This is using ConvertTo as if it were "ConvertTo<i64>".
        where i32: ConvertTo<T> {
    x.convert()
}

// Default method
trait Foo {
    fn is_valid(&self) -> bool;
    fn is_invalid(&self) -> bool { !self.is_valid() }
}


// Inheritance
trait FooBar : Foo {
    fn foobar(&self);
}

struct Baz;
// Baz has to implement Foo if it wants to implement FooBar!
impl Foo for Baz {
    fn is_valid(&self) -> bool { true }
}
impl FooBar for Baz {
    fn foobar(&self) { println!("foobar"); }
}


fn main() {
    let circle0 = Circle{x: 123.0, y: 234.0, radius: 12.0};
    let circle1 = Circle{x: 113.0, y: -394.0, radius: 13.0};
    println!("the area of the circle is: {}", circle0.area());
    println!("is the first circle bigger than the other one? {}", circle0.is_larger_than(&circle1));

    print_area(circle0);
    // Doesn't work
    // i32 doesn't implement the trait HasArea
    // print_area(67);

    let mut r = Rectangle {
        x: 0,
        y: 0,
        width: 13,
        height: 13,
    };
    assert!(r.is_square());
    r.height = 42;
    assert!(!r.is_square());

    let aide : i64 = inverse(9);    
}

// Doesn't work
// T is not guaranteed to have an area method
// fn print_area<T>(shape: T) {
fn print_area<T: HasArea> (shape: T) {
    println!("This shape has an area of {}", shape.area());
}

// All three are the same!
fn foo<T: Clone, K: Clone + Debug>(x: T, y: K) {
    x.clone();
    y.clone();
    println!("{:?}", y);
}

fn bar<T, K>(x: T, y: K) where T: Clone, K: Clone + Debug {
    x.clone();
    y.clone();
    println!("{:?}", y);
}

fn baz<T, K>(x: T, y: K)
    where T: Clone,
          K: Clone + Debug {

    x.clone();
    y.clone();
    println!("{:?}", y);
}