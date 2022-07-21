// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct GroceryItem {
  id: i32,
  quantity: i32
}

fn display_quantity(item: &GroceryItem) {
  println!("Quantity: {:?}", item.quantity);
}

fn display_id(item: &GroceryItem) {
  println!("ID: {:?}", item.id);
}

fn main() {
  let my_grocery = GroceryItem {
    id: 1,
    quantity: 3
  };

  display_quantity(&my_grocery);
  display_id(&my_grocery);
}
