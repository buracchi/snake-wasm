use std::rc::Rc;

use crate::snake::domain::board::Board;
use crate::snake::domain::direction::Direction;
use crate::snake::domain::snake::Snake;

#[derive(Debug)]
pub struct SnakeGame {
    pub is_over: bool,
    board: Rc<Board>,
    snake: Snake,
}

impl SnakeGame {
    pub fn new(length: usize, width: usize) -> Self {
        let board = Rc::new(Board::new(length, width));
        Self {
            board: board.clone(),
            snake: Snake::new(Rc::downgrade(&board)),
            is_over: true,
        }
    }

    pub fn new_game(&mut self) {
        match self.board.spawn_food() {
            Ok(()) => self.is_over = false,
            Err(_) => self.is_over = true
        }
    }

    pub fn on_update(&mut self, direction: Option<Direction>) {
        match self.snake.move_forward(direction) {
            Ok(_) => (),
            Err(_) => self.is_over = true
        }
    }

    pub fn render(&self) {
        self.board.render();
    }
}
