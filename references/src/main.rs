fn main() {
    borrow_example();
    mutable_reference();
    multiple_mutable_references();
    mutable_immutable_reference();
}

fn borrow_example() {
    let s1  = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len)
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn mutable_reference() {
    let mut s = String::from("hello");
    change(&mut s);
    
    println!("The value of s is: {s}")
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn multiple_mutable_references() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s; // r1 is a mutable reference in this scope
    } // r1 no longer exists
    let r2 = &mut s;

    println!("The value of r2 is: {r2}");
}

fn mutable_immutable_reference() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("The value of r1 is: {r1}");
    println!("The value of r2 is: {r2}");
    //variables r1 and r2 are not used after this point

    let r3 = &mut s; // no problem because immutable references are not used
                     // at this point. Therefore, the scopes don't overlap.
                     // The compiler treats this the same as if an explicitly
                     // declared scope has been exited.
    println!("The value of r3 is: {r3}");
}
