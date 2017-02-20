use std::rc::Rc;

fn foo(s: &str) {
    // Borrow a string for a second.
}
fn bar(s: &[i32]) {
    // Borrow a slice for a second.
}

struct Foo;
impl Foo {
    fn foo(&self) { println!("Foo"); }
}

fn main() {
    // String implements Deref<Target=str>.
    let owned = "Hello".to_string();
    // Therefore, this works:
    foo(&owned);


    // Rc<T> implements Deref<Target=T>
    let counted = Rc::new(owned);
    foo(&counted);

    // Vec<T> implements Deref<Target=[T]>.
    let owned = vec![1, 2, 3];
    bar(&owned);

    let my_foo = &&Foo;
    // &&Foo derefs to &Foo which derefs to Foo
    my_foo.foo();
    my_foo.foo();
    (&my_foo).foo();
    (&&my_foo).foo();
    (&&&&&&&&my_foo).foo();
}