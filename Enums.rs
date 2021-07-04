enum WebEvent {
	PageLoad, // Unit like
	PageUnload,
	KeyPress(char), // Tuple Struct like 
	Paste(String),
	Click { x: i64, y: i64 } // C Struct like
}

fn inspect(event: WebEvent) {
	// Looks like a switch case....
	match event {
		WebEvent::PageLoad => println!("Page loaded"),
		WebEvent::PageUnload => println!("Page unloaded"),
		// Seems like we're extracting variables from the event in these next three cases..
		WebEvent::KeyPress(c) => println!("Pressed '{}'.", c),
		WebEvent::Paste(s) => println!("Pasted \"{}\".", s),
		WebEvent::Click { x, y } => {
			println!("Clicked at x={}, y={}.", x,y );
		}
	}
}


enum VeryLongNameIGuees {
	Thing1,
	Thing2
}

// Type aliases are ... type aliases, just another name for a type/enum if that's better for code readability
type Ops = VeryLongNameIGuees;

impl VeryLongNameIGuees {
	fn run(&self, x:i32, y:i32) -> i32 {
		// Notice the use of a Self alias!
		match self {
			Self::Thing1 => x+y,
			Self::Thing2 => x-y
		}
	}
}


fn main() {
	let pressed = WebEvent::KeyPress('x');
	// The page says that "to_owned() creates an owned String from a string slice", no idea what that means 
	// I guess that "My text" is a slice and not a real string, so this connverts it to String
	let pasted = WebEvent::Paste("My text".to_owned());
	let click = WebEvent::Click { x:20, y:80 };
	let load = WebEvent::PageLoad;
	let unload = WebEvent::PageUnload;

	inspect(pressed);
	inspect(pasted);
	inspect(click);
	inspect(load);
	inspect(unload);

	// Instead of writing that long name, we just used the alias	
	let _add = Ops::Thing1;
}
