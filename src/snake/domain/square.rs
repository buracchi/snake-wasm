use std::cell::RefCell;
use std::rc::{Rc, Weak};
use crate::snake::domain::board::Board;
use crate::snake::domain::game_over_event::GameOverEvent;
use crate::snake::domain::game_over_event::GameOverEvent::SnakeCollisionEvent;
use crate::snake::domain::snake::Snake;
use crate::snake::domain::square::Square::{EmptySquare, FoodSquare, SnakeSquare};

#[derive(Debug, PartialEq)]
pub enum Square {
    EmptySquare,
    FoodSquare,
    SnakeSquare,
}

impl Square {
    pub fn on_collision(&self) -> Box<dyn Fn(&Rc<RefCell<Square>>, Weak<Board>, &mut Snake) -> Result<(), GameOverEvent>> {
        match self {
            EmptySquare => Box::new(|square, _, snake| {
                snake.move_into(square);
                Ok(())
            }),
            FoodSquare => Box::new(|square, board, snake| {
                snake.grow_into(square);
                match board.upgrade() {
                    Some(b) => b.spawn_food(),
                    None => panic!("Invalid board!")
                }
            }),
            SnakeSquare => Box::new(|_, _, _| Err(SnakeCollisionEvent))
        }
    }
    pub fn render(&self) -> char {
        match self {
            EmptySquare => 'O',
            FoodSquare => 'X',
            SnakeSquare => 'S'
        }
    }
}
