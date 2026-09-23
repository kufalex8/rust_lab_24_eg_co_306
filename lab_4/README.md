# Lab 4

## Description

This lab explores Rust's standard collections, iterator patterns, closures, custom iterators, and file handling. The exercises demonstrate data processing, functional programming techniques, and reading structured data from CSV files using Rust's standard library.

## Exercises

### Exercise 1: Vector Operations

Implemented statistical analysis using Rust vectors.

Features include:

- Sorting a vector of floating-point numbers.
- Filtering high scores.
- Computing the mean, minimum, and maximum values.
- Calculating the median.
- Computing the variance.
- Calculating the standard deviation.

### Exercise 2: HashMap

Implemented a word frequency analyzer using `HashMap`.

Features include:

- Reading words from a string.
- Cleaning and normalizing words.
- Counting word occurrences.
- Sorting words by frequency.
- Displaying the most frequently occurring words.

### Exercise 3: Closures and Advanced Iterators

Demonstrated functional programming concepts in Rust.

Features include:

- Higher-order functions.
- Closures.
- Returning closures using `impl Fn`.
- Iterator chaining with `filter`, `map`, `take`, `collect`, `sum`, and `product`.
- Computing the sum of squares of odd numbers.
- Generating prime numbers using iterator methods.

### Exercise 4: Custom Iterators

Implemented custom iterators by implementing the `Iterator` trait.

Features include:

- Infinite Fibonacci sequence generator.
- Finding the first Fibonacci number greater than 1,000,000.
- Infinite prime number generator.
- Collecting and displaying generated values using iterator adapters.

## Stretch Goal

Implemented CSV data processing using Rust's standard library.

Features include:

- Reading data from a CSV file.
- Parsing column headers and records.
- Storing records in `HashMap<String, String>`.
- Displaying all student records.
- Filtering students by age.
- Computing the average student score.

## Design Decisions

- Organized each exercise into its own module for better maintainability.
- Used Rust's standard collections (`Vec` and `HashMap`) for efficient data storage.
- Leveraged iterators and closures instead of traditional loops where appropriate.
- Implemented custom iterators to demonstrate Rust's iterator framework.
- Used the standard library for CSV file reading without external dependencies.

## Known Limitations

- CSV parsing assumes well-formatted input and does not handle malformed files.
- Statistical calculations assume the input vector is non-empty.
- The custom iterators generate infinite sequences and rely on iterator adapters like `take()` or `find()` to terminate.
- Word frequency analysis only processes alphabetic characters.

## Cargo Test Output

 Compiling lab_4 v0.1.0 (C:\Users\USER\Documents\rust_lab_24_eg_co_306\lab_4)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.68s
     Running unittests src\main.rs (C:\Users\USER\Documents\rust_lab_24_eg_co_306\target\debug\deps\lab_4-56125d217ac80790.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

## Stretch Goal Status

Completed.