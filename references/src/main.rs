fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");  // 1
    greet(&m1, &m2); // lends the strings to greet // 3
    let s = format!("{} {}", m1, m2);
    println!("{}", s);
}

fn greet(g1: &String, g2: &String) { // burrows the strings
    // 2
    println!("{} {}!", g1, g2);
}


// Frame 1: Stack: [main] {m1*, m2*} -> Heap [Hello, world]
// Frame 2: Stack: [main] {m1*, m2*} [greet] {g1+, g2+} -> Heap [Hello, world]
// Frame 3: Stack: [main] {m1*, m2*} -> Heap [Hello, world]
// var*: var is the owner of the heap memory.
// var+: var is borrowing the heap memory.