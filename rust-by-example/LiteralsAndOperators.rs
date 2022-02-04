// Yoy can express hexadecimal, octal, or binary numbers through
// 0x, 0o, or 0b

// You can also insert underscores in the middle of numeric lietrals to improve readibility 1_000 = 1000, and 0.000_001 = 0.000001

fn main() {
	// Integer additions
	println!("1 + 2 = {}", 1u32 +  2);
	
	// Integer subtraction
	println!("1 - 2 = {}", 1i32 - 2);
	// Note that if we replace i32 by u32, this subtraction will lead to an underflow, the result will be 2^32 - 1

	// Boolean Logic
	println!("true && false is {}", true && false);
	println!("true || false is {}", true || false);
	println!("NOT true is {}", !true);

	// Bitwise operations
	println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
	println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
	println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
	println!("1 << 5 is {}", 1u32 << 5);
	println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

	// Using underscores 
	println!("One million is written as {}", 1_000_000u32);
}

