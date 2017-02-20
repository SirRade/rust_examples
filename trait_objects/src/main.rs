trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String { format!("u8: {}", *self) }
}

impl Foo for String {
    fn method(&self) -> String { format!("String: {}", *self) }
}

fn statically_do_something<T: Foo>(x: &T) {
    println!("This is statically dispached: {}", x.method());
}

fn dynamically_do_something(x: &Foo) {
    println!("This is dynamically dispached: {}", x.method());
}

fn static_to_dynamic_wrap<T: Foo>(x: &T) {
    println!("This is static wrapping the dynamically dispach");
    dynamically_do_something(x);
}

fn main() {
    let x = 5u8;
    let y = "Hello".to_string();

    statically_do_something(&x);
    statically_do_something(&y);

    dynamically_do_something(&x);
    dynamically_do_something(&y);

    static_to_dynamic_wrap(&x);
    static_to_dynamic_wrap(&y);
}
