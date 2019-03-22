fn main() {
    // conditionals
        // conditional operators:
        // == != < > <= >=
        let _n = 16;
        if _n > 21 {
            println!("you can drink mate");
        };
        // can bind conditional to variable; can't use with multiple different types
        let _underage = if _n < 21 {
            "you can't drink mate"
        } else { 
            "you can drink mate"
        };
        println!("{}", _underage);
    // loops
    let mut _i = 0;
    let mut _w = 10;
    let a_vec = vec![10, 20, 30, 40, 50];
    loop {
        println!("{}", _i);
        if _i >= 100 {
            break
        }
        _i += 10;
    }
    while _w > 0 {
        if _w % 5 == 0 {
            println!("{}", _w)
        }
        _w = _w - 1;
    }
    for i in a_vec {
        println!("i: {}", i);
    }
        // iterate without a collection. exclusive is .., inclusive is ...
        for b in 1..101 {
            if b > 40 && b < 50 {
                println!("{}", b);
            }
        }
    // pattern matchings
    // match statement is kind of like switch/case but more powerful; full pattern matching
    // basic example:
    let _ma = 3;
    match _ma {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("default"),
    }
}