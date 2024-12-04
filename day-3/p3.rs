use std::io::{BufReader, BufRead, Result};
use std::fs::File;
use regex::Regex;


fn part1() -> Result<()> {
    let pattern = r"mul\((\d{1,3}),(\d{1,3})\)";
    let regex = Regex::new(pattern).unwrap();
    let file_path = "./src/p3-input.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut total = 0;
    for line in reader.lines() {
        let line = line?;
        for match_res in regex.find_iter(&line) {
            let prefixless = match_res.as_str().strip_prefix("mul(").unwrap();
            let suffixless = prefixless.strip_suffix(")").unwrap();
            let numbers: Vec<i32> = suffixless.split(",")
                .map(|s| s.parse().unwrap())
                .collect();
            total += numbers[0] * numbers[1];
        }
    }
    println!("Part 1 result is {}", total);
    Ok(())
}

fn part2() -> Result<()> {
    let pattern = r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)";
    let regex = Regex::new(pattern).unwrap();
    let file_path = "./src/p3-input.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut total = 0;
    let mut is_enabled = true;
    for line in reader.lines() {
        let line = line?;
        for match_res in regex.find_iter(&line) {
            let match_str = match_res.as_str();
            if match_str == "do()" {
                is_enabled = true;
            } else if match_str == "don't()" {
                is_enabled = false;
            } else if is_enabled {
                let prefixless = match_str.strip_prefix("mul(").unwrap();
                let suffixless = prefixless.strip_suffix(")").unwrap();
                let numbers: Vec<i32> = suffixless.split(",")
                    .map(|s| s.parse().unwrap())
                    .collect();
                total += numbers[0] * numbers[1];
            }
        }
    }
    println!("Part 2 result is {}", total);
    Ok(())
}

fn main() -> Result<()> {
    part2()?;
    Ok(())
}