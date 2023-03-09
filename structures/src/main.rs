// todo area of rectangle
// // refactoring with tuples
// refactoring with structs

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

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
        width: 40,
        height: 35,
    };

    dbg!(&rect1);
    dbg!(&rect2);
    dbg!(&rect3);

    if rect1.can_hold(&rect3) {
        println!("rect1 can hold rect3");
    } else {
        println!("rect1 can not hold rect3");
    }
}
