struct Philosopher {
name: String,
}

impl Philosopher {
// Associated function
fn new(name: &str) -> Philosopher {
	Philosopher {
	name: name.to_string(),
	}
}

// Methods take an explicit self parameter
fn eat(& self) {
	println!("{} is done eating.", self .name);
}
}

fn main() {
	// Vec<T> is a growable array type
	let philosophers = vec![
	Philosopher::new("Judith Butler"),
	Philosopher::new("Gilles Deleuze"),
	Philosopher::new("Karl Marx"),
	Philosopher::new("Emma Goldman"),
	Philosopher::new("Michael Foucalt"),
	];

	for p in &philosophers {
			p.eat();
	}
}
