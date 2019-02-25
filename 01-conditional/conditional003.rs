use std::io;

fn main() {
    // a calculator with basic arithmmetic operations
    let mut op = String::new();
    let mut f_digit = String::new();
    let mut s_digit = String::new();
    println!("Enter first digit");
    io::stdin().read_line(&mut f_digit).expect("Failed first digit");
    let f_digit:i32 = f_digit.trim().parse().expect("Failed to change type");

    println!("Choose operation: \n1. +\n2. -\n3. /\n4. *");
    io::stdin().read_line(&mut op).expect("Failed operator");
    let op:char = op.trim().parse().expect("Failed to put in operator");

    println!("Enter second digit");
    io::stdin().read_line(&mut s_digit).expect("Failed first digit");
    let s_digit:i32 = s_digit.trim().parse().expect("Failed to change type");

    if op=='+' {
        println!("Your sum: {}", f_digit + s_digit);
    } else if op == '-' {
        println!("Your difference: {}", f_digit - s_digit);
    } else if op == '*' {
        println!("Your product: {}", f_digit * s_digit);
    } else if op == '/' {
        println!("Your remainder: {}", f_digit / s_digit);
    } else {
        println!("no operator actually input my g");
    }


}