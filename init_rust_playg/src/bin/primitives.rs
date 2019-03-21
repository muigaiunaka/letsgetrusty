// TO RUN:
// create bin folder in src folder then place file
// cargo run --bin primitives
fn main() {
    // integers
    let _x = 5;
    // variables are immutable by default
    // x = 6; would not work
    let mut _num = 6;
    _num = 9;
    println!("{}", _num);
    let _ut2: u32 = 16;
    // i8, u8, i16, u16, i32, u32, i64, u64
    // actual memory based on the device you are using
    // isize or usize

    // float types
    // f32, f64 (64 bit float)

    let _add = 1 + 17;
    let _sub = 30 - 20;
    let _div = 10 / 2;
    let _multi = 15 * 4;
    let _rem = 49 & 4;

    // bool: true/false
    let _c: char = 'z';

    // tuples: collections of data that don't need to be the same type
    let _tupl: (i32, f64, char) = (42, 6.05, 'm');
    // destructuring tuple
    let (_z, _y, _x) = _tupl;
    // underscores will ignore the other values
    // let (_, _, x) = tupl;
    // debug flag: :?
    println!("{:?}", _tupl);
    // access values in tuples by doing, tuple variable name followed by a dot and index
    // tupl.0 or tupl.1
    // we can nest tuples

    // array
    let _arr = [1,2,3,4,5,6];
    let _a1 = _arr[0];
    println!("{}", _arr.len());
    // array slice - exclusive
    let _arrsl = &_arr[2..4];

    // String - compound type, not literal type
    let _s = "String";
    let _ss = String::from("String");
    let _sss = "String".to_string();
    // string slice
    let _sslice = &_ss[1..4];
    let _con = String::from("Muigai");
    let _cat = String::from("Unaka");
    let _mui = _con + &_cat;

    println!("{}", _tupl.0);
}