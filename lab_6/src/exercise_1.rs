use std::thread;
use std::time::Duration;

pub fn main() {
    // Original example
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("[thread] count = {}", i);
            thread::sleep(Duration::from_millis(50));
        }
    });

    for i in 1..=3 {
        println!("[main] count = {}", i);
        thread::sleep(Duration::from_millis(80));
    }

    handle.join().expect("Thread panicked");
    println!("All done");

    println!("\n----- Exercise A -----");

    let mut handles = vec![];

    for part in 0..4 {
        let start = part * 250 + 1;
        let end = (part + 1) * 250;

        let handle = thread::spawn(move || {
            let mut sum = 0u64;

            for i in start..=end {
                sum += i as u64;
            }

            println!(
                "Thread {} calculated {}..{} = {}",
                part + 1,
                start,
                end,
                sum
            );

            sum
        });

        handles.push(handle);
    }

    let mut total = 0u64;

    for handle in handles {
        total += handle.join().unwrap();
    }

    println!("\nTotal Sum = {}", total);
}
