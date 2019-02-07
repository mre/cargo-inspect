#[derive(Debug)]
struct Point {
	x: i32,
	y: i32,
}

fn main() {
	let a = Point {x: 10, y:10};
	let b = Point {x: 5, y:16};

	println!("a.x: {} a.y: {}", a.x, a.y);
	println!("b.x: {} b.y: {}", b.x, b.y);

    // TODO Do something more itneresting with these two refs
}
