#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn main() {
    let shoe1 = Shoe { size: 44, style: String::from("sneaker") };
    let shoe2 = Shoe { size: 44, style: String::from("sandal") };
    assert_eq!(shoe1, shoe2);
}