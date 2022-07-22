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

fn map_input(input: String) -> Result<PowerState, String> {
    match input.to_lowercase().as_str() {
        "off" => Ok(PowerState::Off),
        "sleep" => Ok(PowerState::Sleep),
        "reboot" => Ok(PowerState::Reboot),
        "shutdown" => Ok(PowerState::Shutdown),
        "hibernate" => Ok(PowerState::Hibernate),
        _ => Err("invalid command, please select one of 'Off', 'Sleep', 'Reboot', 'Shutdown', 'Hibernate'"
            .to_owned()),
    }
}

fn main() {
    let mut received_command = false;
    while !received_command {
        match get_input() {
            Ok(input) => match map_input(input) {
                Ok(state) => {
                    received_command = true;
                    print_msg(state)
                }
                Err(e) => println!("error: {:?}", e),
            },
            Err(e) => println!("error: {:?}", e),
        }
    }
}
