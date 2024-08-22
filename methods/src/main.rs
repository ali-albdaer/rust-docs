#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn fits(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn scale_by(&mut self, scale: u32) {
        self.width *= scale;
        self.height *= scale;
    }

    fn square(side_length: u32) -> Self {
        Self {
            width: side_length,
            height: side_length,
        }
    } // Not a method!
}

fn main() {
    let rect1: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    let scale: u32 = 2;

    let rect2: Rectangle = Rectangle {
        width: dbg!(10 * scale), // Takes ownership but returns it.
        height: 40,
    };

    let one_in_two: bool = rect2.fits(&rect1);
    let two_in_one: bool = rect1.fits(&rect2);

    println!("One in two: {}\nTwo in one: {}", one_in_two, two_in_one);
    println!("Area of rect1: {}", rect1.area());

    println!("Rect2: {:?}", rect2); // :#? for pretty-print.
    dbg!(&rect2); // A reference is used because dbg! takes ownership.

    let square1: Rectangle = Rectangle::square(20);
    println!("Square area: {}", square1.area());

    // Methods are syntacic sugar for function calls:
    let area1 = rect1.area();
    let area2 = Rectangle::area(&rect1);
    assert_eq!(area1, area2);

    // rect1.scale_by(3);  // rect1 is immutable!

    let mut rect3 = Rectangle {
        width: 25,
        height: 35,
    };

    println!("Initial Dimensions: {:#?}", rect3);
    rect3.scale_by(3); // == Rectangle::scale_by(&mut rect3, 3)
    println!("After scaling: {:#?}", rect3);
}
