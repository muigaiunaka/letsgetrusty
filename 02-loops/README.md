Loop
Syntax:
    loop { /* statements */ }

Example:
```
fn main() {
    let mut n:i32 = 0;
    loop {
        if n < 5 {
            println!("Hello");
            n+=1;
        } else {
            break;
        }
    }
}
```

While
useful for a program to evaluate a condition within a loop
no explicit breaking
Syntax:
```
    while condition {
    // statements
    }
```

For