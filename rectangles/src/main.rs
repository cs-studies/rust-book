fn main() {
    demo_area1();
    demo_area2();
    demo_area3();
}

fn demo_area1() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area1(width1, height1)
    );
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn demo_area2() {
    let r = (30, 50);

    println!("The area of the rectangle is {} square pixels.", area2(r));
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn demo_area3() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);
    println!("rect1 is {:#?}", rect1);
    if rect1.is_width() {
        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
        );
    }

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

    let square = Rectangle::square(22);
    dbg!(&square);
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn is_width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
