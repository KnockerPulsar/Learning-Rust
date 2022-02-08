use std::collections::HashMap;

pub fn main() {
    let mut s = String::from("hello");
    s.push_str(", ZA WORDO!");

    print_chars(&count_chars(&s));
}


// Just wetting my feet
// Counts all appearances of characters in a string
// Does not strip or remove non-english characters.
// Note that the hashmap order seems to be random.
fn count_chars(s: &String) -> HashMap<char, u16> {
    let mut char_map: HashMap<char, u16> = HashMap::new();
    for chr in s.chars() {
        let char_ref = char_map.entry(chr).or_insert(0);
        *char_ref += 1;
    }

    return char_map;
}

fn print_chars(char_appearances: &HashMap<char,u16>) {
    println!("{: <10} | {: <12} |", "Character", "Appearances");
    for entry in char_appearances {
        println!("{:<10} | {:<12} |", entry.0, entry.1);
    }
}
