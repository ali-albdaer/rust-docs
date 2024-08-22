#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn fits(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn main () {
    let rect1: Rectangle = Rectangle {
        width: 30,
        height: 50
    };

    let scale: u32 = 2;

    let rect2: Rectangle = Rectangle {
        width: dbg!(10 * scale),  // Takes ownership but returns it.
        height: 40
    };

    let one_in_two: bool = rect2.fits(&rect1);
    let two_in_one: bool = rect1.fits(&rect2);

    println!("One in two: {}\nTwo in one: {}", one_in_two, two_in_one);
    println!("Area of rect1: {}", rect1.area());

    println!("Rect2: {:?}", rect2);  // :#? for pretty-print.
    dbg!(&rect2);  // A reference is used because dbg! takes ownership. 
}
