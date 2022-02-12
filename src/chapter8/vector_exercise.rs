use rand::Rng;
use std::collections::HashMap;

fn median(vec: &Vec<i32>) -> i32 {
    let mut sorted_vec = vec.clone();
    sorted_vec.sort();

    let median_index = sorted_vec.len() / 2 - 1;

    sorted_vec[median_index]
}

fn mode(numbers: &Vec<i32>) -> i32 {
    let mut occurence_map: HashMap<i32, u32> = HashMap::new();
    for number in numbers.iter() {
        let num_occur = occurence_map.entry(*number).or_insert(0);
        *num_occur += 1;
    }

    // Compare the map's entries by comparing the values
    // This gets us the entry with the most occurences
    // We then return its key, the number corresponding to the maximum occurences. The mode.
    let mode = *occurence_map.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().0;
    mode
}

fn gen_rand_vec(num_elements: u32, range_lower: i32, range_upper: i32) -> Vec<i32> {
    let mut random_numbers: Vec<i32> = Vec::new();

    let mut rng = rand::thread_rng();
    for _ in 0..num_elements {
        random_numbers.push(rng.gen_range(range_lower..range_upper));
    }

    random_numbers
}

pub fn test_median() {
    let random_numbers = gen_rand_vec(20u32, -30, 70);
    println!("Original array: {:#?}", random_numbers);
    println!("Median value {}", median(&random_numbers));
}

pub fn test_mode() {
    // Since the number of elements is larger than our range. We're guaranteed we'll get some duplicates
    let random_numbers = gen_rand_vec(40u32, -10, 20);
    println!("Original array:{:#?}", random_numbers);
    println!("Mode:{}", mode(&random_numbers));
}
