use std::any::Any;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::snake::domain::board::Board;
use crate::snake::domain::game_over_event::GameOverEvent;
use crate::snake::domain::snake::Snake;
use crate::snake::domain::square::Square;

#[derive(Debug)]
pub struct EmptySquare {}

impl EmptySquare {
    pub fn new() -> Self { Self {} }
}

impl Square for EmptySquare {
    fn as_any(&self) -> &dyn Any { self }

    fn on_collision(&self) -> Box<dyn Fn(&Rc<RefCell<Box<dyn Square>>>, Weak<Board>, &mut Snake) -> Result<(), GameOverEvent>> {
        Box::new(|square, _, snake| {
            snake.move_into(square);
            Ok(())
        })
    }

    fn render(&self) -> char {
        'O'
    }
}
