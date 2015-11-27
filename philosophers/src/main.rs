use std::thread;
use std::sync::{Mutex, Arc};

struct Philosopher {
	name: String,
	left: usize,
	right: usize,
}

struct Table {
	forks: Vec<Mutex<()>>,
}

impl Philosopher {
	// Associated function
	fn new(name: &str, left: usize, right: usize) -> Philosopher {
		Philosopher {
		name: name.to_string(),
		left: left,
		right: right,
		}
	}

	// Methods take an explicit self parameter
	fn eat(&self, table: &Table) {
		let _left = table.forks[self.left].lock().unwrap();
		let _right = table.forks[self.right].lock().unwrap();

		println!("{} is eating.", self.name);

		thread::sleep_ms(1000);

		println!("{} is done eating.", self .name);

		// Locks will be freed when the variables holding them (_left, _right)
		// go out of scope, automatically
	}
}

fn main() {
	let table = Arc::new(Table { forks: vec![
		Mutex::new(()),
		Mutex::new(()),
		Mutex::new(()),
		Mutex::new(()),
		Mutex::new(()),
	]});

	// Vec<T> is a growable array type
	let philosophers = vec![
	Philosopher::new("Judith Butler", 0, 1),
	Philosopher::new("Gilles Deleuze", 1, 2),
	Philosopher::new("Karl Marx", 2, 3),
	Philosopher::new("Emma Goldman", 3, 4),
	Philosopher::new("Michael Foucalt", 0, 4),
	];

	let handles: Vec<_> = philosophers.into_iter().map( |p| {
		let table = table.clone();

		thread::spawn(move || {
			p.eat(&table);
		})
	}).collect();

	for h in handles {
		h.join().unwrap();
	}
}
