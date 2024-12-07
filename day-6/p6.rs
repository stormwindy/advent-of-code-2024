use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashSet;


// We could find the starting location while parsing input but didn't want to complicate code in case we change it.
fn parse_input(filename: &str) -> Result<Vec<Vec<char>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let grid: Vec<Vec<char>> = lines
        .filter_map(Result::ok)
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();
    Ok(grid)
}

fn find_guard(grid: &Vec<Vec<char>>) -> Result<(usize, usize)> {
    for (i, line) in grid.into_iter().enumerate() {
        for (j, &chr) in line.into_iter().enumerate() {
            if chr == '^' {
                return Ok((i, j))
            }
        }
    }
    panic!("No guards found")
}

fn is_in_bounds(grid: &Vec<Vec<char>>, guard_x: &i32, guard_y: &i32) -> bool {
    *guard_x >= 0 && *guard_y >= 0 && *guard_x < grid.len() as i32 && *guard_y < grid[0].len() as i32
}

fn naviagte_and_count_spaces(grid: &Vec<Vec<char>>, (start_x, start_y): (usize, usize)) -> Result<i32> {
    // Directions are based on problem description where guard turns 90 degrees right at each obstacle.
    let directions = vec![
        (-1, 0), // This is the initial direction: UP!
        (0, 1),
        (1, 0),
        (0, -1)
    ];

    let mut cur_direction_index = 0;

    // We could either do underflow error handling or convert to i32 to figure out if guard has left the grid with negative index.
    let mut guard_x: i32 = start_x as i32;
    let mut guard_y: i32 = start_y as i32;
    let mut total_spaces = 0;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    while  is_in_bounds(grid, &guard_x, &guard_y) {
        if !visited.contains(&(guard_x, guard_y)) {
            total_spaces += 1;
        }
        visited.insert((guard_x, guard_y));
        let cur_direction: (i32, i32) = directions[cur_direction_index];
        let possible_x = guard_x + cur_direction.0;
        let possible_y = guard_y + cur_direction.1;

        if is_in_bounds(grid, &possible_x, &possible_y) && grid[possible_x as usize][possible_y as usize] == '#' {
            cur_direction_index = (cur_direction_index + 1) % 4;
        } else {
            guard_x = possible_x;
            guard_y = possible_y;
        }
    }

    Ok(total_spaces)
}

fn part1() -> Result<()> {
    let file_path = "./p6-input.txt";
    let grid = parse_input(&file_path)?;

    let (guard_x, guard_y) = find_guard(&grid)?;
    println!("Guard starts at {}, {}", guard_x, guard_y);

    let result = naviagte_and_count_spaces(&grid, (guard_x, guard_y))?;

    println!("Part 1 result is {}", result);

    Ok(())
}

/*
PART 2 STARTS
I GUESS Imma brute force this one.
*/

fn is_loop(grid: &Vec<Vec<char>>, (start_x, start_y): (usize, usize)) -> bool {
    let directions = vec![
        (-1, 0), // UP
        (0, 1),
        (1, 0),
        (0, -1)
    ];

    let mut cur_direction_index = 0;
    let mut guard_x: i32 = start_x as i32;
    let mut guard_y: i32 = start_y as i32;

    // Use a HashSet to store (position, direction) states
    let mut visited: HashSet<((i32, i32), i32)> = HashSet::new();

    while is_in_bounds(grid, &guard_x, &guard_y) {
        if !visited.insert(((guard_x, guard_y), cur_direction_index)) {
            return true;
        }

        let cur_direction = directions[cur_direction_index as usize];
        let possible_x = guard_x + cur_direction.0;
        let possible_y = guard_y + cur_direction.1;

        if is_in_bounds(grid, &possible_x, &possible_y) && grid[possible_x as usize][possible_y as usize] == '#' {
            cur_direction_index = (cur_direction_index + 1) % 4;
        } else {
            guard_x = possible_x;
            guard_y = possible_y;
        }
    }

    false
}

fn find_loop_positions(grid: &Vec<Vec<char>>, (guard_x, guard_y): (usize, usize)) -> i32 {
    let mut looping_blocks = 0;
    let rows = grid.len();
    let cols = grid[0].len();

    // Check all empty spaces even though we should only care about guard's path and adjecent ones to it. Oopsie.
    for i in 0..rows {
        for j in 0..cols {
            // Skip guard's position and non-empty spaces
            if grid[i][j] != '.' || (i == guard_x && j == guard_y) {
                continue;
            }

            let mut test_grid = grid.clone();
            test_grid[i][j] = '#';
            if is_loop(&test_grid, (guard_x, guard_y)) {
                looping_blocks += 1;
            }
        }
    }

    looping_blocks
}

fn part2() -> Result<()> {
    let file_path = "./p6-input.txt";
    let grid = parse_input(&file_path)?;

    let (guard_x, guard_y) = find_guard(&grid)?;
    println!("Guard starts at {}, {}", guard_x, guard_y);

    let result = find_loop_positions(&grid, (guard_x, guard_y));

    println!("Part 2 result is {}", result);
    Ok(())
}


fn main() -> Result<()> {
    part1()?;
    part2()?;
    Ok(())
}