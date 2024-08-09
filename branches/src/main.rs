fn main() {
    let number = 6;

    let d = if number % 4 == 0 {
        println!("number is divisible by 4");
        4
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
        3
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
        2
    } else {
        println!("number is not divisible by 4, 3, or 2");
        1 // can't be "e", incompatible types
    };

    println!("{}", d);

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    
    let x;
    
    if true {
        x=1;
    } else {
        x=2;
    }
    
    println!("{}", x); 
    
    /*if true {
        1
    } else {
        'e'
    }; // expected integer, found char*/
    // semi-colon ensures the above thing is a statement (an expression won't work as main's return, if not equal to ())
    ()
}