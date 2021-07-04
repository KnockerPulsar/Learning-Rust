use std::fmt;

struct List(Vec<i32>);

// What this does:
// First, it prints the opening bracket, then the first element
// For each element after that it prints a comma, then the element
// Finally, it closes the square brackets
impl fmt::Display for List {
	fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
		let vec = &self.0;

		// Open up a bracket
		write!(f,"[")?;

		// Loop over every element of the vector
		for(count,elem) in vec.iter().enumerate() {

		// Very weird synatx for if conditions...
		if count != 0  { write!(f,", ")?; }

		// This line was the only one needed to be changed for the activity
		write!(f,"{}: {}",count, elem)?;
		}

		// Remember to not put a semicolon in the last line
		// Very weird too...
		write!(f,"]")
	}
}

fn main() {
	let vec = List(vec![4,5,6]);
	println!("{}", vec);
}
