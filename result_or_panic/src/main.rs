use std::cmp::Ordering;
use std::net::IpAddr;
use std::io;
use rand::Rng;

// `result` is a safe default to use in code that could fail, as a `panic!`
// can still be explicitely implemented to make a recoverable error
// unrecoverable. `unwrap` and `expect` are good for prototype code and
// examples. They make good markers to follow when you go back later to
// make the program more rebust. `panic!` is the best result for tests, as
// a failure is the mark of a failed test, and a `panic!` will provide the
// most useful feedback for this situation.

struct Guess {
    value: i32,
}

impl Guess {
    // validation is performed upon the initialization of the type,
    // so functions can trust that the value is valid without any
    // further error handling.
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        Guess{ value }
    }

    // getter ensures that `self.value` can be made private so that it can be
    // gauranteed that `self.value` is not modified without passing validation
    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    expect_ip_addr();
    guess()
}

fn expect_ip_addr() {
    let home: IpAddr = "127.0.0.1"
        .parse()
        // `expect` is appropriate in cases where failure is impossible
        .expect("Hardcoded IP address should be valid");
}

fn guess() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: Guess = match guess.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_)  => continue,
        };

        // range check is used here to avoid unneccesary `panic!`
        // superseded by validation upon initialization of `Guess`
        if guess.value() < 1 || guess.value() > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        println!("You guessed: {:?}", guess.value);

        match guess.value().cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}
