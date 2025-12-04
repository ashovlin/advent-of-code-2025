use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() -> std::io::Result<i64> {
    let f = File::open("src/day3/input.txt").expect("No input file");
    let reader = BufReader::new(f);
    let mut result: i64 = 0;

    for line in reader.lines() {
        let line = line.expect("Error while reading line");
        result += max_voltage_window(line, 12);
    }

    return Ok(result);
}

fn max_voltage_window(battery: String, size: u8) -> i64 {
    let battery_bytes = battery.as_bytes();
    let mut index: usize = 0;
    let mut result: i64 = 0;
    let mut right_bound: usize = battery.len() - size as usize;

    println!("{battery}");

    while right_bound < battery.len() {
        let (new_digit, largest_index) = largest_in_window(battery_bytes, index, right_bound);

        result *= 10;
        result = result + new_digit as i64;

        index = largest_index + 1;
        right_bound += 1;
    }

    println!("{battery}-> {result}");
    return result;
}

fn largest_in_window(battery: &[u8], start_index: usize, right_bound: usize) -> (u8, usize) {
    let mut largest: u8 = battery[start_index] - 48;
    let mut largest_index: usize = start_index;

    let mut index = start_index + 1;

    while index <= right_bound {
        let candidate = battery[index] - 48;

        if candidate > largest {
            largest = candidate;
            largest_index = index;
        }

        index += 1;
    }
    return (largest, largest_index);
}
