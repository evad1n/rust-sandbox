#[allow(dead_code)]
pub fn run() {
	println!("Iterators!");

	let x = evens(20);

	for n in x {
		println!("{}", n);
	}
}

fn evens(max: i32) -> Vec<i32> {
	let evens: Vec<i32> = (0..max).filter(|x| x % 2 == 0).collect();
	evens
}
