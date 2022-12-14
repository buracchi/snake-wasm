use wasm_bindgen::prelude::*;

use crate::domain::board::Board;
use crate::domain::snake::Snake;
use crate::geometry::vector::Vector;

pub mod domain;
pub mod geometry;

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub enum InputDirection {
    UP,
    LEFT,
    DOWN,
    RIGHT,
}

impl InputDirection {
    const U_X: Vector = Vector { x: 1 as f64, y: 0 as f64 };
    const U_Y: Vector = Vector { x: 0 as f64, y: 1 as f64 };

    pub fn as_vector(&self) -> Vector {
        match self {
            InputDirection::UP => InputDirection::U_Y.opposite(),
            InputDirection::LEFT => InputDirection::U_X.opposite(),
            InputDirection::DOWN => InputDirection::U_Y,
            InputDirection::RIGHT => InputDirection::U_X,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct SnakeGame {
    pub is_over: bool,
    pub score: i32,
    pub board: Board,
    snake: Snake,
    input_direction: Option<Vector>,
}

#[wasm_bindgen]
impl SnakeGame {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize, snake_initial_length: u32, snake_initial_speed: f64, snake_initial_direction: InputDirection) -> Self {
        let mut board = Board::new(width, height);
        let snake = Snake::new(&board, snake_initial_length, snake_initial_speed, snake_initial_direction.as_vector());
        let is_over = board.spawn_food(&snake).is_err();
        Self { is_over, score: 0, board, snake, input_direction: None }
    }

    pub fn export_snake(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.snake).unwrap()
    }

    pub fn set_input_direction(&mut self, input_direction: Option<InputDirection>) {
        self.input_direction = input_direction.map(|i| i.as_vector());
    }

    pub fn run_for(&mut self, timespan: f64) {
        self.is_over = self.snake.move_for(timespan, self.input_direction, &mut self.board, &mut self.score).is_err();
        self.input_direction = None;
    }
}

// main module utils
impl SnakeGame {
    pub fn render(&self) {
        self.snake.render(&self.board);
    }
}

#[cfg(test)]
mod tests {
    use rand::seq::IteratorRandom;

    use crate::InputDirection::*;
    use crate::SnakeGame;

    #[test]
    fn it_works() {
        let mut game = SnakeGame::new(6, 15, 3, 0.006, RIGHT);
        while !game.is_over {
            let direction = [UP, LEFT, DOWN, RIGHT].into_iter().choose(&mut rand::thread_rng());
            let frame_duration = 1000_f64 / 60_f64;
            game.set_input_direction(direction);
            game.run_for(frame_duration);
            println!("{:?}", game);
        }
        assert_eq!(true, true)
    }
}
