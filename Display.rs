// Importing std::fmt
use std::fmt;

// The derive compiler directive tells the compiler to implement a basic debug print implementation for this structure.
// i64 = 64 bit signed integer
#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax { // We can access struct members by index
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
		// No semicolons at the end of the next line!
		write!(f, "({},{})", self.0,self.1)
	}
}

// f64 = 64 bit floating point
#[derive(Debug)]
struct Point3D {
	x: f64,
	y: f64,
	z: f64
}

#[derive(Debug)]
struct Complex(f64,f64);

// For Point3D, we can just use self.index like MinMax since we defined it in a different way
impl fmt::Display for Point3D {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "x: {}, y: {}, z: {}", self.x, self.y, self.z)
	}
}

// Extra : implementing fmt::Display for Complex
impl fmt::Display for Complex {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} + {}i", self.0,self.1)
	}
}

fn main() {
	let minmax = MinMax(0,69);
	
	println!("Let's compare our print and debug print for MinMax");
	println!("Display: {}", minmax);
	println!("Debug: {:?}", minmax);

	// Rust likes snake_case
	let big_range = MinMax(-300,300);
	let smol_range = MinMax(-1,1);

	println!("The big range is {big}, and the small range is {small}",
		big = big_range,
		small = smol_range);

	// The only way to create an object of this sturct
	// This is probably due to how we defined it 
	let pt = Point3D{ x: 3.3, y: 7.2, z: 10.5};
	
	println!("Let's compare our print and debug print for Point3D");
	println!("Display: {}", pt);
	println!("Debug: {:?}", pt);

	// Note that trying to print with hex or binary for a user type without implementing its printing function will result in a compilation error.


	let cmplx = Complex(3.3, 7.2);
	println!("Let's compare our print and debug print for Complex");
	println!("Display: {}", cmplx);
	println!("Debug: {:?}", cmplx);

}
