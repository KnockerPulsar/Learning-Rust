// Normal struct
#[derive(Debug)]
struct Person {
	name: String,
	age: u8
}

// Unit struct
struct UnitStruct;

// Tuple struct
struct Pair(i32,f32);

// We had to tell the complier to both implement Copy() and Clone() to be able to use the rectangle constructor since it uses a point, not a reference to a point
#[derive(Debug, Copy, Clone)]
struct Point {
	x: f32,
	y: f32
}

#[allow(dead_code)]
#[derive(Debug)]
struct Rect {
	// We can nest structs
	top_left : Point,
	bottom_right: Point
}

// Activity 1
fn rect_area( rect : Rect ) -> f32 {
	let width = (rect.top_left.x - rect.bottom_right.x).abs();
	let len =  (rect.top_left.y - rect.bottom_right.y).abs();
    len*width	
}

// Activity 2
fn square(pt : Point, size : f32) -> Rect {
	return Rect { 
			top_left: pt,
			bottom_right : Point { x: pt.x + size, y: pt.y + size }
	};
}
fn main() {
	let name = String::from("Tarek");
	let age = 21;
	let tarek = Person {name, age};
	
	println!("{:?}", tarek);

	let point: Point = Point { x: 10.3, y: 0.4 };
	println!("Point coords: ({},{})", point.x, point.y);

	// Using "struct update" syntax to use the fields of another object of the same type
	let bottom_right = Point { x: 5.2, ..point };

	println!("Second point: ({},{})", bottom_right.x, bottom_right.y);


	// Destructuring the point through "let"
	// I think this will put top_edge = point.x and left_edge = point.y
	let Point {x: top_edge, y: left_edge } = point;

	let _rect = Rect { 
		top_left: Point { x: left_edge, y: top_edge },
		bottom_right: bottom_right
	};

	println!("Rectangle area is {}", rect_area(_rect));

	let pt = Point {x: 1.32, y:4.88};
	let size = 4.0;

	println!("{:?}", square(pt, size));

	let _unit = UnitStruct;

	let pair = Pair(1,0.1);

	println!("Pair contains {:?} and {:?}", pair.0, pair.1);
	let Pair(int, dec) = pair;

	println!("The pair contains {:?} and {:?}", int,dec);
}
