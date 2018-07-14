fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50
    };

    println!("The area of the rectangle is {}", area(&rect));

    println!("rect1 is {:?}", rect);
    println!("rect1 is {:#?}", rect);

    let rect2 = Rectangle {
        width: 10,
        height: 20
    };
    let rect3 = Rectangle {
        width: 40,
        height: 60
    };

    println!("The area of the rectangle is {}", rect.area());
    println!("The area of the rectangle is {}", Rectangle::area(&rect));
    println!();
    println!("rect can hold rect2: {}", rect.can_hold(&rect2));
    println!("rect can hold rect3: {}", rect.can_hold(&rect3));
    println!();

    let rect = (30, 50);

    println!("The area of the rectangle is {}", area2(rect));

    println!("rect1 is {:?}", rect);

    let sq = Rectangle::square(10);
    println!("sq is {:?}", sq);
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn area2(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}