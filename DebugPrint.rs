// Enables debug printing, this allows structs to be printed without having to implement fmt::Display
#[derive(Debug)] 
struct DebugPrintable(i32);

// Can do the same to nested structs
#[derive(Debug)]
struct NestedStruct(DebugPrintable);
#[allow(dead_code)] // Tells the compiler that, "Yes, I know this wasn't ever used. Thank you for informing me. Please go ahead and continue compiling.
#[derive(Debug)]
struct Hmmmm(i32);

/* Defining a struct named Person with 2 components. A string reference "name", and an unsigned integer of 8 bits "age".
 The "'a" we see is called a lifetime, something the language uses to make sure that structs (and functions too) don't use invalid references.
*/
#[derive(Debug)]
struct Person<'a>{
	name: &'a str,
	age: u8
}

// Note that primitive types such as strings and i32 already have fmt::Debug and fmt::Display implemented
fn main()
{
	println!("{:?} we still here? Just to {:?}", "Why", "suffer?");
	println!("Does this work?\n {:?}", DebugPrintable(12));
	println!("Let's try the nested struct!\n {:?}", NestedStruct(DebugPrintable(135)));
	println!("This should work too!\n {:?}", Hmmmm(32));

	// Finally, declaring some variables
	let name = "Tarek";
	let age = 20;
	let tarek = Person { name, age }; // sturct instantiation similar to C

	// The # we see below enables pretty printing for debug outputs
	// This allows a little more elegance, although not as elegant as implementing our own fmt::Display.
	println!("{:#?}", tarek);
}
