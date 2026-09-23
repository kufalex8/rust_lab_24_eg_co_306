use std::fmt;
#[allow(dead_code)]
trait Describable {
    fn describe(&self) -> String;

    // Default method
    fn short_name(&self) -> String {
        self.describe()[..20.min(self.describe().len())].to_string()
    }
}

trait Area {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
}

// TODO 2: Implement Area for Rectangle
impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Describable for Circle {
    fn describe(&self) -> String {
        format!("Circle with radius {:.2}", self.radius)
    }
}

// TODO 2: Implement Describable for Rectangle
impl Describable for Rectangle {
    fn describe(&self) -> String {
        format!(
            "Rectangle with width {:.2} and height {:.2}",
            self.width, self.height
        )
    }
}

// TODO 3: Implement fmt::Display for both Circle and Rectangle

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.describe())
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.describe())
    }
}

// Trait object - dynamic dispatch
fn print_area(shape: &dyn Area) {
    println!("Area = {:.4}", shape.area());
}

pub fn main() {
    let c = Circle { radius: 3.0 };
    let r = Rectangle {
        width: 4.0,
        height: 5.0,
    };

    print_area(&c);
    print_area(&r);

    println!("{}", c.describe());
    println!("{}", r.describe());

    // Display trait
    println!("{}", c);
    println!("{}", r);
}
