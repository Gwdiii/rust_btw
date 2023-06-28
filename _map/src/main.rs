//Unlike vectors and strings, Hash maps have to be imported from `collections`
use std::collections::HashMap;

fn main() {
    // The type `HashMap<K, V>` stores a mapping of keys and values
    // of type `K` and `V` using a hashing function.
    
    // 'HashMap' implements the `new` and `insert` methods
    let mut scores = HashMap::new();
    scores.insert(String::from("Red"), 10);
    scores.insert(String::from("Blue"), 50);

    // `Hashmap` implements the `get` method as well. `get` returns an `&Option<T>`
    //  type. `copied` returns a copy of `Option<T>` type, and unwrap returns the
    //  value if `Some(value)` or 0 if `None`.
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("The value of score is: {score}");

    // Key/Value pairs can be iterated over
    for (key, value) in &scores {
        println!("Key: {key}, Value: {value}");
    }

    // For types that implement the `Copy` trait, such as `i32`, the values are
    // copied into the hash map. For values the implement the `Move` trait,
    // such as `String`, the values will be moved and the hash map will take
    // ownership of those values.
    let field_name = String::from("Favorite color");
    let field_value = String::from("Red");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // Both `field_name` and `field_value` are invalid at this point
    
    // Overwriting a value in a hash map
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // Adding a key and value only if a key isn't present using the `entry`
    // and `or_insert` methods. `or_insert` returns a mutable reference to the
    // value of the corresponding `Entry` key if that key exists. Otherwise,
    // it inserts the paramater and return a mutable reference to the new value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Red")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    // `split_whitespace` returns an iterator over sub-slices delineated by whitespace.
    for word in text.split_whitespace() {
        // `or_insert` returns a mutable reference (&mut V) to the value for
        // the specified key. Since count is a mutable reference, it must be
        // referenced using the `*` operator before assigning to it.
        let count = map.entry(word).or_insert(0);
        *count += 1;
    } // Mutable reference count goes out of scope here and is no longer valid.

    println!("{:?}", map);


}
