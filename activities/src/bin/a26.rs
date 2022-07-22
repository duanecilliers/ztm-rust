// Topic: External crates
//
// Requirements:
// * Display the current date and time
//
// Notes:
// * Use the `chrono` crate to work with time
// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
//   for examples on how to create custom time formats

use chrono::prelude::*;

fn main() {
    let date = Utc::now();
    let formatted_date = date.format("%Y-%m-%d %H:%M:%S").to_string();
    println!("{:?}", formatted_date);
}
