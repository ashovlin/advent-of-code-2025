use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve(remove_as_we_go: bool) -> std::io::Result<i32> {
    let f = File::open("src/day4/input.txt").expect("No input file");
    let reader = BufReader::new(f);

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut result: i32 = 0;

    for line in reader.lines() {
        let line = line.expect("Error while reading line");
        let mut vec = Vec::new();

        for char in line.chars() {
            vec.push(char);
        }

        grid.push(vec);
        println!("{}", line);
    }

    let mut removed_this_round = true;

    while removed_this_round {
        let mut row = 0;
        removed_this_round = false;

        while row < grid.len() {
            let mut col = 0;

            while col < grid[0].len() {
                if grid[row][col] == '@' && is_accessible(&mut grid, row, col, remove_as_we_go) {
                    result += 1;

                    // If part 2, and we removed at least 1,
                    // iterate at least one more time
                    if remove_as_we_go {
                        removed_this_round = true;
                    }
                }
                col += 1;
            }
            row += 1;
        }
    }

    return Ok(result);
}

fn is_accessible(grid: &mut Vec<Vec<char>>, row: usize, col: usize, remove_as_we_go: bool) -> bool {
    let mut neighbors: i8 = 0;

    for diff in [
        (-1i32, 0i32), // top
        (-1, 1),       // top right
        (0, 1),        // right
        (1, 1),        // bottom right
        (1, 0),        // below
        (1, -1),       // bottom left
        (0, -1),       // left
        (-1, -1),      // top left
    ] {
        let neighbor_row: i32 = row as i32 + diff.0;
        let neighbor_col: i32 = col as i32 + diff.1;

        if neighbor_row < 0 || neighbor_row as usize >= grid.len() {
            continue;
        }

        if neighbor_col < 0 || neighbor_col as usize >= grid[0].len() {
            continue;
        }

        if grid[neighbor_row as usize][neighbor_col as usize] != '.' {
            neighbors += 1;
        }

        if neighbors >= 4 {
            return false;
        }
    }

    if remove_as_we_go {
        grid[row][col] = '.';
    }
    return true;
}
