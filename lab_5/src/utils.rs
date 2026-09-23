use crate::geometry::shapes::Polygon;

pub fn print_polygon_info(poly: &Polygon) {
    println!("Polygon Information");
    println!("-------------------");
    println!("Number of vertices: {}", poly.vertices.len());
    println!("Perimeter: {:.2}", poly.perimeter());
    println!("Closed: {}", poly.is_closed());
}
