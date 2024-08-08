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
}
