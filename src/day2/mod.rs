use std::fs;

pub fn solve() -> std::io::Result<i64> {
    let input = fs::read_to_string("src/day2/input.txt").expect("Couldn't open file");
    let mut result: i64 = 0;

    for range in input.split(',') {
        let parts = range.split("-").collect::<Vec<&str>>();
        let start: i64 = parts[0]
            .trim()
            .parse::<i64>()
            .expect("Couldn't parse start");
        let end: i64 = parts[1].trim().parse::<i64>().expect("Couldn't parse end");

        let mut i = start;

        while i <= end {
            if is_invalid_part_2(i.to_string()) {
                result += i;
            }

            i += 1;
        }
    }
    return Ok(result);
}

fn is_invalid_part_1(number: String) -> bool {
    if number.len() % 2 != 0 {
        return false;
    }

    let half = number.len() / 2;
    let front = &number[0..half];
    let back = &number[half..];

    if front == back {
        return true;
    }

    return false;
}

fn is_invalid_part_2(number: String) -> bool {
    let max_length = number.len() / 2;
    let mut current_length = 1;

    while current_length <= max_length {
        // If it's not an even number of parts, skip this length
        if number.len() % current_length != 0 {
            current_length += 1;
            continue;
        }

        let first_piece = &number[0..current_length];
        let mut piece_start_index = current_length;

        while piece_start_index < number.len() {
            let current_piece = &number[piece_start_index..piece_start_index + current_length];

            if current_piece != first_piece {
                current_length += 1;
                break;
            }
            piece_start_index += current_length;
        }

        // If we made it to the end, all pieces matched
        if piece_start_index >= number.len() {
            return true;
        }
    }
    return false;
}
