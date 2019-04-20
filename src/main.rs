//! # Primes CLI
//!
//! `primes-cli` is a small collection of utilities to help you find prime numbers

use primes::PrimeSet;
use std::io;

//	accept a number on stdin and return the nearest primes

fn main() {
	let mut i = String::new();
	io::stdin()
		.read_line(&mut i)
		.expect("stdin is not available");

	let i: u64 = match i.trim().parse() {
		Ok(num) => num,
		Err(e) => {
			//	complain and die
			eprintln!("Not a number. {}", e);
			std::process::exit(1);
		}
	};

	//	find primes
	let mut pset = PrimeSet::new();
	let (p2i, p2) = pset.find(i);
	let p1 = pset.get(p2i - 1);
	if p2 == i {
		let p3 = pset.get(p2i + 1);
		println!("{},{},{}", p1, p2, p3);
	} else {
		println!("{},{}", p1, p2);
	}

	//	exit normally
	std::process::exit(0);
}
