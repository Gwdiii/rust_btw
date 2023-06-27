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
    let state = UsState::Texas;
    let coin  = Coin::Quarter(state);
    let cents = value_in_cents(coin);

    println!("The value of cents is: {cents}");

    let five = Some(5);
    let _six  = plus_one(five);
    let _none = plus_one(None);

}

// Unlike other conditionals, a `match` can evaluate to any type. Matches are
// exhaustive. IE: all cases must be covered for the source to compile
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny   => 1,  // `match` arms consist of a pattern, and some
        Coin::Nickel  => 5,  // code to execute on a match. Each arm is
        Coin::Dime    => 10, // delimited by a comma. Once a match is made,
                             // any remaining arms are not evaluated.
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}

// `match` can be used to extract the inner `<T>` from an `Option` type
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1), // `i` binds to the value contained in `Some`
    }
}

fn handle_dice_roll(dice_roll: u8) {
    match dice_roll {
        3     => add_dunce_hat(),
        7     => remove_dunce_hat(),
        other => move_player(other), // all remaining cases of `u8` are covered
    }
}

fn handle_dice_roll_with_hole(dice_roll: u8) {
    match dice_roll {
        3 => add_dunce_hat(),
        7 => remove_dunce_hat(),
        _ => (), // nothing happens here
    }
}

fn handle_dice_roll_with_unit(dice_roll: u8) {
    match dice_roll {
        3 => add_dunce_hat(),
        7 => remove_dunce_hat(),
        _ => reroll(),
    }
}

fn reroll() {}
fn add_dunce_hat() {}
fn remove_dunce_hat() {}
fn move_player(num_spaces: u8) {}
