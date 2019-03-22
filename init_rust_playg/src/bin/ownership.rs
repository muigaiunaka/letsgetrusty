fn cop(a: i32, b: i32) {
    println!("Copy sum: {}", a + b);
}

fn main() {
    // ownership
    // each variable has a value, the variable itself is an owner
    let x = 1; // x owns 1
    let s = String::from("String");
    // can't do: let y = s; because only one reference can own something at a time

    let a = 32;
    let b = 45;
    cop(a, b);
    println!("the result of adding these two {} {}", a, b)
    // borrowing: lets us have mutiple references to resource while still adhering to the single owner, single place of responsibility rule
    
     
}

// Stack + Heap
// parts of memory available for your code to use at run time
// Stack: stores values in order to get them and then removes the values in the opposite order
// All literals (integer, slice, etc) are stored on Stack
// Heap: a pile of memory, pointers that point to various memory locations, a little slower than stack
// complicated data is held on the heap