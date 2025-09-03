use std::io::{self, Write};

/// Displays the game menu and handles user input for game selection.
pub fn show_game_menu() {
    println!("=== GAME LAUNCHER ===");
    println!("Choose a game by pressing a key:");
    println!();
    println!("5 - 5PUZZLE    B - BLINKY     R - BRIX       C - CONNECT4");
    println!("G - GUESS      H - HIDDEN     I - INVADERS   K - KALEID");
    println!("M - MAZE       E - MERLIN     S - MISSILE    P - PONG");
    println!("O - PONG2      U - PUZZLE     Y - SYZYGY     A - TANK");
    println!("T - TETRIS     L - TICTAC     F - UFO        V - VERS");
    println!("X - VBRIX      W - WIPEOFF    Z - BLITZ");
    println!();
    println!("Press a key in the window or ESC to quit");
    print!("Waiting for input... ");
    io::stdout().flush().unwrap();
}

/// Checks the key typed by the user when selecting a game
pub fn check_key_input() -> Option<String> {
    use std::io::{self, Write};
    print!("\nEnter your choice: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        let key = input.trim().to_uppercase();
        match key.as_str() {
            "5" => Some("5PUZZLE".to_string()),
            "B" => Some("BLINKY".to_string()),
            "R" => Some("BRIX".to_string()),
            "C" => Some("CONNECT4".to_string()),
            "G" => Some("GUESS".to_string()),
            "H" => Some("HIDDEN".to_string()),
            "I" => Some("INVADERS".to_string()),
            "K" => Some("KALEID".to_string()),
            "M" => Some("MAZE".to_string()),
            "E" => Some("MERLIN".to_string()),
            "S" => Some("MISSILE".to_string()),
            "P" => Some("PONG".to_string()),
            "O" => Some("PONG2".to_string()),
            "U" => Some("PUZZLE".to_string()),
            "Y" => Some("SYZYGY".to_string()),
            "A" => Some("TANK".to_string()),
            "T" => Some("TETRIS".to_string()),
            "L" => Some("TICTAC".to_string()),
            "F" => Some("UFO".to_string()),
            "V" => Some("VERS".to_string()),
            "X" => Some("VBRIX".to_string()),
            "W" => Some("WIPEOFF".to_string()),
            "Z" => Some("BLITZ".to_string()),
            _ => None,
        }
    } else {
        None
    }
}