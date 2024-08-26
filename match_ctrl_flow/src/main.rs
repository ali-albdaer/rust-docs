#[derive(Debug)]
enum Clover {
    ThreeLeaf,
    FourLeaf,
}

impl Clover {
    fn new(leaves: u8) -> Result<Self, String> {
        match leaves {
            3 => Ok(Clover::ThreeLeaf),
            4 => Ok(Clover::FourLeaf),
            _ => Err(format!("Invalid Leaf Count: {}", leaves)),
        }
    }
}

fn pick(clover: &Clover) {
    match clover {
        Clover::ThreeLeaf => println!("3 Leaves."),
        Clover::FourLeaf => println!("You are lucky!!"),
    };

    // if let alternative:
    /*
    if let Clover::ThreeLeaf = clover {
        println!("3 Leaves.");
    } else {
        println!("You are lucky!!");
    }
    */
    // Practically, should only be used for 1 match or None. (example above is bad.)
}

fn main() {
    let mut leaves: String = String::new();
    std::io::stdin().read_line(&mut leaves).unwrap();

    let clover: Result<Clover, String> = Clover::new(leaves.trim().parse().unwrap());
    match &clover {
        Ok(_) => println!("Clover!"),
        Err(msg) => println!("{msg}"),  // `ref msg => ...` instead of `match &clover {...}` also works
    }

    pick(&clover.unwrap());
}
