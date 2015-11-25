extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
	println!("Guess the number!");

	let secret_number = rand::thread_rng().gen_range(1, 101);

	loop {
		println!("Please input your guess.");

		let mut guess = String::new();

		// read_line returns an io::Result, which has an ok() method
		// OK discards error information. Call expect() method, which takes the return
		// from ok() and if not success then calls panic!() with the supplied message
		io::stdin().read_line(&mut guess).ok().expect("Failed to read line");

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_)  => {
				println!("Please input a valid number!");
				continue;
			}
		};

		println!("Your guess: {}", guess);

		match guess.cmp(&secret_number) {
				Ordering::Less       => println!("Too small!"),
				Ordering::Greater    => println!("Too big!"),
				Ordering::Equal      => {
				println!("You win!");
				break;
			}
			}
	}
}
