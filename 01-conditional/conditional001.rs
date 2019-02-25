use std::io;

fn main() {
    let mut name = String::new();
    let mut age = String::new();
    let mut ch = String::new();

    println!("Enter name and age");
    io::stdin().read_line(&mut name).expect("Failed, improper name");
    io::stdin().read_line(&mut age).expect("Failed, improper name");
    let age:i32=age.trim().parse().expect("Failed");

    println!("Do you want to create account?");
    io::stdin().read_line(&mut ch).expect("Failed");
    ch=ch.trim().to_string();
    if ch=="y" {
        if age < 10 {
            println!("your age is too low")
        } else {
            println!("Your new account is created");
            println!("Do you want to upload a photo");
            ch.clear();
            io::stdin().read_line(&mut ch).expect("Failed");
            ch=ch.trim().to_string();

            if ch == "y" {
                if age < 13 {
                    println!("You are too young to upload a photo")
                } else {
                    println!("You can upload a photo now :)")
                }
            } else {
                println!("Thanks for trying anyway bud")
            }


        }
    } else if ch == "n"{
        println!("Sad to see you don't want to create an account at this time :(")
    } else {
        println!("Failed to understand your input. Terminating.")
    }
}