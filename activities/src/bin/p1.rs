// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.

use std::{collections::HashMap, io, vec};

#[derive(Debug, Clone, Copy)]
enum MenuOption {
    AddBills,
    ViewBills,
}

#[derive(Debug)]
struct Bill {
    name: String,
    amount_owed: i32,
}

impl Bill {
    fn new(name: String, amount_owed: i32) -> Self {
        Bill { name, amount_owed }
    }
}

fn get_options() -> Vec<MenuOption> {
    vec![MenuOption::AddBills, MenuOption::ViewBills]
}

fn display_menu() {
    let options = get_options();
    println!("Select an option below by typing a number and hitting enter");
    for (i, item) in options.iter().enumerate() {
        println!("{:?}: {:?}", i + 1, item);
    }
}

fn map_selection_to_menu_option(selection: &str) -> Option<MenuOption> {
    let options = get_options();
    match selection.trim() {
        "1" => Some(options[0]),
        "2" => Some(options[1]),
        _ => None,
    }
}

fn display_sub_menu(option: MenuOption, bills: &mut Vec<Bill>) {
    use MenuOption::*;
    match option {
        AddBills => add_bills(bills),
        ViewBills => view_bills(),
    }
}

// fn handle_menu_selection() {}

fn add_bills(bills: &mut Vec<Bill>) {
    let mut name_buffer = String::new();
    let mut name: Option<&str> = None;
    let mut amount_buffer = String::new();
    let mut amount_owed: Option<i32> = None;
    loop {
        if name.is_none() {
            println!("Bill name: ");
            let user_input = io::stdin().read_line(&mut name_buffer);
            if user_input.is_ok() {
                name = Some(name_buffer.trim());
                println!("name: {:?}", name);
            } else {
                break println!("error reading input");
            }
        } else if amount_owed.is_none() {
            println!("Bill amount_owed: ");
            let user_input = io::stdin().read_line(&mut amount_buffer);
            if user_input.is_ok() {
                println!("amount string: {:?}", amount_buffer);
                amount_owed = Some(amount_buffer.trim().to_owned().parse::<i32>().unwrap());
                println!("amount_owed: {:?}", amount_owed);
            } else {
                break println!("error reading input");
            }
        } else {
            let bill = Bill::new(name.unwrap().to_owned(), amount_owed.unwrap());
            println!("Creating bill: {:?}", bill);
            break bills.push(bill);
        }
    }
}

fn view_bills() {
    println!("Bill amount: ");
}

fn main() {
    let mut buffer = String::new();
    let mut bills: Vec<Bill> = vec![];

    let mut selected_menu_item: Option<MenuOption> = None;

    loop {
        if selected_menu_item.is_none() {
            display_menu();
            let user_input = io::stdin().read_line(&mut buffer);
            if user_input.is_ok() {
                match map_selection_to_menu_option(&mut buffer) {
                    Some(option) => selected_menu_item = Some(option),
                    None => println!("invalid selection"),
                };
            } else {
                break println!("error reading input");
            }
        } else {
            break display_sub_menu(selected_menu_item.unwrap(), &mut bills);
        }
    }
}
