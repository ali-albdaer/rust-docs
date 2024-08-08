const ZERO: &str = "0";

fn main() {
    let x: i32 = 5;  // immutable variable can be shadowed by using let
    println!("The value of x is: {x}");

    let x: &str = ZERO;  // shadowing (let needed to convert type)
    println!("The value of x is: {x}");

    {  // inner scope
        let x: i32 = 3;
        println!("The value of x is: {x}");
    }

    println!("The value of x is: {x}");  // back to ZERO

    let y: f64 = 5.0;

    println!("The value of y is: {y}");

    let y: f32 = 6.0;

    println!("The value of y is: {y}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (a, b, c) = tup;

    println!("The value of a is: {a}");
    println!("The value of b is: {b}");
    println!("The value of c is: {c}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The value of five_hundred is: {five_hundred}");
    println!("The value of six_point_four is: {six_point_four}");
    println!("The value of one is: {one}");

    let mut tup2: (i32, f64, i32) = (500, 6.4, 1);

    tup2.2 = 2;


}
