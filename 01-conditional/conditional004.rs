use std::io;

fn main() {
    let mut score = String::new();
    println!("Enter in your score");
    io::stdin().read_line(&mut score).expect("Failed");
    let score:i32 = score.trim().parse().expect("Failed");

    if score > 100 || score < 0 {
        println!("Are you sure your score of {} is correct? You can only score between 0 and 100", score);
    } else if score >= 90 && score <= 100 {
        println!("Your score of {} is an A", score);
    } else if score >= 80 {
        println!("Your score of {} is a B", score);
    } else if score >= 70 {
        println!("Your score of {} is a C", score);
    } else if score >= 60 {
        println!("Your score of {} is a D", score);
    } else {
        println!("You failed with an F my g");
    }
}