// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

impl std::ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, _rhs: Point) -> Point {
        Point {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl std::ops::Sub<Point> for Point {
    type Output = Point;
    fn sub(self, _rhs: Point) -> Point {
        Point {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}

impl Point {
    // add methods
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
    pub fn magnitude(&self) -> f64 {
        return f64::from(self.x * self.x + self.y * self.y).sqrt();
    }
    pub fn dist(self, other: Self) -> f64 {
        return (other - self).magnitude();
    }
}

pub struct Polygon {
    // add fields
    points: Vec<Point>,
}

impl Polygon {
    // add methods
    pub fn new() -> Self {
        Polygon { points: Vec::new() }
    }
    pub fn add_point(&mut self, new_point: Point) {
        self.points.push(new_point)
    }
    pub fn left_most_point(&self) -> Option<Point> {
        return self.points.iter().min_by_key(|p| p.x).copied();
    }
    pub fn iter(&self) -> impl Iterator<Item = &Point> {
        return self.points.iter();
    }
    pub fn length(&self) -> f64 {
        if self.points.len()==0 {
            return 0.0
            
        }
        let mut sums:f64 =0.0;
        let mut last_point = self.points[0];
        for point in &self.points[1..] {
            sums += point.dist(last_point);
            last_point = *point;
        }
        sums += last_point.dist(self.points[0]);
        return sums
    }
}

pub struct Circle {
    // add fields
    center: Point,
    radius: i32,
}

impl Circle {
    // add methods
    pub fn new(center: Point, radius: i32) -> Self {
        Circle { center, radius }
    }
    pub fn circumference(&self) -> f64 {
        2.0*std::f64::consts::PI*f64::from(self.radius)
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl Shape {
    pub fn perimeter(&self) -> f64 {
        match self {
            Shape::Circle(circ) => circ.circumference(),
            Shape::Polygon(poly) => poly.length()
        }
    }
}

impl From<Polygon> for Shape {
    fn from(poly: Polygon) -> Self {
        Shape::Polygon(poly)
    }
}
impl From<Circle> for Shape {
    fn from(circ: Circle) -> Self {
        Shape::Circle(circ)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_perimeters() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
            Shape::from(poly),
            Shape::from(Circle::new(Point::new(10, 20), 5)),
        ];
        let perimeters = shapes
            .iter()
            .map(Shape::perimeter)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        assert_eq!(perimeters, vec![15.48, 31.42]);
    }
}

#[allow(dead_code)]
fn main() {}
