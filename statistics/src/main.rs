use std::collections::HashMap;

fn get_mean(numbers: &[i32]) -> f32 {
    let length = numbers.len();
    let sum:i32 = numbers.iter().sum();
    sum as f32 / length as f32
}

fn get_median (numbers: &mut [i32]) -> i32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

fn get_mode (numbers: &[i32]) -> i32 {
    let mut map = HashMap::new();

    for &i in numbers {
        *map.entry(i).or_insert(0) += 1;
    }

    map.into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers")
}

fn main() {
    // Given a list of integers, use a vector and return the mean (average), median (when sorted, the value in the middle position), and mode (the value that occurs most often; a hash map will be helpful here) of the list.
    let mut list = [1, 6, 3, 5, 7, 5, 6, 5, 9];

    println!("mean {:?}", get_mean(&list));
    println!("median {:?}", get_median(&mut list));
    println!("mode {:?}", get_mode(&list));
}
