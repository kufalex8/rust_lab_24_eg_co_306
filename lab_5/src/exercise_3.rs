use crate::geometry::shapes::{Point, Polygon};
use crate::geometry::transforms::translate_polygon;
use crate::utils::print_polygon_info;

pub fn main() {
    let square = Polygon {
        vertices: vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(1.0, 1.0),
            Point::new(0.0, 1.0),
        ],
    };

    println!("Original Polygon");
    print_polygon_info(&square);

    let moved = translate_polygon(&square, 2.0, 3.0);

    println!("\nTranslated Polygon");
    print_polygon_info(&moved);
}
