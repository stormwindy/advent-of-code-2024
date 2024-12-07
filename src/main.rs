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

    let mut cur_durection_index = 0;

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
        let cur_direction: (i32, i32) = directions[cur_durection_index];
        let possible_x = guard_x + cur_direction.0;
        let possible_y = guard_y + cur_direction.1;

        if is_in_bounds(grid, &possible_x, &possible_y) && grid[possible_x as usize][possible_y as usize] == '#' {
            cur_durection_index = (cur_durection_index + 1) % 4;
            let new_direction = directions[cur_durection_index];
            guard_x = guard_x + new_direction.0;
            guard_y = guard_y + new_direction.1; 
        } else {
            guard_x = possible_x;
            guard_y = possible_y;
        }
    }

    Ok(total_spaces)
}

fn part1() -> Result<()> {
    let file_path = "./src/p6-input.txt";
    let grid = parse_input(&file_path)?;

    let (guard_x, guard_y) = find_guard(&grid)?;
    println!("Guard starts at {}, {}", guard_x, guard_y);

    let result = naviagte_and_count_spaces(&grid, (guard_x, guard_y))?;

    println!("Part 1 result is {}", result);

    Ok(())
}


fn main() -> Result<()> {
    part1()?;
    Ok(())
}