use std::collections::{BTreeMap, HashSet};

use wasm_bindgen::prelude::*;

use crate::domain::board::Board;
use crate::domain::game_over_event::GameOverEvent;
use crate::domain::game_over_event::GameOverEvent::{BorderCollisionEvent, SnakeCollisionEvent};
use crate::geometry::line_segment::LineSegment;
use crate::geometry::vector::Vector;

#[wasm_bindgen]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Snake {
    body: Vec<Vector>,
    speed: f64,
    direction: Vector,
    next_direction: Option<Vector>,
}

#[wasm_bindgen]
impl Snake {
    #[wasm_bindgen(constructor)]
    pub fn new(board: &Board, length: u32, speed: f64, direction: Vector) -> Self {
        let head = Vector::new(
            (board.width as f64 / 2_f64).round() - 0.5,
            (board.height as f64 / 2_f64).round() - 0.5,
        );
        let tail = head - direction * length;
        Self { body: Vec::from([head, tail]), speed, direction, next_direction: None }
    }
}

impl Snake {

    pub fn contains(&self, point: &Vector) -> bool {
        match LineSegment::segments_from_positions(&self.body).iter()
            .map(|s| s.contains(point))
            .reduce(|a, b| (a | b)) {
            Some(result) => result,
            None => false
        }
    }

    pub fn grow(&mut self) {
        let old_tail = self.body.pop().unwrap();
        let tail_segment = LineSegment::new(*self.body.last().unwrap(), old_tail);
        let new_tail = old_tail + tail_segment.direction();
        self.body.push(new_tail);
    }

    pub fn move_for(&mut self, mut timespan: f64, new_direction: Option<Vector>, board: &mut Board, score: &mut i32) -> Result<(), GameOverEvent> {
        if new_direction.is_some() {
            self.next_direction = new_direction
        }
        if self.next_direction.is_some() {
            self.change_direction(&mut timespan, board, score)?
        }
        let mut distance = self.speed * timespan;
        loop {
            match self.nearest_collision_point(distance, board) {
                Some(collision_point) => {
                    let collision_distance = (collision_point - self.body[0]).length();
                    self.move_of(collision_distance);
                    distance -= collision_distance;
                    self.check_collisions(board, score)?
                }
                None => {
                    self.move_of(distance);
                    return Ok(());
                }
            }
        }
    }

    fn move_of(&mut self, distance: f64) {
        if distance > 0 as f64 {
            let old_head_segment_direction = LineSegment::new(self.body[1], self.body[0]).direction();
            let old_head = &mut self.body[0];
            let new_head = &(*old_head + (self.direction * distance));
            if old_head_segment_direction == self.direction || distance < f64::EPSILON {
                *old_head = *new_head;
            } else {
                self.body.insert(0, *new_head);
            }
            let mut tail_length_to_remove = distance;
            while tail_length_to_remove > 0 as f64 {
                let old_tail = self.body.pop().unwrap();
                let (tail_segment_length, tail_segment_direction) = {
                    let tail_segment = LineSegment::new(old_tail, *self.body.last().unwrap());
                    (tail_segment.length(), tail_segment.direction())
                };
                if tail_segment_length > tail_length_to_remove {
                    let new_tail = old_tail + (tail_segment_direction * tail_length_to_remove);
                    self.body.push(new_tail);
                }
                tail_length_to_remove = tail_length_to_remove - tail_segment_length;
            }
        }
    }

    fn change_direction(&mut self, timespan: &mut f64, board: &mut Board, score: &mut i32) -> Result<(), GameOverEvent> {
        let next_direction = self.next_direction.unwrap();
        if self.direction == next_direction || self.direction.opposite() == next_direction {
            return Ok(());
        }
        let next_intersection = self.next_intersection();
        let head = &mut self.body[0];
        let next_intersection_distance = (*head - next_intersection).length();
        if next_intersection_distance < f64::EPSILON {
            *head = next_intersection;
            self.direction = next_direction;
            self.next_direction = None;
            return Ok(());
        }
        let required_time = next_intersection_distance / self.speed;
        if required_time > *timespan {
            return Ok(());
        }
        self.next_direction = None;
        self.move_for(required_time, None, board, score)
            .and_then(|_| {
                *timespan = *timespan - required_time;
                self.direction = next_direction;
                Ok(())
            })
    }

    fn next_intersection(&self) -> Vector {
        let head = &self.body[0];
        let orthogonal_direction = if self.direction.x == 0_f64 { Vector::new(1_f64, 0_f64) } else { Vector::new(0_f64, 1_f64) };
        let head_projection = Vector::scalar_product(head, &self.direction);
        let nearest_intersection_projection = f64::floor(head_projection) + 0.5;
        let nearest_intersection_distance = f64::abs(nearest_intersection_projection - head_projection);
        if nearest_intersection_distance < f64::EPSILON {
            return orthogonal_direction * Vector::scalar_product(head, &orthogonal_direction) + self.direction * nearest_intersection_projection;
        }
        let intersection_projection = if (nearest_intersection_projection - head_projection) > 0 as f64 {
            head_projection + nearest_intersection_distance
        } else {
            head_projection + (1_f64 - nearest_intersection_distance)
        };
        return orthogonal_direction * Vector::scalar_product(head, &orthogonal_direction) + self.direction * intersection_projection;
    }

    fn nearest_collision_point(&self, distance: f64, board: &Board) -> Option<Vector> {
        let mut check_points: BTreeMap<usize, Vector> = BTreeMap::new();
        let mut collisions: HashSet<usize> = HashSet::new();
        let food = board.food.unwrap();
        let head = &self.body[0];
        let mut next_intersection_distance = (*head - self.next_intersection()).length();
        if next_intersection_distance - 0.5 > 0_f64 {
            next_intersection_distance -= 0.5
        }
        while next_intersection_distance < distance {
            check_points.insert(check_points.len(), *head + self.direction * next_intersection_distance);
            next_intersection_distance += 0.5;
        }
        let body_segments = if self.body.len() > 3 { LineSegment::segments_from_positions(&self.body[3..]) } else { Vec::new() };
        board.as_rectangle().as_line_segments().iter()
            .chain(body_segments.iter())
            .for_each(|segment| check_points.keys()
                .filter(|k| segment.contains(&check_points[k]))
                .for_each(|k| { collisions.insert(*k); }));
        check_points.keys()
            .filter(|k| food.position == check_points[k])
            .for_each(|k| { collisions.insert(*k); });
        collisions.iter().map(|collision| check_points[collision]).next()
    }

    fn check_collisions(&mut self, board: &mut Board, score: &mut i32) -> Result<(), GameOverEvent> {
        let head = self.body[0];
        let food = board.food.unwrap();
        if board.as_rectangle().as_line_segments().iter().any(|s| s.contains(&head)) {
            return Err(BorderCollisionEvent);
        }
        if food.position == head {
            food.on_collision(self, board, score)?
        }
        if self.body.len() > 3 && LineSegment::segments_from_positions(&self.body[3..]).iter().any(|s| s.contains(&head)) {
            return Err(SnakeCollisionEvent);
        }
        Ok(())
    }
}

// main module utils
impl Snake {
    pub fn render(&self, board: &Board) {
        let mut buff = (0..board.height).map(|_| (0..board.width).map(|_| '_').chain("\n".chars()).collect::<Vec<char>>()).flatten().collect::<Vec<char>>();
        {
            let Vector { x: food_x, y: food_y } = board.food.unwrap().position;
            buff[food_y.floor() as usize * (board.width + 1) + food_x.floor() as usize] = 'F';
        }
        {
            self.body[..self.body.len() - 1].iter()
                .zip(&self.body[1..])
                .map(|(&begin, &end)| (Vector::new(begin.x.floor(), begin.y.floor()), Vector::new(end.x.floor(), end.y.floor())))
                .map(|(begin, end)| {
                    let mut segment_points: Vec<Vector> = Vec::new();
                    segment_points.push(begin);
                    if begin.x == end.x {
                        (0..(f64::abs(begin.y - end.y)) as i32).map(|i| (i + 1) as f64)
                            .for_each(|i| segment_points.push(Vector::new(begin.x, i + if begin.y < end.y { begin.y } else { end.y })));
                    } else {
                        (0..(f64::abs(begin.x - end.x)) as i32).map(|i| (i + 1) as f64)
                            .for_each(|i| segment_points.push(Vector::new(i + if begin.x < end.x { begin.x } else { end.x }, begin.y)));
                    }
                    segment_points.push(end);
                    println!("{:?}", segment_points);
                    segment_points
                })
                .flatten()
                .for_each(|position| buff[position.y as usize * (board.width + 1) + position.x as usize] = 'S');
        }
        println!("{}", buff.iter().collect::<String>());
    }
}
