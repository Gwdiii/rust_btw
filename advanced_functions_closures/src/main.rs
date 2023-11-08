fn add_one(x: i32) -> i32 {x + 1}
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {f(arg) + f(arg)}

fn with_fn() {
    let list_of_numbers = vec![0, 1, 2];
    let list_of_strings: Vec<String> =
        list_of_numbers
            .iter()
            .map(|i| i.to_string())
            .collect();

    println!("list of strings from fn: {:?}", list_of_strings);
}

fn with_closure() {
    let list_of_numbers = vec![0, 1, 2];
    let list_of_strings: Vec<String> =
        list_of_numbers
            .iter()
            .map(ToString::to_string)
            .collect();

    println!("list of strings from closure {:?}", list_of_strings);
}

#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

fn initializer_as_closure() {
    let list_of_statuses: Vec<Status> = 
        (0u32..20)
            .map(Status::Value)
            .collect();

    println!("list_of_statuses: {:?}", list_of_statuses);
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    with_fn();
    with_closure();
    initializer_as_closure();
}
