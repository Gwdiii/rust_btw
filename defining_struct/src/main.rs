struct User {
    active: bool,
    username: String,
    email: String,
    signin_count: u32,
}

 // no keys needed for tuple structs, only types
struct Color(u8, u8, u8);
struct Point(i32, i32, i32);

// unit structs behave similarly to the unit type ()
// they can be useful when a trait needs to be implemented
// on some type, but no data needs to be stored in the type
struct AlwaysEqual;

fn main() {
    let _struct = build_struct("foo@bar.com", "foo");

    let email = String::from("foo@bar.com");
    let username = String::from("foo_bar");

    let _struct = shorthand_struct(email, username);
    let _struct = update_struct(_struct);
    let tuple_structs: (Color, Point) = build_tuple_structs();
    let subject = AlwaysEqual;
}

fn build_struct (email: &str, username: &str) -> User {
    let mut user1 = User {
        username: String::from(username),
        email: String::from(email),
        signin_count: 1_000_000,
        active: true,
    };
    user1.email = String::from("bar@foo.com");
    println!("active is: {}", user1.active);
    println!("username is: {}", user1.username);
    println!("email is: {}", user1.email);
    println!("signin_count is: {}", user1.signin_count);

    user1
}

fn shorthand_struct (email: String, username: String) -> User {
    let mut user1 = User {
        username, // as long as they are of the same name and type,
        email,    // the value does not need to be expressed explicitly
        signin_count: 1_000_000,
        active: true,
    };
    user1.email = String::from("bar@foo.com");
    println!("active is: {}", user1.active);
    println!("username is: {}", user1.username);
    println!("email is: {}", user1.email);
    println!("signin_count is: {}", user1.signin_count);

    user1
}

fn update_struct (user1: User) -> User {
    let user2 = User {
        email: String::from("foooo@bar.com"),
        ..user1 // any fields that weren't explicitly set
                // are populated from user1. This must come last.
                // Since username is a String, it is moved rather
                // than copied. Therefore, user1 is no longer valid
                // If the only elements populated from user1 were
                // of a known size at compile time, a move would
                // not occur, and user1 would still be valid
    };
    user2
}

fn build_tuple_structs () -> (Color, Point) {
    let black  = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    (black, origin)
}
