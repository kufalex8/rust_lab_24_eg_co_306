use std::sync::mpsc;
use std::thread;

#[derive(Debug)]
enum WorkResult {
    Sum(u64),
    Error(String),
}

fn worker(id: usize, data: Vec<u64>, tx: mpsc::Sender<WorkResult>) {
    let sum: u64 = data.iter().sum();

    println!("Worker {} computed sum = {}", id + 1, sum);

    // TODO 3:
    // If the chunk's sum is greater than 30000,
    // send an Error instead of Sum.
    if sum > 30000 {
        tx.send(WorkResult::Error(format!(
            "Worker {}: Sum {} is greater than 30000",
            id + 1,
            sum
        )))
        .unwrap();
    } else {
        tx.send(WorkResult::Sum(sum)).unwrap();
    }
}

pub fn main() {
    let (tx, rx) = mpsc::channel();

    let dataset: Vec<Vec<u64>> = (0..4)
        .map(|i| {
            ((i * 250 + 1)..=((i + 1) * 250))
                .map(|x| x as u64)
                .collect()
        })
        .collect();

    for (id, chunk) in dataset.into_iter().enumerate() {
        let tx_clone = tx.clone();

        thread::spawn(move || {
            worker(id, chunk, tx_clone);
        });
    }

    drop(tx);

    let mut total = 0u64;

    for result in rx {
        match result {
            WorkResult::Sum(value) => {
                println!("Received Sum: {}", value);
                total += value;
            }
            WorkResult::Error(message) => {
                println!("Received Error: {}", message);
            }
        }
    }

    println!("\nGrand Total = {}", total);
}
