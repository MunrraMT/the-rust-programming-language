fn main() {
    rectangles();
    println!("----------");
    rectangles_with_tuple();
    println!("----------");
    rectangles_with_struct();
}

fn rectangles() {
    let width_1 = 30;
    let height_1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width_1, height_1)
    );

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }
}

fn rectangles_with_tuple() {
    let rect = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect)
    );

    fn area(dimension: &(u32, u32)) -> u32 {
        dimension.0 * dimension.1
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn rectangles_with_struct() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    dbg!(&rect);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect)
    );

    fn area(dimension: &Rectangle) -> u32 {
        dimension.width * dimension.height
    }
}
