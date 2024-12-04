use std::io::{self, BufReader, Result, BufRead};
use std::fs::File;
use std::num::ParseIntError;



fn part1() -> Result<()> {
    let file_path = "./p2-input.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut safe: u32 = 0;

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()  // Split on any whitespace (tabs or spaces)
            .filter(|s| !s.is_empty()) // get rid of empty spaces since I am a rust noob and have no clue how to get rid of them any other way.
            .map(|s| s.parse().map_err(|e: ParseIntError| {
                io::Error::new(io::ErrorKind::InvalidData, e.to_string())
            }))
            .collect::<io::Result<Vec<i32>>>()?;
        
        if numbers.len() == 1 {
            safe += 1;
            continue;
        }
        if numbers[0] < numbers[1] {
            if numbers.windows(2).all(|pair| pair[1] - pair[0] <= 3 && pair[0] < pair[1]) {
                safe += 1;
            };
        } else if numbers[0] > numbers[1] {
            if numbers.windows(2).all(|pair| pair[0] - pair[1] <= 3 && pair[1] < pair[0]) {
                safe += 1;
            };
        } else {
            continue;
        }
    }

    println!("Part 1 safe count {}", safe);
    Ok(())
}

fn generate_variations(vec: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut variations = Vec::with_capacity(vec.len());
    for skip_index in 0..vec.len() {
        let variation: Vec<i32> = vec.iter()
            .enumerate()
            .filter(|(i, _)| *i != skip_index)
            .map(|(_, element)| element.clone())
            .collect();
        variations.push(variation);
    }
    variations
}

fn test_safety(numbers: &Vec<i32>) -> bool {
    if numbers.len() == 1 {
        return true;
    }
    if numbers[0] < numbers[1] {
        if numbers.windows(2).all(|pair| pair[1] - pair[0] <= 3 && pair[0] < pair[1]) {
            return true;
        };
    } else if numbers[0] > numbers[1] {
        if numbers.windows(2).all(|pair| pair[0] - pair[1] <= 3 && pair[1] < pair[0]) {
            return true;
        };
    }
    false
}

fn part2() -> Result<()> {
    let file_path = "./p2-input.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut safe: u32 = 0;

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()  // Split on any whitespace (tabs or spaces)
            .filter(|s| !s.is_empty()) // get rid of empty spaces since I am a rust noob and have no clue how to get rid of them any other way.
            .map(|s| s.parse().map_err(|e: ParseIntError| {
                io::Error::new(io::ErrorKind::InvalidData, e.to_string())
            }))
            .collect::<io::Result<Vec<i32>>>()?;
        if test_safety(&numbers) {
            safe += 1;
        } else {
            let variations = generate_variations(&numbers);
            let is_safe = variations.iter().any(|nums| test_safety(nums));
            if is_safe {
                safe += 1;
            };
        }
    }

    println!("Part 2 safe count {}", safe);
    Ok(())
}

fn main() -> Result<()> {
    part1()?;
    part2()?;
    Ok(())
}