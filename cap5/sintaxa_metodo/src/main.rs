fn main() {
    definindo_metodos();
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        self.width > other_rect.width && self.height > other_rect.height
    }

    fn square(size: u32) -> Self {
        Self {
            height: size,
            width: size,
        }
    }
}

fn definindo_metodos() {
    let rect_1 = Rectangle {
        height: 50,
        width: 30,
    };

    let rect_2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect_3 = Rectangle::square(40);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect_1.area()
    );

    println!("Can rect1 hold rect2? {}", rect_1.can_hold(&rect_2));
    println!("Can rect1 hold rect3? {}", rect_1.can_hold(&rect_3));
}
