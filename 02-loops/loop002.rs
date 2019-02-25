fn main() {
    // print all even numbers between 1 to 100
    for num in 2..100 {
        if num % 2 == 0 {
            println!("Even number: {}", num)
        }
    }
}