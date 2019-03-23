#![allow(dead_code)]
// tells compiler to ignore dead code warnings

#[derive(Debug)]
enum Direction {
    Up(Point),
    Down(Point),
    Left(Point),
    Right(Point),
}
#[derive(Debug)]
enum Keys {
    UpKey(String),
    DownKey(String),
    LeftKey(String),
    RightKey(String),
}

impl Direction {
    fn match_direction(&self) -> Keys {
        match *self {
            Direction::Up(_) => Keys::UpKey(String::from("Pressed w")),
            Direction::Down(_) => Keys::DownKey(String::from("Pressed s")),
            Direction::Left(_) => Keys::LeftKey(String::from("Pressed a")),
            Direction::Right(_) => Keys::RightKey(String::from("Pressed d")),
        }
    }
}

impl Keys {
    // take in a reference to self, output a reference to String
    fn destruct(&self) -> &String {
        match *self {
            Keys::UpKey(ref s) => s,
            Keys::DownKey(ref s) => s,
            Keys::LeftKey(ref s) => s,
            Keys::RightKey(ref s) => s,
        }
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

// options in Rust not nill/null
// good for creating optional functional arguments and nullable pointers
enum Option<T> {
    Some(T),
    None
}

fn division(x: f64, y:64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}

fn main() {
    let _u = Direction::Up(Point {x: 0, y: 1});
    let _d = Direction::Down(Point {x: 0, y: -1});
    let _l = Direction::Left(Point {x: -1, y: 0});
    let _r = Direction::Right(Point {x: 1, y: 0});

    let _k = _u.match_direction();
    let _x = _k.destruct();
    
    println!("{:?}", _x);

    let _res = division(5.0, 7.0);
    let _rem = division(13.0, 0.0);
    match res {
        Some(_b) => println!("{}", _b),
        None => println!("cannot divide by 0"),
    }
}