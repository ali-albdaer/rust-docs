fn main() {
    let first = String::from("Ferris"); //1
    let full = add_suffix(first); //4
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
// Frame 4: Stack: [main] {first*} -> Heap [Ferris Jr.]
// var*: var is the owner of the heap memory.
