pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
    // methods with default implementations elided
}

fn main() {
    // iter_map_collect();
    iter_loop();
}

fn iter_loop() {
    let v1 = vec![1, 2, 3];
    
    let v1_iter = v1.iter();
    println!("{:?}", v1_iter);
    for val in v1_iter {
        println!("Got: {}", val);
    }
}

fn iter_map_collect() {
    let v1 = vec![1, 2, 3];

    println!("v1: {:?}", v1);

    let v2 = v1.iter().map(|e| e * 2).collect::<Vec<u32>>();

    println!("v2: {:?}", v2);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iterator_demonstrator() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
        in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ]
        );
    }
}


