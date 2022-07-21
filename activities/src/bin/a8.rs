// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavour {
  Orange,
  Grape,
  Pinapple
}

struct Drink {
  flavour: Flavour,
  ounces: f64,
}

fn print_drink_flavour(drink: Drink) {
  match drink.flavour {
      Flavour::Orange => println!("Orange"),
      Flavour::Grape => println!("Grape"),
      Flavour::Pinapple => println!("Pinapple")
  }
  println!("ounces: {:?}", drink.ounces);
}
fn main() {
  let my_drink = Drink {
    flavour: Flavour::Orange,
    ounces: 2.33
  };
  print_drink_flavour(my_drink)
}
