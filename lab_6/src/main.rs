mod exercise_1;
mod exercise_2;
mod exercise_3;
mod exercise_4;
mod exercise_5;
mod stretch_goal;

#[tokio::main]
pub async fn main() {
    exercise_1::main();
    exercise_2::main();
    exercise_3::main();
    let _ = exercise_4::main();
    exercise_5::main().await;
    stretch_goal::run().await;
}
