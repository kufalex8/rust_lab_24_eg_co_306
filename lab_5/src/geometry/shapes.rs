pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    pub fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

pub struct Polygon {
    pub vertices: Vec<Point>,
}

impl Polygon {
    // TODO 3:implement perimeter (&self) -> f64
    pub fn perimeter(&self) -> f64 {
        if self.vertices.len() < 2 {
            return 0.0;
        }

        let mut total = 0.0;

        for i in 0..self.vertices.len() {
            let current = &self.vertices[i];
            let next = &self.vertices[(i + 1) % self.vertices.len()];
            total += current.distance(next);
        }

        total
    }

    // TODO 4: implement is_closed(&self) -> bool
    pub fn is_closed(&self) -> bool {
        self.vertices.len() >= 3
    }
}
