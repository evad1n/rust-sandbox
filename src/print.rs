struct Person {
	name: String,
	grade: i32
}

#[allow(dead_code)]
pub fn run() {
	// Numbered
	println!("{0} is a {1} and {0} will do what {1}s do!", "Will", "thug");

	// Named
	let will = Person {name: "Will".to_string(), grade: 97};
	println!("{name} has a grade of {grade}", name = will.name, grade = will.grade);
}