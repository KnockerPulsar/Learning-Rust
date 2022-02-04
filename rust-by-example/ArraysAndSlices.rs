// Array: Same good old static arrays
// Slices: Dynamic arrays

use std:: mem;

// First void function!
// Also notice the signature of the slice &[type]
fn analyze_slice(slc: &[i32]) {
	println!("First element of the slice: {}", slc[0]);
	println!("The slice has {} elements", slc.len() );
}

fn main() {
	// Notice how you declare arrays
	// The array signature is [type; length]
	let xs: [i32; 5] = [1, 2, 3, 4, 5];

	// You can initialize all elements of an array to the same value
	// Maybe even do parts of the array?
	let ys: [i32; 500] = [0; 500];

	println!("The first element of the array: {}", xs[0]); 
	println!("The second element of the array: {}", xs[1]); 

	// We can use xs.len() to get the #elements in the array
	println!("The length of the array is {}", xs.len());
	// We can also figure out its length through the stack
	println!("#Bytes occupied by the array {}", mem::size_of_val(&xs));

	// Arrays are automatically borrowed as slices 
	// I think using the & operator is what is meant by "borrowing"
	analyze_slice(&xs);

	// A slice can point to a section of an array
	// [startingIndex .. endingIndex), the last element is endingIndex -1
	println!("Borrowing a section of the array");
	analyze_slice(&ys[1..4]);

	// Out of bound indexing causes compiler errors (Thanks Rust!)
	//println!("{}", xs[5]);
}
