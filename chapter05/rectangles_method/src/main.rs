struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

	let rect2 = Rectangle::new(40, 60);
	println!(
		"The area of second rectangle is {} square pixels.",
		rect2.area()
	);
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

	fn new(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width,
            height,
        }
    }
}
