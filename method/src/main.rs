#[derive(Debug)]
struct Rectangle {
    width:  u32,
    height: u32,
}

impl Rectangle {
    // these are all associated functions because they're associated with
    // the type implemented, in this case `Rectangle`
    fn square(size: u32) -> Self {
        Self {
            width:  size,
            height: size,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> u32 { // This is a getter. The compiler knows the
                            // difference between self.width() and self.width,
                            // so we can call them the same thing. Convenient
        self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
fn main() {
    let r1 = Rectangle {
        width:  30,
        height: 50,
    };

    let r2 = Rectangle {
        width:  10,
        height: 40,
    };

    let r3 = Rectangle {
        width:  60,
        height: 45,
    };

    println!("Width of r1 is: {}", r1.width());
    println!("Can r1 hold r2? {}", r1.can_hold(&r2));
    println!("Can r1 hold r3? {}", r1.can_hold(&r3));

    // associated function square is namespaced by the struct
    let square = Rectangle::square(25);
}

