#![allow(dead_code)]

// enum with implicit discriminator
// AKA enums that index starting at 0
enum Number {
	Zero,
	One,
	Two
}

// enum with explicit discriminator
enum Color {
	Red = 0xff0000,
	Green = 0x00ff00,
	Blue = 0x0000ff
}

fn main() {
	use Number::*;

	// Hmm, some sort of type casting?
	println!("Zero is {}", Zero as i32);
	println!("One is {}", One as i32);

	use Color::*;
	println!("Roses are #{:06x}", Red as i32);
	println!("Violets are #{:06x}", Blue as i32);
}
