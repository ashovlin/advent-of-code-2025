mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let answer = day4::solve(true).expect("Error while solving");
    println!("Answer for Day 4: {answer}");

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
