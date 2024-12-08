use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::{HashMap, HashSet};
use std::env;


fn parse_data(file_path: &str) -> Result<HashMap<char, Vec<(usize, usize)>>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut antenna_locations: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (line_num, line) in lines.enumerate() {
        let line = line?;

        for (col_num, chr) in line.chars().enumerate() {
            if chr != '.' {
                antenna_locations.entry(chr).or_default().push((line_num, col_num));
            }
        } 

    }

    Ok(antenna_locations)
}

fn create_pairs(antenna_locations: &HashMap<char, Vec<(usize, usize)>>) -> HashMap<char, Vec<Vec<((usize, usize), (usize, usize))>>> {
    let mut result: HashMap<char, Vec<Vec<((usize, usize), (usize, usize))>>> = HashMap::new();

    for (&antenna, loc_list) in antenna_locations {
        for i in 0..loc_list.len() {
            for j in (i+1)..loc_list.len() {

                result.entry(antenna).or_default().push(vec![(loc_list[i].clone(), loc_list[j].clone())]);
            }
        }
    }
    result
}

fn find_antinodes(grid_size: (usize, usize), antenna_pairs: &HashMap<char, Vec<Vec<((usize, usize), (usize, usize))>>>) -> u32 {
    let antinode_location: HashSet<(usize, usize)> = HashSet::new();

    for (&antenna, pairs) in antenna_pairs {
        for (antenna_1, antenna_2) in pairs {
            // TODO
        }
    }
}

fn main() -> Result<()> {
    let input_file = "./src/p8-input.txt";
    let args: Vec<String> = env::args().collect();
    let debug_mode = args.contains(&"--debug".to_string());

    let antenna_locations = parse_data(&input_file)?;

    if debug_mode {
        for (antenna, loc_list) in &antenna_locations {
            println!("{}: {:?}", antenna, loc_list);
        }
    }

    println!("Building pairs");
    let all_antenna_pairs = create_pairs(&antenna_locations);

    if debug_mode {
        for (antenna, pairs) in all_antenna_pairs {
            println!("{}: {:?}", antenna, pairs);
        }
    }

    Ok(())
}