use crate::geometry::vector::Vector;

#[derive(Debug)]
pub struct LineSegment {
    pub begin: Vector,
    pub end: Vector,
}

impl LineSegment {
    pub fn new(begin: Vector, end: Vector) -> LineSegment { Self { begin, end } }

    pub fn as_vector(&self) -> Vector { self.end - self.begin }

    pub fn is_point(&self) -> bool { self.begin == self.end }

    pub fn length(&self) -> f64 { self.as_vector().length() }

    pub fn direction(&self) -> Vector { self.as_vector().normalized() }

    pub fn contains(&self, point: &Vector) -> bool {
        let start_to_point = LineSegment::new(self.begin, point.clone());
        let end_to_point = LineSegment::new(point.clone(), self.end);
        let dist = f64::abs(self.length() - (start_to_point.length() + end_to_point.length()));
        dist < f64::EPSILON
    }

    pub fn segments_from_positions(positions: &[Vector]) -> Vec<LineSegment> {
        positions[..positions.len() - 1].iter()
            .zip(&positions[1..])
            .map(|(&begin, &end)| LineSegment::new(begin, end))
            .collect::<Vec<LineSegment>>()
    }
}
