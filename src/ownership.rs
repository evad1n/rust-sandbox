#[allow(dead_code)]
pub fn run() {
	println!("Ownership demo!");

	let n1 = Number { value: 42 };

	print_number_own(n1);
	// print_number_own(n1); won't work

	let n2 = Number { value: 42 };

	print_number_ref(&n2);
	print_number_ref(&n2);
}

struct Number {
	value: i32,
}

fn print_number_own(n: Number) {
	println!("{}", n.value)
}

fn print_number_ref(n: &Number) {
	println!("{}", n.value)
}
