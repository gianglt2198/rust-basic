#[derive(PartialEq, Debug)]
pub struct Shoe {
    size: i32,
    style: String,
}

pub fn shoe_in_size(shoes: Vec<Shoe>, shoe_size: i32) -> Vec<Shoe> {
    shoes.into_iter().filter(|x| x.size == shoe_size).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn filter() {
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

        let my_shoe_size = shoe_in_size(shoes, 10);

        assert_eq!(
            my_shoe_size,
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
        )
    }
}
