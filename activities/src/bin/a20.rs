// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

use std::io;

enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl PowerState {
    fn new(state: &str) -> Option<PowerState> {
        match state.trim().to_lowercase().as_str() {
            "off" => Some(PowerState::Off),
            "sleep" => Some(PowerState::Sleep),
            "reboot" => Some(PowerState::Reboot),
            "shutdown" => Some(PowerState::Shutdown),
            "hibernate" => Some(PowerState::Hibernate),
            _ => None,
        }
    }
}

fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}

fn print_msg(state: PowerState) {
    match state {
        PowerState::Off => println!("turning off"),
        PowerState::Sleep => println!("going to sleep"),
        PowerState::Reboot => println!("rebooting system"),
        PowerState::Shutdown => println!("shutting down"),
        PowerState::Hibernate => println!("Going into hybernation"),
    }
}

fn main() {
    let mut received_command = false;
    while !received_command {
        match get_input() {
            Ok(input) => match PowerState::new(&input) {
                Some(state) => {
                    received_command = true;
                    print_msg(state)
                }
                None => println!("Invalid command"),
            },
            Err(e) => println!("error: {:?}", e),
        }
    }
}
