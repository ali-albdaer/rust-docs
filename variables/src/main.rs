const ZERO: &str = "0";

fn main() {
    let x: i32 = 5;  // immutable variable can be shadowed by using let
    println!("The value of x is: {x}");

    let x = ZERO;  // shadowing (let needed to convert type)
    println!("The value of x is: {x}");

    {  // inner scope
        let x = 3;
        println!("The value of x is: {x}");
    }

    println!("The value of x is: {x}");  // back to ZERO
}
