#[allow(dead_code)]
pub fn run() {
	let mut a = Animal::new("Jimmy");
	a.print();
	a.kill();
	a.print();

	let d = Dog::new("Hanna", "Border Collie");
	d.print();
	d.speak();

	let mut c = Cat::new("Joe", "Tabby");
	c.print();
	c.speak();
	c.kill();
	c.speak();
	c.print();

	let mut s = Sheep::new("Broseph", "black");
	s.print();
	s.speak();
	s.shear();
	s.print();
	s.shear();
	s.kill();
	s.speak();
	s.shear();
	s.print();
}

trait IsAnimal {
	fn kill(&mut self) {}

	fn print(&self) {}
}

trait Speaks {
	fn speak(&self);

	fn noise(&self) -> &str;
}

struct Animal {
	name: String,
	alive: bool,
}

impl Animal {
	fn new(name: &str) -> Animal {
		Animal {
			name: name.to_string(),
			alive: true,
		}
	}
}

impl IsAnimal for Animal {
	fn kill(&mut self) {
		self.alive = false;
	}

	fn print(&self) {
		println!(
			"Animal {} is {}",
			self.name,
			if self.alive { "alive" } else { "dead" }
		);
	}
}

struct Dog {
	animal: Animal,
	breed: String,
}

impl Dog {
	fn new(name: &str, breed: &str) -> Dog {
		Dog {
			animal: Animal::new(name),
			breed: breed.to_string(),
		}
	}
}

impl IsAnimal for Dog {
	fn kill(&mut self) {
		self.animal.alive = false;
	}
	fn print(&self) {
		println!(
			"{} the {} dog is {}",
			self.animal.name,
			self.breed,
			if self.animal.alive { "alive" } else { "dead" }
		);
	}
}

impl Speaks for Dog {
	fn speak(&self) {
		if self.animal.alive {
			println!("{} goes {}", self.animal.name, self.noise());
		} else {
			println!("{} is dead!", self.animal.name);
		}
	}

	fn noise(&self) -> &str {
		"woof"
	}
}

pub struct Cat {
	animal: Animal,
	breed: String,
}

impl Cat {
	fn new(name: &str, breed: &str) -> Cat {
		Cat {
			animal: Animal::new(name),
			breed: breed.to_string(),
		}
	}
}

impl IsAnimal for Cat {
	fn kill(&mut self) {
		self.animal.alive = false;
	}

	fn print(&self) {
		println!(
			"{} the {} cat is {}",
			self.animal.name,
			self.breed,
			if self.animal.alive { "alive" } else { "dead" }
		);
	}
}

impl Speaks for Cat {
	fn speak(&self) {
		if self.animal.alive {
			println!("{} goes {}", self.animal.name, self.noise());
		} else {
			println!("{} is dead!", self.animal.name);
		}
	}

	fn noise(&self) -> &str {
		"meow"
	}
}

// Sheep

struct Sheep {
	animal: Animal,
	color: String,
	naked: bool,
}

impl Sheep {
	fn new(name: &str, color: &str) -> Sheep {
		Sheep {
			animal: Animal::new(name),
			color: color.to_string(),
			naked: false,
		}
	}

	fn shear(&mut self) {
		if self.animal.alive {
			if self.naked {
				println!("Already sheared!")
			} else {
				self.naked = true;
				println!("You got some nice {} wool!", self.color)
			}
		} else {
			println!("{} is dead!", self.animal.name);
		}
	}
}

impl IsAnimal for Sheep {
	fn kill(&mut self) {
		self.animal.alive = false;
	}
	fn print(&self) {
		println!(
			"A {} sheep named {}. {}. Currently {}.",
			self.color,
			self.animal.name,
			if self.naked {
				"Has been sheared"
			} else {
				"Not sheared"
			},
			if self.animal.alive { "alive" } else { "dead" }
		);
	}
}

impl Speaks for Sheep {
	fn speak(&self) {
		if self.animal.alive {
			println!("{} goes {}", self.animal.name, self.noise());
		} else {
			println!("{} is dead!", self.animal.name);
		}
	}

	fn noise(&self) -> &str {
		"baaa"
	}
}
