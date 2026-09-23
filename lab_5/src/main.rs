mod exercise_1;
mod exercise_2;
mod exercise_3;
mod stretch_goal;

mod geometry;
mod utils;

fn main() {
    println!("===== Exercise A =====");
    exercise_1::main();

    println!("\n===== Exercise B =====");
    exercise_2::main();

    println!("\n===== Exercise C =====");
    exercise_3::main();

    println!("\n===== Stretch Goal =====");
    stretch_goal::run();
}
