fn main()
{
println!("{} days left for vacation!", 31);
println!("{0}, this is {1}. {1}, this is {0}.", "Alex","Jones");
println!("{OH},{MY},{GOD}", OH= "OH", MY="MY", GOD="GOD");
println!("69 in binary is {:b}",69);
println!();

// Should align to the left with 8 whitespaces
println!("{number:>width$}", number =420, width = 8);
// Maybe pad with a few x's?
println!("{number:x>width$}", number =420, width = 8);
}
