#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    fn times_three(number: u32) -> u32 {
        number * 3
    }
}

pub fn main() {
    let square = Rectangle::square(20);
    let nine = Rectangle::times_three(3);

    println!("Associated Stuff! {:?}, {}", square, nine);
}