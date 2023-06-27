fn main() {
    string_slice();
    string_literal();
    use_improved_first_word();
    slice();
}

fn string_slice() {
    let s = String::from("hello world");
    let word = first_word(&s);

    println!("The value of word is: {word}");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { return &s[..i]; }
    }
    &s[..]
}

fn string_literal() {
    // The type of string literal s is &str
    // like anything else of type &str, string literals are immutable
    let s = "Hello, world!";

    println!("{s}")
}

fn use_improved_first_word() {
    let s = String::from("hello world");
    // 'improved_first_word' works on slices of Strings, partial or whole
    let word = improved_first_word(&s[..6]);
    let word = improved_first_word(&s[..]);
    println!("The value of word is: {word}");

    // 'improved_first_word' also works on references to Strings, which are
    // equivalent to whole slices of Strings
    let word = improved_first_word(&s);
    println!("The value of word is: {word}");

    let s_literal = "hello, world";

    // 'improved_first_word' works on slices of string literals,
    // whether partial or whole
    let word = improved_first_word(&s_literal[..6]);
    let word = improved_first_word(&s_literal[..]);
    println!("The value of word is: {word}");
    
    // Because string literal are &str anyway, this works too
    let word = improved_first_word(s_literal);
    println!("The value of word is: {word}");
}

// By changing the type of parameter 's' to &str, the function can now
// use an argument of type &String or &str. This is because of deref coercion
fn improved_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { return &s[..i]; }
    }
    &s[..]
}

fn slice() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}
