// I'd say this line just imports the specified parts
use std::fmt::{self, Formatter, Display};

struct City {
	// I think 'static means that the string will exist for the lifetime of the program, or at least tells the compiler that
	name : &'static str,	
	lat: f32,
	lon: f32,
}


impl Display for City {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let lat_c = if self.lat >= 0.0 {'N'} else {'S'};
		let lon_c = if self.lon >= 0.0 {'E'} else {'W'};

		// The {:.3} is probably to print just 3 decimal places
		write!(f, "{}: {:.3}°{} {:.3}°{}", self.name, self.lat.abs(),
			lat_c, self.lon.abs(), lon_c)
		}
}


#[derive(Debug)]
struct Color {
	red: u8,
	green: u8,
	blue: u8,
}

impl Display for Color {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// Ok, so for multiple write!() calls, we should use the ? for all calls except the last one. This function could also be done with one call but I preferred to split it up.
		write!(f, "RGB ({}, {}, {}) ", self.red, self.green, self.blue)?;
		write!(f, "0x{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
	}
}


fn main() {
	for city in [ 
		City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
	].iter() {
		println!("{}", *city);
	}

	for color in [
		Color { red: 128, green: 255, blue: 90},
		Color { red: 0, green: 3, blue: 254},
		Color { red: 0, green: 0, blue: 0},
	].iter() {
		println!("{}", *color);
	}
}

