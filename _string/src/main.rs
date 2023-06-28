fn main() {
    // Rust core only has one string type `&str`, referred to as a string slice
    // The standard library provides the type `String` which is a growable,
    // mutable, owned, UTF-8 encoded string type. 

    // Example initializes an empty mutable string
    let mut s = String::new();

    // Example initializes an immutable string with a value
    let data = "initial contents";
    
    let s = data.to_string();

    // This also works on a literal directly
    let s = "initial contents".to_string();

    // The method `from` can also be implemented on the `String` type
    let s = String::from("initial contents");

    // strings being utf-8 encoded, these are all valid 
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // a `&str` can be appended to a `String` using the `push_str` method
    let mut s1 = String::from("foo");
    let s2 = "bar";
   
    // `push_str` takes a `&str` type, so ownership is not transferred
    s1.push_str(s2);
    println!("s1 is {s1}");
   
    // s2 could not be used here if `push_str` took ownership of s2
    println!("s2 is {s2}");

    // The `push` method takes a single `char` as a parameter and adds
    // it to the `String`;
    let mut s = String::from("lo");
    s.push('l');
    println!("s is: {s}");

    // String can also be concatenated using the `+` operator
    // fn add(self, s: &str) -> String {}
    // The `+` or `add` method signature looks like this:
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved and is no longer valid

    // Strings can not be indexed. This wont work:
    // let s1 = String::from("hello");
    // let h = s1[0];

    // A `String` is a wrapper over a `Vec<u8>`, but the characters are
    // utf-8 encoded, so the length of different literals may not be apparent
    let hello = String::from("Hola");
    println!("{hello}");

    let hello = String::from("Здравствуйте");
    println!("{hello}");
    
    // This wont work because the index 0 is only one byte, but the reasonable
    // expectation is that index 0 would return `3`. Since `3` is only one byte
    // of value `208`, the compiler assumes this is not likely what is wanted
    // and simply does not allow the operation. This doesn't work:
    // let hello = "Здравствуйте";
    // let answer = &hello[0];
    
    // String slicing can use ranges, but the ranges must contain only complete
    // and valid utf-8 characters. The characters in this string are 2 bytes
    // long, so this is valid:
    let hello = "Здравствуйте";
    let s = &hello[..4];
    println!("{s}");

    //This is however, quite dangerous, as if the range did not end at the
    // termination of a valid utf-8 character, it would case the program to
    // crash. This wouldn't work because the last byte is not a complete
    // utf-8 character:
    // let hello = "Здравствуйте";
    // let s = &hello[..4];
    
    // The best way to iterate over strings is to be explicit about whether
    // the iteration should be taken over character using the `chars` method
    // or over bytes using the `bytes` method implemented on the `String` type
    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }
}
