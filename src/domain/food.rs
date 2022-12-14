use wasm_bindgen::prelude::*;

use crate::domain::board::Board;
use crate::domain::game_over_event::GameOverEvent;
use crate::domain::snake::Snake;
use crate::geometry::vector::Vector;

#[derive(Debug, Copy, Clone)]
#[wasm_bindgen]
pub struct Food {
    pub position: Vector,
}

#[wasm_bindgen]
impl Food {
    #[wasm_bindgen(constructor)]
    pub fn new(position: Vector) -> Self { Self { position } }
}

impl Food {
    pub(crate) fn on_collision(&self, snake: &mut Snake, board: &mut Board, score: &mut i32) -> Result<(), GameOverEvent> {
        *score += 1;
        snake.grow();
        board.spawn_food(snake)
    }
}
