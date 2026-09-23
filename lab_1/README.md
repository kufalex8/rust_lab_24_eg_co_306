# Lab 1

## Description

This lab introduces the fundamental concepts of the Rust programming language, including variables, ownership, borrowing, functions, recursion, and basic string manipulation. The exercises demonstrate Rust's memory safety model and essential programming techniques.

## Exercises

### Exercise 1: Variables and Data Types

Implemented basic Rust variable declarations and primitive data types.

Features include:

- Declaring immutable variables.
- Declaring mutable variables.
- Updating mutable values.
- Using primitive data types (`f64`, `bool`, and `char`).
- Accessing Rust's built-in mathematical constant (`PI`).
- Demonstrating variable shadowing.
- Parsing a string into an integer.

### Exercise 2: Ownership and Borrowing

Implemented examples demonstrating Rust's ownership system.

Features include:

- Ownership transfer (move semantics).
- Cloning values to create independent copies.
- Borrowing data using immutable references.
- Calculating string length without taking ownership.
- Extracting the first word of a string using string slices.

### Exercise 3: Functions and Recursion

Implemented reusable functions and recursive algorithms.

Features include:

- Creating functions with parameters and return values.
- Returning formatted strings.
- Implementing a recursive factorial function.
- Demonstrating function calls and formatted output.

## Stretch Goal

Implemented additional utility functions.

Features include:

- Converting temperatures from Celsius to Fahrenheit.
- Checking whether a string is a palindrome.
- Using iterators to reverse strings.
- Demonstrating both utility functions with sample inputs.

## Design Decisions

- Organized each exercise into separate modules.
- Used Rust's ownership and borrowing model to ensure memory safety.
- Applied string slices where ownership transfer was unnecessary.
- Used recursion to demonstrate algorithm implementation.
- Kept each exercise independent for easier understanding and testing.

## Known Limitations

- The factorial function may overflow for very large input values.
- The `first_word` function assumes words are separated by spaces.
- The palindrome checker is case-sensitive and does not ignore whitespace or punctuation.
- Example values are hard-coded for demonstration purposes.

## Cargo Test Output

Compiling lab_1 v0.1.0 (C:\Users\VICTOR OKOKO\Documents\rust_lab_24_eg_co_394\lab_1)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.51s
     Running unittests src\main.rs (C:\Users\VICTOR OKOKO\Documents\rust_lab_24_eg_co_394\target\debug\deps\lab_1-3efd608bdccde28e.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

## Stretch Goal Status

Completed.