// Given a list of integers, use a vector and return the median (when sorted,
// the value in the middle position) and mode (the value that occurs most often;
// a hash map will be helpful here) of the list.

use std::collections::HashMap;
use std::io;

fn main() {
    // let vec = vec![2, 10, 5, 4, 3];
    // let vec = vec![];
    // let vec = vec![7, 5, 7, 3];

    let vec = read_input();

    match median(&vec) {
        Some(m) => println!("Median: {}", m),
        None => println!("Median: Not found!"),
    }

    match mode(&vec) {
        Some(i) => println!("Mode: {}", i),
        None => println!("Mode: Not found!"),
    }
}

fn read_input() -> Vec<i32> {
    println!("Enter integers separated by spaces.");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn median(vec: &Vec<i32>) -> Option<i32> {
    let mut v = vec.clone();
    v.sort();
    println!("Sorted: {:?}", v);
    let len = v.len();
    let mut median: Option<i32> = None;
    if len > 0 {
        let mid = len / 2;
        if v.len() % 2 == 0 {
            median = Some((v[mid - 1] + v[mid]) / 2);
        } else {
            median = Some(v[mid]);
        }
    }
    median
}

fn mode(vec: &Vec<i32>) -> Option<i32> {
    let mut map = HashMap::new();
    for i in vec {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    let mut max_key: Option<i32> = None;
    let mut max_val = 0;
    for (key, val) in map {
        if val > max_val {
            max_val = val;
            max_key = Some(*key);
        }
    }
    max_key
}
