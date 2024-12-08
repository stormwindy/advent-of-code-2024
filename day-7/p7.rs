use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::{HashMap};
use std::env;


fn parse_file(input_path: &str) -> Result<HashMap<u128, Vec<u128>>> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut map = HashMap::new();
    
    for line in lines {
        let line = line?;
        if let Some((key_str, values_str)) = line.split_once(':') {
            if let Ok(key) = key_str.trim().parse::<u128>() {
                let values: Vec<u128> = values_str
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok()) // Dangereous. If we were parsing into u32, we wouldn't see overflows.
                    .collect();

                map.insert(key, values);
            }
        }
    }
    Ok(map)
}

fn create_combination(input_len: u32, combination_idx: u128, concat_allowed: bool) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    let mut remaining = combination_idx;
    for j in 0..(input_len - 1) {
        if !concat_allowed {
            res.push(((combination_idx >> j) & 1) as u8);
        } else {
            res.push((remaining % 3) as u8);
            remaining /= 3;
        }
    }
    res
}

fn check_possible(target: u128, feed_list: Vec<u128>, concat_allowed: bool, debug_mode: bool) -> Result<u128> {

    let feed_list_len = feed_list.len() as u32;
    let total_combinations = if !concat_allowed {
        2_u128.pow(feed_list_len - 1)
    } else {
        3_u128.pow(feed_list_len - 1)
    };

    if debug_mode {
        println!("Checking target: {}", target);
        println!("Feed list: {:?}", feed_list);
        println!("Total combinations to check: {}", total_combinations);
    }


    for cur_combination in 0..total_combinations {
        let combination: Vec<u8> = create_combination(feed_list_len, cur_combination as u128, concat_allowed);

        if debug_mode && cur_combination < 500 {
            println!("Combination {}: {:?}", cur_combination, combination);
        }


        let mut j = 1;
        let mut cur_res: u128 = feed_list[0] as u128;
        while j < feed_list_len {
            let cur_op = combination[(j - 1) as usize];
            if cur_op == 1 {
                cur_res += feed_list[j as usize] as u128;
            } else if cur_op == 2 {
                cur_res = merge_nums(cur_res, feed_list[j as usize])
            } else {
                cur_res *= feed_list[j as usize] as u128;
            }
            j += 1;
        }

        if cur_res == target as u128 {
            return Ok(cur_res);
        }
    }

    Ok(0_u128)
}

fn part1(debug_mode: bool) -> Result<u128> {
    let file_path = "./src/p7-input.txt";
    let parsed_data = parse_file(&file_path)?;

    let mut possible_correct: u128 = 0;

    for (target, feed_list) in parsed_data {
        possible_correct += check_possible(target, feed_list, false, debug_mode)?;
    }

    Ok(possible_correct)
}

fn merge_nums(x: u128, y: u128) -> u128 {

    let concatenated = format!("{}{}", x, y)
        .parse::<u128>()
        .expect("Failed to parse concatenated number");

    concatenated
}

fn part2(debug_mode: bool) -> Result<u128> {
    let file_path = "./src/p7-input.txt";
    let parsed_data = parse_file(&file_path)?;

    let mut possible_correct: u128 = 0;

    for (target, feed_list) in parsed_data {
        possible_correct += check_possible(target, feed_list, true, debug_mode)?;
    }

    Ok(possible_correct)
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let debug_mode = args.contains(&"--debug".to_string());

    let part_1_result = part1(debug_mode).unwrap();
    println!("Part 1 solution is {}", part_1_result);
    let part_2_result = part2(debug_mode).unwrap();
    println!("Part 2 result is {}", part_2_result);
    Ok(())
}