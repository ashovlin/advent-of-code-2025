mod day1;
mod day2;
mod day3;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let answer = day3::solve().expect("Error while solving");
    println!("Answer for Day 3: {answer}");

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
