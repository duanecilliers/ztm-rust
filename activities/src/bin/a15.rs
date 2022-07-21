// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Tickets {
  Backstage(i32, String),
  VIP(i32, String),
  Standard(i32)
}

fn main() {
  let tickets = vec![
    Tickets::Backstage(200, "Duane".to_owned()),
    Tickets::VIP(150, "Pete".to_owned()),
    Tickets::Standard(100),
  ];

  for ticket in tickets {
    match ticket {
        Tickets::Backstage(price, name) => println!("Price: {:?}, Name: {:?}", price, name),
        Tickets::VIP(price, name) => println!("Price: {:?}, Name: {:?}", price, name),
        Tickets::Standard(price) => println!("Price: {:?}", price),
    }
  }
}
