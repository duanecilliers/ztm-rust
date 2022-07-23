// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing

#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

#[derive(Debug)]
struct Shoe(Color);
impl Shoe {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

#[derive(Debug)]
struct Shirt(Color);
impl Shirt {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

#[derive(Debug)]
struct Pants(Color);
impl Pants {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

fn print_shoe(shoe: Shoe) {
    println!("Shoe: {:?}", shoe);
}

fn print_shirt(shirt: Shirt) {
    println!("Shirt: {:?}", shirt);
}

fn print_pants(pants: Pants) {
    println!("Pants: {:?}", pants);
}

fn main() {
    let red_shoe = Shoe::new(Color::Red);
    let blue_shoe = Shoe::new(Color::Blue);
    let green_shirt = Shirt::new(Color::Green);
    let purple_pants = Pants::new(Color::Purple);
    print_shoe(red_shoe);
    print_shoe(blue_shoe);
    print_shirt(green_shirt);
    print_pants(purple_pants);
}
