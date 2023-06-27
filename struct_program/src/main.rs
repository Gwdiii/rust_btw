#[derive(Debug)] // this is an outer attribute to derive the debug trait
struct Rectangle {
    width:  u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let r = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&r)
    ); // r must be passed by reference to prevent a move

    println!("r is: {:?}", r); // this prints to stdout
    dbg!(&r);                  // this prints to stderr
}

fn area(dimensions: &Rectangle) -> u32 {
    dimensions.width * dimensions.height
}
