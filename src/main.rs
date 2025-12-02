mod day1;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let answer = day1::solve(true).expect("Error while solving");
    println!("Answer for Day 1: {answer}");

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
