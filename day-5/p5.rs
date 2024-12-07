use std::fs::File;
use std::io::{BufRead, BufReader, Result, Error, ErrorKind};
use std::num::ParseIntError;
use std::collections::{HashMap, HashSet, VecDeque};


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


/**
SOLUTION FOR PART 2 BELOW
Assumes each update is a DAG which can be sorted topologically. And it seems to be the right assumption to solve the question.
**/

// Clean up parsing to make it easier to see what is going on in the solution method.
fn parse_input(filename: &str) -> Result<(HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>)> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    
    let mut updates = Vec::new();
    let mut parsing_rules = true;

    let mut rules_map: HashMap<i32, HashSet<i32>> = HashMap::new();

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            parsing_rules = false;
            continue;
        }

        if parsing_rules {
            let nums: Vec<i32> = line.split('|')
                .map(|n| n.parse().unwrap())
                .collect();
            rules_map.entry(nums[0]).or_default().insert(nums[1]);
        } else {
            let update: Vec<i32> = line.split(',')
                .map(|n| n.parse().unwrap())
                .collect();
            updates.push(update);
        }
    }

    Ok((rules_map, updates))
}

fn is_valid_update(update: &Vec<i32>, rules: &HashMap<i32, HashSet<i32>>) -> Result<bool> {
    let mut seen_set: HashSet<i32> = HashSet::new();

    for num in update {
        let prohobited_list = rules.get(&num).unwrap();
        for prohibited_num in prohobited_list {
            if seen_set.contains(&prohibited_num) {
                return Ok(false);
            }
        }
        seen_set.insert(*num);
    }
    Ok(true)
}

fn calculate_middle(update: Vec<i32>) -> Result<i32> {
    let mid_point: usize = (update.len() as i32 / 2) as usize;
    Ok(update[mid_point])
}

fn topological_sort(update: &Vec<i32>, rules: &HashMap<i32, HashSet<i32>>) -> Result<i32> {
    let mut in_degree: HashMap<i32, i32> = HashMap::new();

    let mut result: Vec<i32> = Vec::new();

    for num in update {
        if let Some(after_set) = rules.get(&num) {
            let after_list: Vec<i32> = after_set
                .iter()
                .filter(|&&after_num| update.contains(&after_num))
                .cloned()
                .collect();

            // We want to know 0 values as well. So initialize all.
            in_degree.entry(*num).or_insert(0);
            for has_in_num in after_list {
                *in_degree.entry(has_in_num).or_insert(0) += 1;
            }
        }
    }

    let mut free_to_add: VecDeque<i32> = update.iter()
        .copied()
        .filter(|&n| in_degree[&n] == 0)
        .collect();

    let mut visited: HashSet<i32> = HashSet::new();
    while let Some(free_num) = free_to_add.pop_front() {
        if visited.contains(&free_num) {
            continue;
        }

        result.push(free_num);
        visited.insert(free_num);

        // let mut new_to_discover = rules.get(&free_num).filter(|n| update.contains(&n) && !visited.contains(&n)).collect();

        let new_to_discover = if let Some(next_nums) = rules.get(&free_num) {
            next_nums
                .iter()
                .filter(|&&n| update.contains(&n) && !visited.contains(&n))
                .cloned()
                .collect()
        } else {
            Vec::new()
        };
        for new_num in new_to_discover {
            *in_degree.get_mut(&new_num).unwrap() -= 1;
            if *in_degree.get(&new_num).unwrap() == 0 {
                free_to_add.push_back(new_num);
            }
        }
    }

    let middle_num = calculate_middle(result).unwrap();
    Ok(middle_num)
}

fn part2() -> Result<()> {
    let file_path = "./src/p5-input.txt";
    let (rules, updates) = parse_input(file_path)?;

    let mut total = 0;

    for update in updates {
        if !is_valid_update(&update, &rules).unwrap() {
            total += topological_sort(&update, &rules).unwrap();
        }
    }

    println!("Result for part 2 is {}", total);
    Ok(())
}

fn main() -> Result<()> {
    part1()?;
    part2()?;
    Ok(())
}