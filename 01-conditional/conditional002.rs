use std::io;

fn main() {
    // a program that takes in user input and determines whether or not their input is a string
    let mut letter = String::new();
    println!("Enter a letter");
    io::stdin().read_line(&mut letter).expect("Failed to read letter");
    let letter:String=letter.trim().parse().expect("Failed");
    if letter == "a" || letter == "e" || letter =="i" || letter =="o" || letter =="u" {
        println!("Nice, that is a vowel")
    } else if letter == "y" {
        println!("tricky, this one is sometimes a vowel. a fauxvowel if you will");
    } else {
        println!("That is certainly not a vowel");
    }
}