// Signed ints: i8, i16, i32, i64, i128, isize
// Unsigned ints: u8, u16, u32, u64, u128, usize
// Floating point: f32, f64
// Char: 4 bytes of unicode values
// bool: true or false
// Unit type (): Only one possible value, an empty tuple ()

// Note that compiling this file will give you a bunch of warnings about unused variables, but it's alright
fn main() {
	// This is how you explicitly define types
	let logical: bool = true;
	let a_float : f64 = 1.0;

	// Notice that we didn't do "name : type" here, we added as suffix to the number
	let an_int = 5i32;

	// Rust will assign types by default if you don't specify them for ints and floats
	let default_float = 3.0;	// f64
	let default_int = 7; 		// i32

	// Rust can also infer type from context (at least for primitives)
	let mut inferred_type = 12;
	inferred_type = 4294967296i64; // The compiler sets the type to i64

	let mut mutable = 12;
	mutable = 21;

	// This line causes an error. Mutable variables can't change their type  even if they can change their value
	//mutable = true;

	// Redefining? The page calls it "shadowing"
	let mutable = true;
}
