fn main() {
    let mut n:i32 = 0;
    let mut m:i32 = 1;
    loop {
        if n < 5 {
            println!("Hello");
            n+=1;
        } else {
            break;
        }
    }
    while m <= 9 {
        println!("Hola 9x");
        m+=1;
    }
    for x in 1 .. 10 {
        println!("for loop is {} times more performant than while loop : {}", x, x)
    }
}