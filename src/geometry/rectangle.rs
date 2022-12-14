use crate::geometry::line_segment::LineSegment;
use crate::geometry::vector::Vector;

#[derive(Debug)]
pub struct Rectangle {
    pub bottom_left_vertex: Vector,
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn new(bottom_left_vertex: Vector, width: f64, height: f64) -> Self {
        Self { bottom_left_vertex, width, height }
    }

    pub fn as_line_segments(&self) -> [LineSegment; 4] {
        let bottom_left_vertex = &self.bottom_left_vertex;
        let bottom_right_vertex = &Vector::new(bottom_left_vertex.x + self.width, bottom_left_vertex.y);
        let top_right_vertex = &Vector::new(bottom_right_vertex.x, bottom_right_vertex.y + self.height);
        let top_left_vertex = &Vector::new(bottom_left_vertex.x, top_right_vertex.y);
        return [
            LineSegment::new(bottom_left_vertex.clone(), bottom_right_vertex.clone()),
            LineSegment::new(bottom_right_vertex.clone(), top_right_vertex.clone()),
            LineSegment::new(top_right_vertex.clone(), top_left_vertex.clone()),
            LineSegment::new(top_left_vertex.clone(), bottom_left_vertex.clone()),
        ];
    }
}
