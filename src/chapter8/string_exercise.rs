
fn pig_latin(latin: &str) -> String {
    let mut pig_latin = String::new();
    let words_split: Vec<&str> = latin.split_whitespace().collect();
    for word in words_split {
        let first_char = word.chars().nth(0).unwrap();
        match first_char {
            // Vowels
            'a' | 'e' | 'i' | 'o' | 'u' => {
                pig_latin += format!("{}-hay ", word).as_str();
            }
            // Consonants
            _ => {
                // https://stackoverflow.com/questions/49393462/what-does-str-does-not-have-a-constant-size-known-at-compile-time-mean-and
                pig_latin += format!("{}-{}ay", &word[1..], &first_char).as_str();
            }
        }
    }

    pig_latin
}

pub fn test_pig_latin() {
    println!("\"apple first\" turns into {}", pig_latin("apple first"));
}