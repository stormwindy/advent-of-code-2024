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

fn find_grid_size(file_path: &str) -> Result<(usize, usize)> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    
    // Collect into a Vec to get line count and first line
    let lines: Vec<_> = reader.lines()
        .map(|line| line.unwrap())
        .collect();

    let line_count = lines.len();
    let first_line_length = lines.first()
        .map_or(0, |line| line.len());

    println!("Number of lines: {}", line_count);
    println!("Length of first line: {}", first_line_length);

    Ok((line_count, first_line_length))
}

fn create_pairs(antenna_locations: &HashMap<char, Vec<(usize, usize)>>) -> HashMap<char, Vec<((usize, usize), (usize, usize))>> {
    let mut result: HashMap<char, Vec<((usize, usize), (usize, usize))>> = HashMap::new();

    for (&antenna, loc_list) in antenna_locations {
        for i in 0..loc_list.len() {
            for j in (i+1)..loc_list.len() {
                result.entry(antenna).or_default().push((loc_list[i].clone(), loc_list[j].clone()));
            }
        }
    }
    result
}

fn is_valid_antinode(antinode: (i32, i32), grid_size: (usize, usize), antenna_1: (i32, i32), antenna_2: (i32, i32)) -> bool {

    if antinode.0 < 0 || antinode.1 < 0 {
        return false
    }

    if antinode.0 >= grid_size.0 as i32 || antinode.1 >= grid_size.1 as i32 {
        return false
    }

    if (antinode == antenna_1 || antinode == antenna_2) {
        return false
    }

    true
}

fn get_antinodes(antenna: (usize, usize), other_antenna: (usize, usize), direction: (i32, i32), grid_size: (usize, usize)) -> Vec<(i32, i32)> {
    // There are four places to check, two from each antenna and one from each antenna is the other antenna. 
    // So we get two valid antinode locations.
    let antenna = (
        antenna.0 as i32,
        antenna.1 as i32
    );
    let other_antenna = (
        other_antenna.0 as i32,
        other_antenna.1 as i32
    );
    let antinode_1 = (antenna.0 + direction.0, antenna.1 + direction.1);
    let antinode_2 = (antenna.0 - direction.0, antenna.1 - direction.1);

    let res_vector = vec![antinode_1, antinode_2];

    let result = res_vector.into_iter().filter(|&an| is_valid_antinode(an, grid_size, antenna, other_antenna)).collect::<Vec<(i32, i32)>>();

    result
}

fn get_antinodes_part2(antenna: (usize, usize), other_antenna: (usize, usize), direction: (i32, i32), grid_size: (usize, usize)) -> Vec<(i32, i32)> {
    let antenna = (
        antenna.0 as i32,
        antenna.1 as i32
    );
    let other_antenna = (
        other_antenna.0 as i32,
        other_antenna.1 as i32
    );

    let mut antinodes: Vec<(i32, i32)> = Vec::new();

    antinodes.push(antenna);
    antinodes.push(other_antenna);

    let mut antinode_1 = (antenna.0 + direction.0, antenna.1 + direction.1);
    while is_valid_antinode(antinode_1, grid_size, antenna, other_antenna) {
        antinodes.push(antinode_1);
        antinode_1 = (antinode_1.0 + direction.0, antinode_1.1 + direction.1);
    }

    let mut antinode_2 = (antenna.0 - direction.0, antenna.1 - direction.1);
    while is_valid_antinode(antinode_2, grid_size, antenna, other_antenna) {
        antinodes.push(antinode_2);
        antinode_2 = (antinode_2.0 - direction.0, antinode_2.1 - direction.1);
    }

    antinodes
}

fn find_antinodes(grid_size: (usize, usize), antenna_pairs: &HashMap<char, Vec<((usize, usize), (usize, usize))>>, is_part2: bool) -> usize {
    let mut antinode_location: HashSet<(i32, i32)> = HashSet::new();

    for (&antenna, pairs) in antenna_pairs {
        for &((antenna_1_0, antenna_1_1), (antenna_2_0, antenna_2_1)) in pairs {
            let direction = (antenna_1_0 as i32 - antenna_2_0 as i32, antenna_1_1 as i32 - antenna_2_1 as i32);
            let antinodes_1 = if !is_part2 {
                get_antinodes((antenna_1_0, antenna_1_1), (antenna_2_0, antenna_2_1), direction, grid_size)
            } else {
                get_antinodes_part2((antenna_1_0, antenna_1_1), (antenna_2_0, antenna_2_1), direction, grid_size)
            };
            for (x, y) in antinodes_1 {
                println!("X {}, Y {}", x, y);
                antinode_location.insert((x, y));
            }
            let antinodes_2 = if !is_part2{
                get_antinodes((antenna_2_0, antenna_2_1), (antenna_1_0, antenna_1_1), direction, grid_size)
            } else {
                get_antinodes_part2((antenna_2_0, antenna_2_1), (antenna_1_0, antenna_1_1), direction, grid_size)
            };
            for (x, y) in antinodes_2 {
                println!("X {}, Y {}", x, y);
                antinode_location.insert((x, y));
            }
        }
    }
    antinode_location.len()
}

fn main() -> Result<()> {
    let input_file = "./src/p8-input.txt";
    let args: Vec<String> = env::args().collect();
    let debug_mode = args.contains(&"--debug".to_string());

    // I know I am reading the file for another time for no reason.
    let grid_size = find_grid_size(&input_file)?;

    let antenna_locations = parse_data(&input_file)?;

    if debug_mode {
        for (antenna, loc_list) in &antenna_locations {
            println!("{}: {:?}", antenna, loc_list);
        }
    }

    println!("Building pairs");
    let all_antenna_pairs = create_pairs(&antenna_locations);

    if debug_mode {
        for (antenna, pairs) in &all_antenna_pairs {
            println!("{}: {:?}", antenna, pairs);
        }
    }

    // let antinodes1 = find_antinodes(grid_size, &all_antenna_pairs, false);
    let antinodes2 = find_antinodes(grid_size, &all_antenna_pairs, true);

    // println!("Part 1 result is {}", antinodes1);
    println!("Part 2 result is {}", antinodes2);


    Ok(())
}