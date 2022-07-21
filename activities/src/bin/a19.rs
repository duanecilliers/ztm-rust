// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main() {
    let mut items = HashMap::new();
    items.insert("chairs", 5);
    items.insert("beds", 3);
    items.insert("tables", 2);
    items.insert("couches", 0);

    for (item, stock_count) in items.iter() {
        match stock_count {
            0 => println!("{:?} is out of stock", item),
            s => println!("{:?} has {:?} items in stock", item, s),
        }
    }
}
