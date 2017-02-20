fn main() {
    let plus_one = |x: i32| -> i32 { x + 1 };
    // Type inference:
    let plus_one = |x: i32| x + 1;
    let plus_two = |x| {
        let mut result: i32 = x;

        result += 1;
        result += 1;

        result
    };


    // Closures use variables of the enclosing scope
    // Here num gets borrowed:
    let num = 5;
    let plus_num = |x: i32| x + num;
    assert_eq!(10, plus_num(5));

    // And here nums gets moved:
    let nums = vec![1, 2, 3];
    let take_nums = || nums;
    // Doesn't work
    // nums got moved
    // println!("{:?}", nums);

    // And here everything gets moved!
    let world_eater = move || take_nums;


    // Using a closure as a parameter
    let answer = call_with_one(|x| x + 2);
    assert_eq!(3, answer);
    let answer = call_with_one_dynamically(&|x| x + 2);

    call_with_ref(|x| x + 1);

    // One can use function pointers inplace of closures
    call_with_one(&add_one);

    let someFn = gibe_closure();
    someFn(2);
}


// Using a closure as an argument
fn call_with_one<F>(some_closure: F) -> i32
    where F: Fn(i32) -> i32 {
    some_closure(1)
}

fn call_with_one_dynamically(some_closure: &Fn(i32) -> i32) -> i32 {
    some_closure(1)
}

fn call_with_ref<F>(some_closure:F) -> i32
    where F: for<'a> Fn(& 'a i32) -> i32 {

    let value = 0;
    some_closure(&value)
}

fn add_one(x: i32) -> i32 {
    x + 1
}


// Returning a closure
fn gibe_closure() -> Box<Fn(i32) -> i32> {
    let num = 5;
    Box::new(move |x| num + x)
}