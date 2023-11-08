use std::{ fmt, io::Error };

type Kilometers = i32;
type Thunk = Box<dyn Fn() + Send + 'static>;
type Result<T> = std::result::Result<T, std::io::Error>;

pub trait Write {
    fn flush(&mut self) -> Result<()>;
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

// impl<T> Option<T> {
//     pub fn unwrap(self) -> T {
//         match self {
//             Some(val) => val,
//             None => panic!("called `Option::unwrap()` on a `None` value"),
//         }
//     }
// }

fn never() {
    let guess = "0";
    for i in 0..10 {
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
}

fn sized() {
    fn generic_inferred_sized<T>(t: T) {
        // --snip--
    }

    fn generic_explicit_sized<T: Sized>(t: T) {
        // --snip--
    }

    fn generic_maybe_sized<T: ?Sized>(t: &T) {
        // --snip--
    }
}

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }
}
