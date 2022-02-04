// const: The good old constant we all know and love, cannot be changed
// static: Can possibly be changed, but it's unsafe to do so, note that its lifetime is inferred to be 'static (exists as long as the program runs)

static LANGUAGE : &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
	n > THRESHOLD
}

fn main() {
	let n = 16;
	println!("This is {}", LANGUAGE);
	println!("The threshold is {}", THRESHOLD);
	println!("{} is {}", n, if is_big(n) { "big" } else { "small" } );

	// This line shoud give a compiler error
	//THRESHOLD = 5;
}
