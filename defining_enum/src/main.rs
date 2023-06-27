enum IpAddress {
    V4(String), // V4 is the name, but also a function that takes a String
    V6(String), // as an argument and returns an instance of the enum
}

enum Message {
    ChangeColor(u8, u8, u8), // The variants of an enum do not need to be
    Move { x: i32, y: i32 }, // of the same type. If there is no need for
    Write(String),           // associated data, they don't even need to
    Quit,                    // to be typed at all, for example: `Quit`
}

impl Message { // Enums can be implimented with associated functions too
    fn call(&self) {
        // some method body defined here
    }
}

// enum Option<T> { // The `<T>` is a generic type parameter. The `Some`
//     None,        // variant of `Option` can hold a value of any type
//     Some(T),     // passed into `Option`
// }

fn main() {
    // variants of an enum are namespaced under their identifier
    let _four = IpAddress::V4(String::from("127.0.0.1"));
    let _six  = IpAddress::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let _some_number = Some(5);
    let _some_char = Some('e');

    // the `None` variant of `Option` serves the same function as `null`
    // in other languages, but the implementation causes far fewer problems
    let _absent_number: Option<i32> = None;
}
