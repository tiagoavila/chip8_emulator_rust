use std::time::{Duration, Instant};

use clap::Parser;
use minifb::Key;


mod chip8;
mod chip8_util;
mod constants;
mod game_menu;
mod user_input;
mod screen;

/// Command-line arguments for the Chip-8 Emulator
#[derive(Parser, Debug)]
#[command(version = "1.0", about = "A Chip-8 Emulator written in Rust")]
struct Args {
    /// Run the emulator in debug mode
    #[arg(long)]
    debug: bool,

    /// Number of instructions to execute before manual stepping in debug mode
    #[arg(long = "instruction_count", default_value_t = 20)]
    instruction_count: usize,

    #[arg(long = "rom", default_value_t = String::from("PONG"))]
    rom: String,
}

/// Example usage:
/// Normal mode: `cargo run`
/// Debug mode: `cargo run -- --debug --instruction_count 50`

fn main() {
    let args = Args::parse();
    let debug_mode = args.debug;
    let instruction_count = args.instruction_count;

    if debug_mode {
        run_debug_mode(instruction_count, args.rom);
    } else {
        game_menu::show_game_menu();

        let mut selected_rom: Option<String> = None;

        while selected_rom.is_none() {
            // Check for keyboard input using stdin
            if let Some(game) = game_menu::check_key_input() {
                selected_rom = Some(game.clone());
                println!("Selected game: {}", game);
                break;
            }
        }

        if let Some(game) = selected_rom {
            println!("Loading {}...", game);
            run_normal_mode(game);
            // Here you would load and run the selected game
        } else {
            println!("No game selected. Exiting.");
        }
    }
}

fn run_normal_mode(rom_file: String) {
    let binary =
        chip8_util::Chip8Util::read_rom(format!("files/roms/{}", rom_file).as_str()).unwrap();
    let mut chip8 = chip8::Chip8::start(binary);

    let mut is_running = true;
    let mut last_timer_update = Instant::now();
    let frame_duration = Duration::from_millis(1000 / 60); // 60 FPS

    let mut buffer = screen::initialize_buffer();
    let mut window = screen::initialize_window();

    // Normal mode loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        chip8.reset_keyboard();
        chip8.needs_redraw = false;

        // Check pressed keys
        if let Some(key) = user_input::get_pressed_key(&window) {
            chip8.keyboard[key as usize] = true;
        }

        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::Yes) {
            is_running = !is_running;
            if is_running {
                println!("Resuming execution");
            } else {
                println!("Pausing execution");
            }
        }

        if is_running {
            // What is a "frame" in this context?
            // A frame is essentially one iteration of your main loop where you update the game state and redraw the screen.
            // Think of it as:
            // Poll input (keyboard state)
            // Run some number of CHIP-8 CPU cycles
            // Update timers (delay + sound, 60Hz)
            // Redraw the display
            // Since the CHIP-8 was designed to run on very simple hardware, its timers tick at 60 Hz (once every 1/60th of a second),
            // independent of how many CPU cycles are run in between.
            // Run multiple CPU cycles per frame

            // Decouple CPU cycles from timers and frames. Typically:
            // Decide on a target FPS → usually 60 Hz, to match the CHIP-8 timers.
            // Run multiple CPU cycles per frame → because CHIP-8 executes more than 60 instructions per second (usually ~500–1000).
            // Decrement timers exactly once per frame.
            // Redraw the screen at 60 Hz.
            for _ in 0..10 {
                // Adjust: 8–12 is common
                chip8.tick();
            }

            // 3. Update timers at ~60Hz
            if last_timer_update.elapsed() >= frame_duration {
                chip8.update_timers();

                last_timer_update = Instant::now();
            }

            chip8.tick();
        }

        screen::draw_screen_if_needed(&mut buffer, &chip8);

        screen::update_window_with_buffer(&mut buffer, &mut window);
    }
}

fn run_debug_mode(instruction_count: usize, rom_file: String) {
    // let binary = read_rom("files/roms/IBM_Logo.ch8").unwrap();
    // let binary = read_rom("files/roms/chip8-logo.ch8").unwrap();
    // let binary = read_rom("files/roms/3-corax+.ch8").unwrap();
    // let binary = read_rom("files/roms/5-quirks.ch8").unwrap();
    // let binary = read_rom("files/roms/4-flags.ch8").unwrap();
    let binary =
        chip8_util::Chip8Util::read_rom(format!("files/roms/{}", rom_file).as_str()).unwrap();

    let mut chip8 = chip8::Chip8::start(binary);

    // Debug mode loop
    let mut space_pressed = false;

    println!("CHIP-8 Debug Mode");
    println!("Controls:");
    println!("  SPACE - Execute one instruction");
    println!("  ESC   - Quit");
    println!();

    // Execute initial instructions if instruction_count > 0
    for _ in 0..instruction_count {
        chip8.tick();
    }

    chip8.enable_debug_mode(instruction_count);

    let mut buffer = screen::initialize_buffer();
    let mut window = screen::initialize_window();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let space_down = window.is_key_down(Key::Space);
        if space_down && !space_pressed {
            chip8.tick();
        }
        space_pressed = space_down;

        screen::draw_screen_if_needed(&mut buffer, &chip8);

        screen::update_window_with_buffer(&mut buffer, &mut window);
    }
}

