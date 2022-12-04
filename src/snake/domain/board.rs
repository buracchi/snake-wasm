use std::cell::RefCell;
use std::rc::Rc;

use rand::seq::IteratorRandom;

use crate::snake::domain::direction::Direction;
use crate::snake::domain::game_over_event::GameOverEvent;
use crate::snake::domain::game_over_event::GameOverEvent::NoEmptySquaresEvent;
use crate::snake::domain::square::empty_square::EmptySquare;
use crate::snake::domain::square::food_square::FoodSquare;
use crate::snake::domain::square::Square;

#[derive(Debug)]
pub struct Board {
    length: usize,
    width: usize,
    squares: Vec<Vec<Rc<RefCell<Box<dyn Square>>>>>,
}

impl Board {
    pub fn new(length: usize, width: usize) -> Self {
        Self {
            length,
            width,
            squares: (0..length)
                .map(|_| (0..width).map(|_| Rc::new(RefCell::new(Box::new(EmptySquare::new()) as Box<dyn Square>))).collect())
                .collect(),
        }
    }

    pub fn get_middle_square(&self) -> Rc<RefCell<Box<dyn Square>>> {
        self.squares[self.width / 2][self.length / 2].clone()
    }

    pub fn get_adjacent_square(&self, square: &Rc<RefCell<Box<dyn Square>>>, direction: &Direction) -> Option<Rc<RefCell<Box<dyn Square>>>> {
        let mut coord = None;
        for i in 0..self.width {
            match self.squares[i].iter().position(|s| Rc::ptr_eq(s, square)) {
                Some(j) => coord = Some((i, j)),
                None => (),
            }
        }
        match coord {
            Some((x, y)) => {
                match direction {
                    Direction::UP =>    if x > 0 && x - 1 < self.length  && y < self.width { Some(self.squares[x - 1][y].clone()) } else { None },
                    Direction::DOWN =>  if x + 1 < self.length  && y < self.width { Some(self.squares[x + 1][y].clone()) } else { None },
                    Direction::RIGHT => if x < self.length && y + 1 < self.width { Some(self.squares[x][y + 1].clone()) } else { None },
                    Direction::LEFT =>  if x < self.length && y > 0 && y - 1 < self.width  { Some(self.squares[x][y - 1].clone()) } else { None },
                }
            }
            None => None
        }
    }

    pub fn spawn_food(&self) -> Result<(), GameOverEvent> {
        match self.squares.iter()
            .flatten()
            .filter(|&s| s.as_ref().borrow().as_any().downcast_ref::<EmptySquare>().is_some())
            .choose(&mut rand::thread_rng()) {
            Some(s) => {
                *s.borrow_mut() = Box::new(FoodSquare::new());
                Ok(())
            }
            None => Err(NoEmptySquaresEvent)
        }
    }

    pub fn render(&self) {
        for v in self.squares.clone().iter() {
            let mut buff = String::new();
            for s in v {
                buff.push(s.as_ref().borrow().render());
                buff.push(' ');
            }
            println!("{}", buff);
        }
    }
}
