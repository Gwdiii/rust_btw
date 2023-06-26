use std::io;

fn main() {
    let sum = 5 + 10;
    println!("sum is: {sum}");

    let difference = 95.5 - 4.3;
    println!("difference is: {difference}");

    let product = 4 * 30;
    println!("product is: {product}");

    let quotient = 56.7 / 32.2;
    println!("quotient is: {quotient}");

    let remainder = 43 % 5;
    println!("remainder is: {remainder}");

    //bools
    let t = true;
    let f: bool = false;

    //chars
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';

    //tuples
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    println!("five_hundred is: {five_hundred}");

    let six_point_four = x.1;
    println!("six_point_four is: {six_point_four}");

    let one = x.2;
    println!("one is: {one}");

    //arrays
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April",
                  "May", "June", "July", "August",
                  "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let last = a[a.len()-1];
    println!("last is: {last}");

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    
    println!("The value of the element at index {index} is: {element}");
}
