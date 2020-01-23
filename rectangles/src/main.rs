fn main() {
    let width1 = 30;
    let height1 = 40;
    let rect1 = (30, 40);
    let rect2 = Rectangle { width: 30, height: 40 };

    println!("Area of rectangle is: {}", area(width1, height1));
    println!("Area of rectangle is: {}", area_tup(rect1));
    println!("Area of rectangle {:?} is: {}", rect2, area_s(&rect2)); // or {:#?}
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tup(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_s(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
