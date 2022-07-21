// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
  age: i32,
  name: String,
  color: String
}

fn print(data: &str) {
  println!("{:?}", data);
}


fn main() {
  let store = vec![
    Person { age: 5, name: "Joe".to_owned(), color: "Blue".to_owned() },
    Person { age: 10, name: "Jane".to_owned(), color: "Red".to_owned() },
    Person { age: 32, name: "Duane".to_owned(), color: "Purple".to_owned() },
  ];

  for item in store {
    if item.age <= 10 {
      print(&item.name);
      print(&item.color);
    }
  }
}
