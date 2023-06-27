fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    // s1 consists of a fixed amount of memory on the stack and a variable
    // amount of memory on the heap. The stack memory contains a pointer to
    // the heap allocation as well as a length and capacity. The heap contains
    // the characters that make up the string contents.
    let s1 = String::from("hello");

    // s2 consists only of a copy of the stack memory from s1. No allocation
    // is made to the heap. It merely points to the existing allocation made
    // when instantiating s1.
    let s2 = s1;
    
    // once s2 is instantiated, s1 is no longer considered valid, and therefore
    // cannot be used. This is to prevent the situation where a double free for
    // the single allocation on the heap could occur when exiting the scope.
    // This process of transfering ownership of the heap allocating from s1 to
    // s2 is known as a move. Once ownership is transfered, S1 is useless and
    // therefore popped off the stack. It literally doesn't exist at this point

    // Here, a new string called s1 is instantiated. It consists of an entirely
    // new allocation to the stack as well as the heap.
    let s1 = String::from("hello");

    // s2 is a clone of s1, or otherwise a deep copy. It also consists of an
    // entirely new allocation to both the stack and the heap.
    let s2 = s1.clone();

    // S1 is still valid in this context because ownership of its' heap
    // allocation never changed.
    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x;

    // Both x and y are valid in this context because they are both integers,
    // and therefore their size is known at compile time. Therefore no heap
    // allocation have been made and ownership rules do not come into play.
    println!("x = {}, y = {}", x, y);

    //Any type of a known size implements the "Copy" trait. Any type that
    //implements the Copy trait will not result in moves for variables of
    //that type. Conversely, any type that implements a "Drop" trait required
    //for deallocation, cannot implement the "Copy" trait.

}

fn transfer_ownership_by_passing() {
    let s = String::from("hello"); //s comes into scope
    takes_ownership(s);            // s's value moves into the function...
                                   // ... and so is no londer valid here

    let x = 5;                     // x comes into scope

    makes_copy(x);                 // x would move into the funciton,
                                   // but i32 has the Copy trait, so it's
                                   // still ok to use afterward
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and 'Drop' is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {      // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn transfer_ownership_by_return() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it
    
    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope
    a_string //a_string is return and moves out to the calling function
}
