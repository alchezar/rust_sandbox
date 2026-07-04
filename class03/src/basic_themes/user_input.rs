// IKinder

use std::io;

fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_owned())
}

pub fn run() {
    let mut all_input = Vec::new();
    let mut times_input = 0;

    while times_input < 2 {
        match get_input() {
            Err(e) => println!("error: {}", e),
            Ok(words) => {
                all_input.push(words);
                times_input += 1;
            }
        }
    }

    for input in all_input {
        println!("Original: {}, capitalized: {}", input, input.to_uppercase());
    }
}

// Activity.

#[derive(Eq, PartialEq, Hash)]
enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl PowerState {
    fn new(state: &str) -> Option<Self> {
        let state = state.trim().to_lowercase();
        match state.as_str() {
            "off" => Some(PowerState::Off),
            "sleep" => Some(PowerState::Sleep),
            "reboot" => Some(PowerState::Reboot),
            "shutdown" => Some(PowerState::Shutdown),
            "hibernate" => Some(PowerState::Hibernate),
            _ => None,
        }
    }
}

fn print_power_message(power_state: PowerState) {
    use PowerState::*;
    let msg = match power_state {
        Off => "Power off",
        Sleep => "Slipping",
        Reboot => "Rebooting",
        Shutdown => "Shutting down",
        Hibernate => "Hibernation",
    };
    println!("OK: {msg}");
}

pub fn activity() {
    let mut state_received = false;
    while !state_received {
        println!("Please enter power state:");
        let mut buffer = String::new();
        if io::stdin().read_line(&mut buffer).is_err() {
            println!("Reading input error.");
            continue;
        }

        match PowerState::new(&buffer) {
            Some(state) => {
                print_power_message(state);
                state_received = true;
            }
            None => println!("ER: Invalid power state!"),
        }
    }
}
