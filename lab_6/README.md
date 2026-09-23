# Lab 6

## Description

This lab focuses on concurrent and asynchronous programming in Rust. It covers multithreading, synchronization, message passing, file handling, asynchronous programming with Tokio, and asynchronous network requests. The exercises demonstrate how Rust enables safe, efficient, and scalable concurrent applications.

## Exercises

### Exercise 1: Multithreading

Implemented concurrent computations using Rust's standard threading library.

Features include:

- Creating threads with `thread::spawn`.
- Executing multiple threads concurrently.
- Synchronizing threads using `join()`.
- Using `move` closures to transfer ownership safely.
- Dividing a large computation into multiple worker threads.
- Combining partial results into a final total.

### Exercise 2: Shared State Concurrency

Implemented shared mutable state using synchronization primitives.

Features include:

- Sharing data using `Arc`.
- Protecting shared data with `Mutex`.
- Coordinating multiple worker threads.
- Measuring execution time using `Instant`.
- Comparing a naive locking approach with an optimized implementation that minimizes lock contention.

### Exercise 3: Message Passing

Implemented communication between threads using channels.

Features include:

- Creating multi-producer, single-consumer (mpsc) channels.
- Sending computation results from worker threads.
- Using a custom `WorkResult` enum for successful results and errors.
- Handling both successful and failed computations.
- Aggregating values received through channels.

### Exercise 4: File Handling

Implemented file and filesystem operations using Rust's standard library.

Features include:

- Creating and writing log files.
- Reading files using buffered I/O.
- Counting the number of lines in a file.
- Filtering log entries.
- Recursively traversing directories.
- Listing Rust source files (`.rs`).
- Removing temporary files after processing.

### Exercise 5: Asynchronous Programming with Tokio

Implemented asynchronous programming using the Tokio runtime.

Features include:

- Writing asynchronous functions.
- Simulating asynchronous operations.
- Using `.await`.
- Running asynchronous tasks sequentially.
- Executing multiple asynchronous tasks concurrently using `tokio::join!`.
- Comparing sequential and concurrent execution times.

## Stretch Goal

Implemented an asynchronous web data collector.

Features include:

- Performing asynchronous HTTP requests using `reqwest`.
- Limiting concurrent requests using `tokio::sync::Semaphore`.
- Spawning asynchronous tasks with `tokio::spawn`.
- Counting the number of words in downloaded web pages.
- Writing results to a CSV file asynchronously using `tokio::fs`.
- Generating a summary report of processed URLs.

## Design Decisions

- Organized each exercise into separate modules for clarity and maintainability.
- Used Rust's ownership and borrowing system to guarantee thread safety.
- Applied synchronization primitives only where shared state was required.
- Preferred message passing over shared mutable state where appropriate.
- Leveraged Tokio for scalable asynchronous execution.
- Used asynchronous file operations to complement asynchronous networking.

## Known Limitations

- Network requests depend on internet connectivity and remote server availability.
- The web crawler performs basic word counting and does not parse HTML content.
- Error handling for HTTP requests is intentionally simple.
- Directory traversal assumes appropriate filesystem permissions.
- Thread-based examples use fixed workloads for demonstration purposes.

## Cargo Test Output

Compiling lab_6 v0.1.0 (C:\Users\USER\Documents\rust_lab_24_eg_co_306\lab_6)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.25s
     Running unittests src\main.rs (C:\Users\USER\Documents\rust_lab_24_eg_co_306\target\debug\deps\lab_6-1c5b45fd875f67b7.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


## Stretch Goal Status

Completed.