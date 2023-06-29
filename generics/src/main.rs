use std::cmp::PartialOrd;

// `Result` can be `Some` of any type
enum Result<T> {
    Some(T),
    None,
}

// `Option`, as implemented here, can be an `Ok` of any type, or an
// `Error` of any type.
enum Option<T, E> {
    Ok(T),
    Err(E),
}

// `Point` can have an `x` value of any type and a `y` value of any type
struct Point<T, U> {
    x: T,
    y: U,
}

// Generic types must be declared on the `impl` directly to use them in methods
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}

// This method only works on Point structs with f32 values for both `x` and `y`
impl Point<f32, f32> {
    fn distance_from_orgin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct Point_<X1, Y1> {
    x: X1,
    y: Y1,
}

// This method is called on a `Point_` struct with `x` and `y` values of
// generic types, takes a parameter of another `Point_` struct with 'x' and
// 'y' values of generic types, and returns a new `Point_` struct with a `x`
// from the caller struct and `y` from the parameter struct.
impl<X1, Y1> Point_<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point_<X2, Y2>) -> Point_<X1, Y2> {
        Point_ {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_int(&number_list);
    println!("The largest number is {}", result);

    let result = largest(&number_list);
    println!("The largest nubmer is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let _int_float = Point {x: 5,  y : 2.};
    let _integer   = Point {x: 2,  y : 10};
    let float      = Point {x: 1., y : 4.};

    let p = Point {x: 5, y: 10.1};
    println!("p.x = {}, p.y = {}", p.x(), p.y());

    let distance = float.distance_from_orgin();
    println!("distance is: {distance}");

    let this_p = Point_ {x: 5, y: 10.1};
    let that_p = Point_ {x: "foo", y: "bar"};
    let mix_p  = this_p.mixup(that_p);

    println!("mix_p is: {:?}", mix_p);
}

// `largest_int` takes a reference to a list of i32 only
fn largest_int(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// `largest_char` takes a reference to a list of char only
fn largest_char(list: &[char]) -> &char{
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// `largest` takes a reference to a list of any type which implements
// the trait `PartialOrd`. THis works for both `char` and `i32`
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
