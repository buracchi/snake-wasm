use rand::seq::IteratorRandom;
use wasm_bindgen::prelude::*;

use crate::domain::food::Food;
use crate::domain::game_over_event::GameOverEvent;
use crate::domain::game_over_event::GameOverEvent::NoEmptySquaresEvent;
use crate::domain::snake::Snake;
use crate::geometry::rectangle::Rectangle;
use crate::geometry::vector::Vector;

#[derive(Debug, Copy, Clone)]
#[wasm_bindgen]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub food: Option<Food>,
}

#[wasm_bindgen]
impl Board {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height, food: None }
    }
}

impl Board {
    pub fn spawn_food(&mut self, snake: &Snake) -> Result<(), GameOverEvent> {
        self.food = (0..self.width).map(|x| (0..self.height).map(|y| (x, y)).collect::<Vec<_>>())
            .flatten()
            .map(|(x, y)| Vector::new(x as f64 + 0.5, y as f64 + 0.5))
            .filter(|position| !snake.contains(position))
            .choose(&mut rand::thread_rng())
            .map(|free_position| Food::new(Vector::new(free_position.x, free_position.y)));
        match self.food {
            Some(_) => Ok(()),
            None => Err(NoEmptySquaresEvent)
        }
    }

    pub fn as_rectangle(&self) -> Rectangle {
        Rectangle::new(Vector::new(0 as f64, 0 as f64), self.width as f64, self.height as f64)
    }
}
