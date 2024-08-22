struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn fits(&self, other: &Rectangle) -> bool {
        if self.width >= other.width && self.height >= other.height {
            true
        } else {
            false
        }
    }
}

fn main () {
    let rect1: Rectangle = Rectangle {
        width: 30,
        height: 50
    };

    let rect2: Rectangle = Rectangle {
        width: 20,
        height: 40
    };

    let one_in_two: bool = rect2.fits(&rect1);
    let two_in_one: bool = rect1.fits(&rect2);

    print!("One in two: {}\nTwo in one: {}", one_in_two, two_in_one);
}