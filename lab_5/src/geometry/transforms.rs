use super::shapes::{Point, Polygon};

pub fn translate(point: &Point, dx: f64, dy: f64) -> Point {
    Point::new(point.x + dx, point.y + dy)
}

pub fn translate_polygon(poly: &Polygon, dx: f64, dy: f64) -> Polygon {
    let mut vertices = Vec::new();

    for p in &poly.vertices {
        vertices.push(translate(p, dx, dy));
    }

    Polygon { vertices }
}
