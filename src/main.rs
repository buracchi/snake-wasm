use std::io;

use crate::snake::domain::direction::Direction;
use crate::snake::domain::direction::Direction::{DOWN, LEFT, RIGHT, UP};
use crate::snake::snake_game::SnakeGame;

mod snake;

fn get_input() -> Option<Direction> {
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
    let mut game = SnakeGame::new(5, 5);
    game.new_game();
    game.render();
    while !game.is_over {
        let direction = get_input();
        game.on_update(direction);
        game.render();
    }
}
