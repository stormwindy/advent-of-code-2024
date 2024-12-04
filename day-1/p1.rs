use std::fs::File;
use std::io::{self, BufReader, Result, BufRead};
use std::num::ParseIntError;
use std::collections::HashMap;

use std::collections::BinaryHeap;

fn read_file_into_heap(file_path: &str) -> Result<(BinaryHeap<i32>, BinaryHeap<i32>)> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut left_col = BinaryHeap::new();
    let mut right_col = BinaryHeap::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()  // Split on any whitespace (tabs or spaces)
            .filter(|s| !s.is_empty()) // get rid of empty spaces since I am a rust noob and have no clue how to get rid of them any other way.
            .map(|s| s.parse().map_err(|e: ParseIntError| {
                io::Error::new(io::ErrorKind::InvalidData, e.to_string())
            }))
            .collect::<io::Result<Vec<i32>>>()?;
        
        left_col.push(numbers[0]);
        right_col.push(numbers[1]);
    }
    Ok((left_col, right_col))
}

fn part1(file_path: &str) -> Result<()> {
    let (mut left_heap, mut right_heap) = read_file_into_heap(file_path)?;
    assert_eq!(left_heap.len(), right_heap.len());
    let mut total_len = 0;
    while let Some(l_val) = left_heap.pop() {
        let r_val = right_heap.pop().unwrap();
        let diff = l_val - r_val;
        total_len += diff.abs()
    }
    println!("Part 1 result is {}", total_len);
    Ok(())
}

fn read_file_into_hashmap_count(file_path: &str) -> Result<(HashMap<i32, i32>, HashMap<i32, i32>)> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut left_count: HashMap<i32, i32> = HashMap::new();
    let mut right_count: HashMap<i32, i32> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()  // Split on any whitespace (tabs or spaces)
            .filter(|s| !s.is_empty()) // get rid of empty spaces since I am a rust noob and have no clue how to get rid of them any other way.
            .map(|s| s.parse().map_err(|e: ParseIntError| {
                io::Error::new(io::ErrorKind::InvalidData, e.to_string())
            }))
            .collect::<io::Result<Vec<i32>>>()?;
        
        left_count.entry(numbers[0]).and_modify(|e| *e += 1).or_insert(1);
        right_count.entry(numbers[1]).and_modify(|e| *e += 1).or_insert(1);
    }

    Ok((left_count, right_count))
}

fn part2(file_path: &str) -> Result<()> {
    let (left_count, right_count) = read_file_into_hashmap_count(file_path)?;
    let mut total = 0;
    for (key, left_val) in &left_count {
        let right_val = right_count.get(key).unwrap_or(&0);
        total += left_val * right_val * key;
    }

    println!("Part 2 result is {}", total);
    Ok(())
}

fn main() -> Result<()> {
    let input_path = "./p1-input.txt";
    part1(input_path)?;
    part2(input_path)?;
    Ok(())
}