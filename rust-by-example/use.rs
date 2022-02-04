#![allow(dead_code)]

enum Status {
	Rich,
	Poor
}

enum Work {
	Civilian,
	Soldier
}

fn main() {
	// Just like C++ namespaces (I think), we can use "use" here to save us the hassle of scoping variables and such
	use crate::Status::{Poor, Rich}; 	// Using specific things
	use crate::Work::*; 				// Automatically uses everything nside Work

	let status = Poor; // == let status = Status::Poor;
	let work = Civilian;

	match status {
		// Note that we don't use scoping here too
		Rich => println!("The rich have lots of money!"),
		Poor => println!("The poor have no money...")
	}	

	match work {
		Civilian => println!("Civilians work!"),
		Solider => println!("Soldiers fight!")
	}
}
