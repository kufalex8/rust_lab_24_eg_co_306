//execise 1.3
pub fn main() {
    // Immutable varriable
    let x = 5;
    println!("x = { }", x);

    //Mutable varriable
    let mut y = 10;
    println!("y before = {}", y);
    y += 5;
    println!("y after = {}", y);

    let pi: f64 = std::f64::consts::PI;
    let is_learning: bool = true;
    let grade: char = 'A';
    println!("pi = { }", pi);
    println!("is_learning ={ }", is_learning);
    println!("grade ={ }", grade);

    //shadowing
    let z = "42";
    let z: u32 = z.parse().expect("Not a number!");
    println!("parsed z ={ }", z);
}
