use std::fmt;
// Tuples can be used to return multiple values from functions or to pass them to a function

// Seems that the last line in a function without a semicolon is the return statement, interesting...
fn reverse(pair: (i32, bool)) -> (bool,i32) {
	let (integer,boolean) = pair;
	(boolean,integer)
}

#[derive(Debug)] 
struct Matrix(f32,f32,f32,f32);

// Activity 1
impl fmt::Display for Matrix {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
	}
}

// Activity 2
fn transpose( mat: Matrix ) -> Matrix {
	Matrix(mat.0, mat.2, mat.1, mat.3)
}



fn main() {
	// A tuple can have any number of any type of members
	let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64,
						0.1f32, 0.2f64, 'a', true);
	
	// We can access tuple members through their index
	println!("long tuple index 0 value : {}", long_tuple.0);
	println!("long tuple index 1 value : {}", long_tuple.1);

	// We can even nest tuples
	let nested_tuple = ((1u8, 2u16, 3u32), (4u64, -1i8), -2i16);
	
	// Tuples are printable (through the debug print)
	println!("Nested tuple: {:?}", nested_tuple);
	
	// Long tuples can't be printed, the compiler doesn't like them
	let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the compiler error

	let pair = (1,true);
	println!("The pair is {:?}", pair);
	println!("The reversed pair is {:?}", reverse(pair));

	// To create a one element tuple (no idea why'd do that)
	// You must leave a comma to differentiate them from a literal surrounded by parens
	println!("One element tuple {:?}", (5u32,));
	println!("Just an integer {:?}", (5u32));

	// Tuples can be broken down into seperate variables
	let tuple = (1, "hello", 4.5, true);
	let (a,b,c,d) = tuple;
	println!("{:?}, {:?}, {:?}, {:?}", a,b,c,d);

	let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
	println!("Matrix\n{}", matrix);
	println!("Transpose\n{}", transpose(matrix));
}

