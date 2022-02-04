use crate::List::*;

// List { node, nextNode } is what I think this look is 
enum List {
	Cons(u32, Box<List>), // Tuple struct that wraps and element and a pointer to the next element
	Nil // A node that signifies the end of a the linked list
}

// Methods can be attached to an enum!
impl List {
	// Constructor? Probably an allocator
	fn new() -> List {
		Nil // Nil is of type List
	}
	
	// Adds the passed node at the front of the list
	fn prepend(self, elem: u32) -> List {
		Cons(elem, Box::new(self))
	}

	// Return the length of the list
	fn len(&self) -> u32 {
		// It's usually better to match concrete types (AKA not references)
		// That's why we're matching *self (dereferencing)
		match *self { 
			// Uses a reference to the next node since we can't take ownership of it (I don't really fully understand the ownership system of rust) 
			// I think this also calls the function recursively until it reaches the Nil node.
			Cons(_, ref tail) => 1 + tail.len(),
			Nil => 0
		}
	}

	// Prints the linked list recursively 
	fn stringify(&self) -> String {
		match *self {
			Cons(head, ref tail) => {
				// format is similar to print, but returns a heap allocated string instead of printing to console
				format!("{}, {}", head, tail.stringify())
			},
			Nil => {
				format!("Nil")
			}
		}
	}

}


fn main() {
	let mut list = List::new();

	list = list.prepend(1);
	list = list.prepend(2);
	list = list.prepend(3);
	
	println!("Linked list has length {}" , list.len());
	println!("{}", list.stringify());
}
