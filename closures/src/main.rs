use std::{thread, vec};

#[derive(Debug)]
struct Rectangle {
    width : u32,
    _height: u32,
}

fn main() {
    immutable_ref();
    mutable_ref();
    force_ownership();
    fn_mut();
    broken_fn_once();
    fixed_fn_once();
}

// This captures an immutable reference because that is all that is needed
// to print.
fn immutable_ref() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrrows();
    println!("After calling closure: {:?}", list);
}

// This captures a mutable reference required to implement the push method.
// No other references are allowed until after the last (and in this case, only)
// call to `borrows_mutably` is made.
fn mutable_ref() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    // This will not work because the previous mutable reference is used later
    // println!("After calling closure: {:?}", list);
    borrows_mutably();
    // Okay to pass immutable reference here because previous mutable reference
    // is not invoked later
    println!("After calling closure: {:?}", list);
}

// This forces a move so that a new thread takes ownership
fn force_ownership() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // the move keyword forces the change in ownership. Without this guarantee,
    // there is no way to ensure that the new thread will access the reference
    // to list before the `force_ownership` scope closes, rendering list invalid
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}

// This function implements the `sort_by_key` method, which invokes the `FnMut`
// trait. The trait is required so that the closure may be called multiple
// times, once for each item in the slice. Somewhat unintuitively, `FnMut` does
// not require that something is mutated, but rather that the closure does not
// capture, or move anything from it's environment.
fn fn_mut() {
    let mut list = [
        Rectangle { width: 10, _height:  1 },
        Rectangle { width:  3, _height:  5 },
        Rectangle { width:  7, _height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
}

fn broken_fn_once() {
    let mut list = [
        Rectangle { width: 10, _height:  1 },
        Rectangle { width:  3, _height:  5 },
        Rectangle { width:  7, _height: 12 },
    ];

    let mut _sort_operations: Vec<String> = vec![];
    let _value = String::from("by key called");

    list.sort_by_key(|r| {
        // This wont work because type `String` does not implement the `Copy`
        // trait. Therefore, a move is performed, transferring ownership of
        // `value` out of the environment, to the `sort_operations` vector.
        // At this point there is no valid reference to `value` to capture,
        // as it has already moved. This is why any closure which results in
        // a move can only implement the `FnOnce` trait. "Once" the value has
        // been moved, the closure is no longer valid.
        // _sort_operations.push(_value);
        r.width
    });
    println!("{:#?}", list);

}


// Unlike `broken_fn_once`, this works because `num_sort_operations` is of type
// `usize` which is of a known size at compile time, and therefore sits on the
// stack, and does not implement the `Move` trait. It only captures a mutable
// reference.
fn fixed_fn_once() {
    let mut list = [
        Rectangle { width: 10, _height:  1 },
        Rectangle { width:  3, _height:  5 },
        Rectangle { width:  7, _height: 12 },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}
