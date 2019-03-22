use std::fmt;
// standard library called fmt is where the Display trait lives

#[derive(Debug)] 
struct Object {
    // properties
    width: u32,
    height: u32,
}
// impl keyword followed by struct name
impl Object {
    // METHOD
    fn area(&self) -> u32 {
        let area : u32 = self.width * self.height;
        // rust automatically returns the last statement in a function
        return area;
    }
    // RELATED FUNCTION
    fn create_new(width:u32, height:u32) -> Object {
        Object {
            width,
            height,
        }
    }
    // METHOD
    fn reveal(&self) {
        println!("{}x{} with area: {}", self.width, self.height, self.area());
    }
}

impl fmt::Display for Object {
    // formats the way we display Object struct
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.width, self.height)
    }
}



// ampersand is a reference, allows us to keep struct in scope
// fn area(obj: &Object) -> u32 {
//     let area : u32 = obj.width * obj.height;
//     // rust automatically returns the last statement in a function
//     return area;
// }
// to run: area(&o);

fn main() {
    let o = Object {
        width: 16,
        height: 54,
    };
    let obj = Object::create_new(58, 82);
    
    o.reveal();
    obj.reveal();
    println!("{:#?}", o);
    println!("{:#?}", obj);
}