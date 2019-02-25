use std::io;

fn main() {
    // create a quiz app. ask a question from user and give
    // 3 opportunities to attempt the question. If correct, quick the application
    let mut isGuessing:bool = true;
    let mut answer = String.new();
    while isGuessing {
        println!("Who is the mother of dragons?");
        io::stdin().read_line(&mut answer).expect("Failed");

        let answer:String = answer.trim().to_string().expect("Failed");
//        if answer == "daenerys"
    }
}