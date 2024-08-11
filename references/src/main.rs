fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");  // 1
    greet(&m1, &m2); // lends the strings to greet // 3
    let s = format!("{} {}", m1, m2);
    // println!("{}", s);

    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x;         // *x reads the heap value, so a = 1
    *x += 1;                 // *x on the left-side modifies the heap value,
                             //     so x points to the value 2
    
    let r1: &Box<i32> = &x;  // r1 points to x on the stack
    let b: i32 = **r1;       // two dereferences get us to the heap value
    
    let r2: &i32 = &*x;      // r2 points to the heap value directly
    let c: i32 = *r2;

    println!("{} {} {}", a, b, c);
}

fn greet(g1: &String, g2: &String) { // burrows the strings
    // 2
    //println!("{} {}!", g1, g2);
}


// Frame 1: Stack: [main] {m1*, m2*} -> Heap [Hello, world]
// Frame 2: Stack: [main] {m1*, m2*} [greet] {g1+, g2+} -> Heap [Hello, world]
// Frame 3: Stack: [main] {m1*, m2*} -> Heap [Hello, world]
// var*: var is the owner of the heap memory.
// var+: var is borrowing the heap memory.