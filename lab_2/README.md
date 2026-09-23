# Lab 2

## Description

This lab explores intermediate concepts in the Rust programming language, including control flow, pattern matching, enums, error handling, iterators, and user input processing. The exercises demonstrate Rust's ability to handle decision-making, data modeling, safe operations, and practical problem-solving techniques.

## Exercises

### Exercise 1: Control Flow and Loop Expressions

Implemented Rust control flow concepts using conditional expressions and different types of loops.

Features include:

- Using `if` expressions to return values.
- Checking whether numbers are even or odd.
- Demonstrating variable shadowing.
- Using infinite loops with `break` values.
- Returning values from loops.
- Implementing `while` loops for repeated execution.
- Using `for` loops with ranges.
- Calculating the sum of numbers in a range.
- Generating multiplication tables using iteration.

### Exercise 2: Enums and Pattern Matching

Implemented custom data types and pattern matching using Rust enums.

Features include:

- Creating a `Shape` enum with different variants:
  - Circle.
  - Rectangle.
  - Triangle.
- Calculating shape areas using pattern matching.
- Applying Heron's formula for triangle area calculation.
- Using references to prevent unnecessary ownership transfer.
- Matching string values to describe directions.
- Handling unknown cases with a default match pattern.

### Exercise 3: FizzBuzz Using Pattern Matching

Implemented the classic FizzBuzz problem using Rust pattern matching.

Features include:

- Using tuple patterns with `match`.
- Checking divisibility using the modulo operator.
- Returning:
  - "FizzBuzz" for numbers divisible by both 3 and 5.
  - "Fizz" for numbers divisible by 3.
  - "Buzz" for numbers divisible by 5.
  - The original number otherwise.
- Iterating through a range of numbers.

### Exercise 4: Error Handling with Option

Implemented safe error handling using Rust's `Option` type.

Features include:

- Creating functions that return optional values.
- Handling division by zero safely.
- Using `Some` and `None` variants.
- Processing optional values with `match`.
- Using `if let` for simplified pattern matching.
- Providing fallback values using `unwrap_or`.
- Searching collections using iterators.
- Finding the first even number in a list.

## Stretch Goal

Implemented a simple command-line calculator.

Features include:

- Reading user input from the terminal.
- Parsing mathematical expressions.
- Supporting basic arithmetic operations:
  - Addition.
  - Subtraction.
  - Multiplication.
  - Division.
- Handling invalid inputs safely.
- Preventing division by zero.
- Using pattern matching to select operations.

## Design Decisions

- Organized each exercise into separate modules for better readability and maintainability.
- Used Rust pattern matching to handle different cases clearly.
- Applied enums to represent structured data.
- Used `Option` for operations that may fail instead of unsafe error handling.
- Used iterators for efficient collection processing.
- Separated the calculator functionality into a stretch goal module.
- Avoided unnecessary ownership transfers by using references where appropriate.

## Known Limitations

- The calculator only supports basic arithmetic operations.
- The calculator does not support complex mathematical expressions.
- Shape calculations assume valid input values.
- FizzBuzz only works with unsigned integer values.
- Division uses floating-point numbers and may have precision limitations.

## Cargo Test Output
 Compiling lab_2 v0.1.0 (C:\Users\USER\Documents\rust_lab_24_eg_co_306\lab_2)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.32s
     Running unittests src\main.rs (C:\Users\USER\Documents\rust_lab_24_eg_co_306\target\debug\deps\lab_2-94cef7ee01ecb57e.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s