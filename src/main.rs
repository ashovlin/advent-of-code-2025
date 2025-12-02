mod day1;
mod day2;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let answer = day2::solve().expect("Error while solving");
    println!("Answer for Day 2: {answer}");

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
