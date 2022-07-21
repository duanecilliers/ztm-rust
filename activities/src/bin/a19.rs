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
    items.insert("Chair", 5);
    items.insert("Bed", 3);
    items.insert("Table", 2);
    items.insert("Couch", 0);

    for (item, qty) in items.iter() {
        let stock_count = match qty {
            0 => format!("{:?} is out of stock", item),
            s => format!("{:?} has {:?} items in stock", item, s),
        };
        println!("item={:?}", stock_count);
    }
}
