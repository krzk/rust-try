fn main() {
    let width1 = 30;
    let height1 = 40;
    let rect1 = (30, 40);
    let rect2 = Rectangle { width: 30, height: 40 };
    let rect3 = Rectangle { width: 20, height: 40 };
    let rect4 = Rectangle { width: 32, height: 42 };

    println!("Area of rectangle is: {}", area(width1, height1));
    println!("Area of rectangle is: {}", area_tup(rect1));
    println!("Area of rectangle {:?} is: {}", rect2, area_s(&rect2)); // or {:#?}
    println!("Rectangle.area of {:?} is: {}", rect2, rect2.area()); // or {:#?}
    println!("Rectangle {:?} can hold {:?}: {}", rect2, rect3, rect2.can_hold(&rect3));
    println!("Rectangle {:?} can hold {:?}: {}", rect2, rect4, rect2.can_hold(&rect4));

    let rs = Rectangle::square(30);
    println!("Rectangle.area of {:?} is: {}", rs, rs.area());
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

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn area_s(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
