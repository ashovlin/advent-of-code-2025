use std::fs::File;
use std::io::{BufRead, BufReader, Result};

pub fn solve(count_all_zeroes: bool) -> Result<i32> {
    let f = File::open("src/day1/input.txt").expect("No input file");
    let reader = BufReader::new(f);

    let mut dial_position: i32 = 50;
    let mut result = 0;

    for line in reader.lines() {
        let line = line.expect("Unable to read line");

        let direction = line.chars().nth(0).expect("Invalid direction");
        let magnitude = &line[1..];
        let num: i32 = magnitude.parse().expect("Not a number!");

        let diff = if direction == 'R' { 1 } else { -1 };
        let mut steps = num.abs();

        // Brute force, simulating each click
        while steps > 0 {
            dial_position += diff;

            if dial_position == 100 {
                dial_position = 0;
            } else if dial_position == -1 {
                dial_position = 99;
            }

            if count_all_zeroes && dial_position == 0 {
                result += 1
            }

            steps -= 1;
        }

        if !count_all_zeroes && dial_position == 0 {
            result += 1;
        }
    }
    return Ok(result);
}
