#[derive(Debug)]
enum UsState {
    California,
    Texas,
    NewYork,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    if_let();
    if_let_else();
}

fn if_let() {
    let config_max = Some(3u8);
    if let Some(max) = config_max { // If config_max is `Some(thing)` bind max
                                    // to the thing and execute the scoped code
        println!("The maximum is configured to be {}", max);
    } // Unlike `match`, `if let` is not exhaustive
}

fn if_let_else() {
    let state = UsState::Texas;
    let coin  = Coin::Quarter(state);
    let mut count = 0;

    if let Coin::Quarter(state) = coin{
        println!("State quarter from {:?}", state);
    } else { // `if let` statements can include an `else`
        count += 1
    }
    println!("count is: {count}");
}
