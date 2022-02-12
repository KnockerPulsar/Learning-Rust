#![allow(dead_code)]

pub mod map_exercise;
pub mod vector_exercise;
pub mod string_exercise;

// Note: You cannot have both mutable and immutable references in the same scope.
fn immutable_vs_mutable_refs() {
    #[allow(unused_mut)]
    let mut v = vec![1, 2, 3, 4, 5, 6];

    // Get immutable reference to the first element
    let first = &v[0];

    // Mutate the vector
    // Errors out the compilation because we already
    // have an immutable reference to the first element.
    // v.push(6);

    println!("The first element is: {}", first);
}

fn immatble_for_loop() {
    let meme = vec![
        "You",
        "were",
        "expecting",
        "a",
        "loop.",
        "But",
        "it",
        "was",
        "me, ",
        "DIO",
    ];

    for word in meme {
        println!("{}", word);
    }
}
