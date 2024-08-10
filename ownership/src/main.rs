fn main() {
    let a = Box::new([0; 1_000]);  // Owner of the heap memory at frame 1.
    let b = a;  // Transfer the ownership of the heap memory from a to b. Frame 2.

    println!("{}", b[0]);  // Access the heap memory through b. Frame 3.
    // println!("{}", a[0]);  // Error: value borrowed here after move.

    let first = String::from("Ferris"); //1
    let full = add_suffix(first); //4
    // println!("{first}"); Error: value borrowed here after move.
    println!("{full}");    
}

fn add_suffix(mut name: String) -> String {
    //2
    name.push_str(" Jr."); //3
    name
}

// Frame 1: Stack: [main] {first*} -> Heap [Ferris]
// Frame 2: Stack: [main] {first} [add_suffix] {name*} -> Heap [Ferris]
// Frame 3: Stack: [main] {first} [add_suffix] {name*} -> Heap [Ferris Jr.]
// Frame 4: Stack: [main] {full*} -> Heap [Ferris Jr.]
// var*: var is the owner of the heap memory.
