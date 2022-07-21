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
  Blue
}

struct Box {
  dimensions: (i32, i32, i32), // width, height, depth
  weight: i32,
  color: Color
}

impl Box {
  fn create_box() -> Self {
    Self { dimensions: (2, 2, 2), weight: 10, color: Color::Red }
  }

  fn print_box(&self) {
    println!("Dimensions: {:?}", self.dimensions);
    println!("Weight: {:?}", self.weight);
    // println!("Color: {:?}", self.color);
  }
}

fn main() {
  let my_box = Box::create_box();
  my_box.print_box();
}
