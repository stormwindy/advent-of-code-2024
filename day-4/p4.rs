use std::io::{Result, BufRead, BufReader};
use std::fs::File;
use std::collections::{VecDeque, HashSet};

const DIRECTIONS: [(i32, i32); 8] = [
    (0, 1),   // right
    (1, 0),   // down
    (-1, 0),  // up
    (0, -1),  // left
    (-1, -1), // up-left
    (1, 1),   // down-right
    (-1, 1),  // up-right
    (1, -1)   // down-left
];

fn bfs((start_x, start_y): (i32, i32), grid: &Vec<Vec<char>>) -> Result<i32> {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    // Now we store direction index along with position and string
    let mut deque: VecDeque<(i32, i32, String, Option<usize>)> = VecDeque::new();

    // Initial state: no direction chosen yet (None)
    deque.push_back((start_x, start_y, "".to_string(), None));
    let mut xmas_count = 0;

    while let Some((x, y, cur_str, dir_idx)) = deque.pop_front() {
        if x >= 0 && y >= 0 && x < grid.len() as i32 && y < grid[0].len() as i32 && !visited.contains(&(x, y)) {
            let new_str = cur_str.to_owned() + &grid[x as usize][y as usize].to_string();
            if new_str == "XMAS" {
                xmas_count += 1;
            } else if new_str.len() < 4 {
                match dir_idx {
                    // If no direction chosen yet, try all directions
                    None => {
                        for (idx, &(d_x, d_y)) in DIRECTIONS.iter().enumerate() {
                            let new_x = x + d_x;
                            let new_y = y + d_y;
                            deque.push_back((new_x, new_y, new_str.to_owned(), Some(idx)));
                        }
                    },
                    // If direction already chosen, continue only in that direction
                    Some(idx) => {
                        let (d_x, d_y) = DIRECTIONS[idx];
                        let new_x = x + d_x;
                        let new_y = y + d_y;
                        deque.push_back((new_x, new_y, new_str.to_owned(), Some(idx)));
                    }
                }
            }
            visited.insert((x, y));
        }
    }
    Ok(xmas_count)
}

// Waaaay to complicated for no reason but I was trying to figure out how to use Rust.
fn part1() -> Result<()> {
    let file_path = "./src/p4-input.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }

    let mut total = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            if ch == 'X' {
                total += bfs((i as i32, j as i32), &grid)?;
            }
        }
    }

    println!("Part 1 solution is {}", total);
    Ok(())
}

fn part2() -> Result<()> {
    let file_path = "./src/p4-input.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }

    let mut total = 0;

    for i in 1..grid.len()-1 {
        for j in 1..grid[i].len()-1 {
            if grid[i][j] == 'A' {
                let cross_one = format!("{}{}{}", 
                    grid[i-1][j-1], 'A', grid[i+1][j+1]);
                let cross_two = format!("{}{}{}",
                    grid[i+1][j-1], 'A', grid[i-1][j+1]);

                // Create strings for comparison
                let mas = String::from("MAS");
                let sam = String::from("SAM");

                if (cross_one == mas || cross_one == sam) && 
                   (cross_two == mas || cross_two == sam) {
                    total += 1;
                }
            }
        }
    }

    println!("Part 2 solution is {}", total);
    Ok(())
}

fn main() -> Result<()> {
    part1()?;
    part2()?;
    Ok(())
}