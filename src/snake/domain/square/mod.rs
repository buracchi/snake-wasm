use std::any::Any;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::snake::domain::board::Board;
use crate::snake::domain::game_over_event::GameOverEvent;
use crate::snake::domain::snake::Snake;

pub mod empty_square;
pub mod food_square;
pub mod snake_square;

pub trait Square : core::fmt::Debug {
    fn as_any(&self) -> &dyn Any;
    fn on_collision(&self) -> Box<dyn Fn(&Rc<RefCell<Box<dyn Square>>>, Weak<Board>, &mut Snake) -> Result<(), GameOverEvent>>;
    fn render(&self) -> char;
}
