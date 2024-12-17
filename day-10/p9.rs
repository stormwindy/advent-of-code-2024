use std::fs::File;
use std::io::{BufReader, BufRead, Result};
use std::collections::{HashSet, VecDeque, HashMap};


fn parse_file(input_path: &str) -> Result<Vec<Vec<u32>>> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let number_grid: Vec<Vec<u32>> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect()
        })
        .collect();

    Ok(number_grid)
}

fn bfs(map: &Vec<Vec<u32>>, start: (usize, usize)) -> (usize, usize) {
    let mut in_degree: HashMap<(usize, usize), usize> = HashMap::new();
    let mut peaks: HashSet<(usize, usize)> = HashSet::new();
    let mut in_queue = HashSet::new();

    let mut queue = VecDeque::new();

    let row_len = map.len() as i32;
    let col_len = map[0].len() as i32;

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    queue.push_back(start);
    in_queue.insert(start);
    in_degree.insert(start, 1);
    

    while let Some((row, col)) = queue.pop_front() {
        in_queue.remove(&(row, col));  // Remove from tracking set
        let current_paths = *in_degree.get(&(row, col)).unwrap();
        if map[row][col] == 9 {
            peaks.insert((row, col));
        }

        for (dx, dy) in directions.iter() {
            let new_row = row as i32 + dx;
            let new_col = col as i32 + dy;


            if new_row >= 0 && new_row < row_len as i32 && new_col >= 0 && new_col < col_len as i32 {
                let new_pos = (new_row as usize, new_col as usize);

                let current_altitude = map[row][col];
                let new_altitude = map[new_pos.0][new_pos.1];

                // This if allows us to not have infinite loops.
                if new_altitude as i32 - current_altitude as i32 == 1 {
                    *in_degree.entry(new_pos).or_insert(0) += current_paths;
                    if !in_queue.contains(&new_pos) {
                        queue.push_back(new_pos);
                        in_queue.insert(new_pos);
                    }
                }
            }
        }
    }
    (
        peaks.len(),    
        in_degree.iter()
            .filter(|&(pos, _)| map[pos.0][pos.1] == 9)
            .map(|(_, count)| count)
            .sum()
    )
}

fn find_reachable_peaks(map: &Vec<Vec<u32>>) -> (usize, usize) {
    let mut reached_peaks: usize = 0;
    let mut total_paths: usize = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 0 {
                let (new_peaks, part2_paths) = bfs(map, (i as usize, j as usize));
                reached_peaks += new_peaks;
                total_paths += part2_paths;
            }
        }
    }
    (reached_peaks, total_paths)
}


fn main() {
    let map = parse_file("./src/p10-input.txt").unwrap();
    let (part1, part2) = find_reachable_peaks(&map);
    println!("{} {}", part1, part2);
}