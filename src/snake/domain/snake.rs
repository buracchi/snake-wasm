use std::cell::RefCell;
use std::collections::LinkedList;
use std::rc::{Rc, Weak};

use crate::snake::domain::board::Board;
use crate::snake::domain::direction::Direction;
use crate::snake::domain::game_over_event::GameOverEvent;
use crate::snake::domain::game_over_event::GameOverEvent::BoundCollisionEvent;
use crate::snake::domain::square::Square;
use crate::snake::domain::square::Square::{EmptySquare, SnakeSquare};

#[derive(Debug)]
pub struct Snake {
    board: Weak<Board>,
    body: LinkedList<Rc<RefCell<Square>>>,
    direction: Direction,
}

impl Snake {
    pub fn new(board: Weak<Board>) -> Self {
        let starting_square = match board.upgrade() {
            Some(b) => b.get_middle_square(),
            None => panic!("Fatal error: invalid board")
        };
        *starting_square.as_ref().borrow_mut() = SnakeSquare;
        Self {
            board,
            body: LinkedList::from([starting_square]),
            direction: Direction::RIGHT,
        }
    }

    pub fn move_forward(&mut self, new_direction: Option<Direction>) -> Result<(), GameOverEvent> {
        let head_square = match self.body.front() {
            Some(square) => square,
            None => panic!("Fatal error: snake body was empty")
        };
        match new_direction {
            Some(d) => if d != self.direction.opposite() { self.direction = d },
            None => ()
        }
        let next_square = match self.board.upgrade() {
            Some(b) => b.get_adjacent_square(head_square, &self.direction),
            None => panic!("Fatal error: invalid board")
        };
        match next_square {
            Some(s) => {
                let collision_strategy = s.as_ref().borrow().on_collision();
                collision_strategy(&s, self.board.clone(), self)
            }
            None => Err(BoundCollisionEvent),
        }
    }

    pub fn move_into(&mut self, next_square: &Rc<RefCell<Square>>) {
        *next_square.as_ref().borrow_mut() = SnakeSquare;
        self.body.push_front(next_square.clone());
        match self.body.pop_back() {
            Some(tail_square) => *tail_square.as_ref().borrow_mut() = EmptySquare,
            None => panic!("Fatal error: snake body was empty")
        }
    }

    pub fn grow_into(&mut self, next_square: &Rc<RefCell<Square>>) {
        *next_square.as_ref().borrow_mut() = SnakeSquare;
        self.body.push_front(next_square.clone());
    }
}
