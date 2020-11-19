struct Person {
	name: String,
	grade: i32
}

trait Printable {
	fn p(self);
}

impl Printable for Person {
	fn p(self) {
		println!("{}: {}", self.name, self.grade);
	}
}

#[allow(dead_code)]
pub fn run() {
	let x = 67;
	let y = 69;
	let (a, b) = (x, y);
	println!("a: {}, b: {}", a, b);
	print_number(a);
	print_number(b);
	print_number(-1);

	let will =  Person{name: "Will".to_string(), grade: 97};
	will.p();

	loop_example();
}


fn print_number(n: i32) {
	match n {
		1 => println!("One"),
		2 => println!("Two"),
		_ => println!("{}", n),
	};
}

fn loop_example() {
    for _ in 0..10 {
        hello();
    }
}

fn hello() {
    println!("Hello, world!");
}