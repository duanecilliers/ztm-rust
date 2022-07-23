// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

trait Shape {
    fn perimeter(&self) {}
}

struct Square {
    side: i32,
}
impl Shape for Square {
    fn perimeter(&self) {
        println!("The square's perimeter is {:?}", &self.side * 4);
    }
}

struct Triangle {
    a: i32,
    b: i32,
    c: i32,
}
impl Shape for Triangle {
    fn perimeter(&self) {
        println!(
            "The triangle's perimeter is {:?}",
            &self.a + &self.b + &self.c
        );
    }
}

fn print_perimeter(shape: impl Shape) {
    shape.perimeter();
}

fn main() {
    print_perimeter(Square { side: 2 });
    print_perimeter(Triangle { a: 2, b: 2, c: 2 });
}
