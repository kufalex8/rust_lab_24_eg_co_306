use tokio::time::{Duration, sleep};

async fn fetch_data(id: u32) -> String {
    // Simulate network delay
    sleep(Duration::from_millis(100)).await;

    format!("Data from source {}", id)
}

pub async fn main() {
    // Sequential execution
    let t0 = std::time::Instant::now();

    for id in 1..=4 {
        let data = fetch_data(id).await;
        println!("Sequential: {}", data);
    }

    println!("Sequential time: {:?}", t0.elapsed());

    println!();

    // Concurrent execution
    let t1 = std::time::Instant::now();

    let (r1, r2, r3, r4) = tokio::join!(fetch_data(1), fetch_data(2), fetch_data(3), fetch_data(4));

    println!("Concurrent: {}", r1);
    println!("Concurrent: {}", r2);
    println!("Concurrent: {}", r3);
    println!("Concurrent: {}", r4);

    println!("Concurrent time: {:?}", t1.elapsed());
}
