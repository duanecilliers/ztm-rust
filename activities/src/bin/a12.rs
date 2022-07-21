// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Red => println!("red"),
            Color::Green => println!("green"),
            Color::Blue => println!("blue"),
        }
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!(
            "Dimensions: {:?}, {:?}, {:?}",
            self.width, self.height, self.depth
        );
    }
}

struct Box {
    dimensions: Dimensions,
    weight: f64,
    color: Color,
}

impl Box {
    fn create(dimensions: Dimensions, weight: f64, color: Color) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }

    fn print(&self) {
        self.dimensions.print();
        self.color.print();
        println!("Weight: {:?}", self.weight);
    }
}

fn main() {
    let my_box = Box::create(
        Dimensions {
            width: 2.0,
            height: 2.0,
            depth: 2.0,
        },
        20.0,
        Color::Blue,
    );
    my_box.print();
}
