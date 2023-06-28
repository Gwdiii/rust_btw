enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // Vectors are implemented using generics. When initializing a vector
    // using `Vec::new`, the type stored within the vector can be specified
    // using the syntax `Vec<T>`.
    let v: Vec<i32> = Vec::new();
    
    // It's not necessary to annotate the type of vector. In this example,
    // the type can be infered by the values being pushed into it.
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Vectors can also be intialized with values using the `vec!` macro.
    let v = vec![1, 2, 3, 4, 5];

    // third is a reference to the third element of v
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // By wrapping the vector in an Option, using `v.get`, a value will only be
    // returned if it exists at a valid index. If not, None is returned, and
    // the non-value can be handles in a match statement.
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None        => println!("There is no third element."),
    }

    // iterating over the values in an immutable vector can be done using
    // a reference
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    // Example of mutating a mutable vector
    // note that `i` is dereferenced using the `*` operator
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{i}");
    }

    // A single instance of a vector can only store values of the same type.
    // Using an enum, we can store values of different types, within the same
    // type of enum.
    let row = vec! [
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("red")),
        SpreadsheetCell::Float(10.12),
    ];
}
