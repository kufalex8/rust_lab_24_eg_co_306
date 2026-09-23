# Lab 3

## Description

This lab demonstrates the use of Rust's core programming concepts, including structs, traits, dynamic dispatch, custom error handling, and modular programming. The lab is divided into three exercises and one stretch goal, each focusing on a different aspect of the Rust language.

## Exercises

### Exercise 1: Bank Account System

Implemented a `BankAccount` struct with methods to:

- Create a new bank account.
- Deposit funds into the account.
- Withdraw funds while checking for insufficient balance.
- Retrieve the current account balance.
- Handle withdrawal errors using `Result`.

### Exercise 2: Traits and Polymorphism

Implemented traits and trait objects using geometric shapes.

Features include:

- `Area` trait for calculating shape areas.
- `Describable` trait with a default method.
- Implementations of both traits for `Circle` and `Rectangle`.
- `Display` trait implementation for custom output formatting.
- Dynamic dispatch using trait objects (`&dyn Area`).

### Exercise 3: Custom Error Handling

Implemented a custom `AppError` enum to demonstrate robust error handling.

Features include:

- Parsing integer input.
- Validating input within a specified range.
- Detecting empty input.
- Preventing division by zero.
- Implementing the `Display` trait for user-friendly error messages.
- Automatic conversion from `ParseIntError` using the `From` trait.

## Stretch Goal

Implemented a student grading system using traits.

Features include:

- `Student` struct containing student names and scores.
- `GradeReport` trait providing:
  - Average score calculation
  - Highest score
  - Lowest score
  - Letter grade determination
- Display of complete grade reports for multiple students.

## Design Decisions

- Organized the lab into separate modules for each exercise and the stretch goal.
- Used structs and traits to promote code reuse and abstraction.
- Applied Rust's `Result` type and custom error enums for safe error handling.
- Used trait objects to demonstrate runtime polymorphism.
- Implemented the `Display` trait to improve output readability.

## Known Limitations

- Data is stored only in memory and is not persisted.
- User input is simulated rather than collected interactively.
- The bank account system supports only basic deposit and withdrawal operations.
- The grading system uses fixed sample data.

## Cargo Test Output

   Compiling lab_3 v0.1.0 (C:\Users\VICTOR OKOKO\Documents\rust_lab_24_eg_co_394\lab_3)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.49s
     Running unittests src\main.rs (C:\Users\VICTOR OKOKO\Documents\rust_lab_24_eg_co_394\target\debug\deps\lab_3-38ee14cfeb3a8b35.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

## Stretch Goal Status

Completed.