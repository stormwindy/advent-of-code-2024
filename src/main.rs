use std::fs::File;
use std::io::{BufRead, BufReader, Result, Error, ErrorKind};
use std::num::ParseIntError;
use std::collections::{HashMap, HashSet};


fn part1() -> Result<()> {
    let file_path = "./src/p5-input.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut reading_rules = true;
    let mut rules_map: HashMap<i32, HashSet<i32>> = HashMap::new();

    let mut result = 0;

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            reading_rules = false;
            continue;
        }

        if reading_rules {
            let (left, right) = line.split_once('|').unwrap();
            let left_num = left.parse::<i32>().unwrap();
            let right_num = right.parse::<i32>().unwrap();

            rules_map.entry(left_num).or_default().insert(right_num);

        } else {
            let update_pages: Vec<i32> = line
                .split(',')                     // Split the string at commas
                .map(|s| s.trim())             // Remove any whitespace around each number
                .filter(|s| !s.is_empty())     // Remove any empty entries
                .map(|s| s.parse().map_err(|e: ParseIntError| {
                    Error::new(ErrorKind::InvalidData, e.to_string())
                }))
                .collect::<Result<Vec<i32>>>()?;

            let mut seen_set: HashSet<i32> = HashSet::new();
            let mut valid = true;
            for page in &update_pages {
                let not_allowed_before_page = rules_map.get(&page);
                if let Some(set) = &not_allowed_before_page {
                    for seen_page in &seen_set {
                        if set.contains(&seen_page) {
                            valid = false;
                        }
                    }
                    seen_set.insert(*page);
                }
            }

            if valid {
                let mid_point: i32 = update_pages.len() as i32 / 2;
                result += update_pages[mid_point as usize]
            }
        }
    }
    println!("Part 1 result is {}", result);
    Ok(())
}

fn main() -> Result<()> {
    part1()?;
    Ok(())
}