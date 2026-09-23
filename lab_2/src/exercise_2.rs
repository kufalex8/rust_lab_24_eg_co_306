//exercise 2.3
#[derive(Debug)]
enum Shape {
    Circle(f64),             // radius
    Rectangle(f64, f64),     // width, height
    Triangle(f64, f64, f64), // sides a, b, c
}

fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,

        Shape::Rectangle(w, h) => w * h,

        Shape::Triangle(a, b, c) => {
            // Heron's formula
            let s = (a + b + c) / 2.0;
            (s * (s - a) * (s - b) * (s - c)).sqrt()
        }
    }
}

fn describe_direction(direction: &str) {
    // TODO 3
    match direction {
        "North" => println!("Heading North - towards the mountains"),
        "South" => println!("Heading South"),
        "East" => println!("Heading East"),
        "West" => println!("Heading West"),
        _ => println!("Unknown direction"),
    }
}

pub fn main() {
    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Rectangle(4.0, 6.0),
        Shape::Triangle(3.0, 4.0, 5.0),
    ];

    for s in &shapes {
        println!("{:?} -> area = {:.2}", s, area(s));
    }

    describe_direction("North");
}
