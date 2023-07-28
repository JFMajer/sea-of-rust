fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square1 = Rectangle::square(5);
    println!("I've create a square with side length {}", square1.width);

    println!("rect1 is a square? {}", rect1.am_i_square());
    println!("square1 is a square? {}", square1.am_i_square());
    println!("square1 is a square? {}", Rectangle::am_i_square(&square1)); // different way of calling method
}

struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn can_hold(&self, other_rectangle: &Rectangle) -> bool {
        self.width > other_rectangle.width && self.height > other_rectangle.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }

    fn am_i_square(&self) -> bool {
        self.width == self.height
    }
}
