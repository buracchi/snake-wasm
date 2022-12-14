use std::io;

use snake_wasm::*;
use snake_wasm::InputDirection::*;

// Comment '[lib] crate-type = ["cdylib"]' on Cargo.toml to compile this binary

fn get_input() -> Option<InputDirection> {
    let mut input = String::new();
    println!("Choose direction [w|a|s|d]: ");
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    println!();
    match input.chars().next() {
        Some('w') => Some(UP),
        Some('a') => Some(LEFT),
        Some('s') => Some(DOWN),
        Some('d') => Some(RIGHT),
        _ => None
    }
}

fn main() {
    const FRAME_DURATION: f64 = 1000_f64 / 60_f64;
    let mut game = SnakeGame::new(15, 5, 4, 0.1 / FRAME_DURATION, RIGHT);
    game.render();
    while !game.is_over {
        let direction = get_input();
        game.set_input_direction(direction);
        game.run_for(FRAME_DURATION);
        game.render();
        println!("{:?}", game);
    }
}
