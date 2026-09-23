# Lab 5

## Description

This lab explores advanced Rust programming concepts, including lifetimes, generics, trait bounds, modular project organization, and reusable data structures. The exercises demonstrate safe borrowing, generic programming, module design, geometric transformations, and the implementation of a generic cache.

## Exercises

### Exercise 1: Lifetimes

Implemented lifetime annotations to ensure safe borrowing and prevent dangling references.

Features include:

- A `longest` function using explicit lifetime parameters.
- A struct containing borrowed data (`Important`).
- Methods that safely return borrowed string slices.
- A `first_sentence` function that extracts the first sentence from a string without allocating additional memory.
- Demonstration of Rust's borrow checker and lifetime rules.

### Exercise 2: Generics and Trait Bounds

Implemented generic functions and data structures.

Features include:

- A generic function for finding the largest element in a collection.
- A generic `Pair<T>` struct.
- Trait bounds using `Display` and `PartialOrd`.
- A generic `zip_with` function that combines two collections using a closure.
- Demonstrations of generic programming with different operations.

### Exercise 3: Modules and Geometry

Organized the project into reusable modules.

Features include:

- A `Point` structure representing two-dimensional coordinates.
- Distance calculation between points.
- A `Polygon` structure containing multiple vertices.
- Perimeter calculation for polygons.
- Checking whether a polygon is closed.
- Translation of points and polygons using geometric transformations.
- Utility functions for displaying polygon information.
- Separation of functionality into `geometry` and `utils` modules.

## Stretch Goal

Implemented a generic cache with automatic capacity management.

Features include:

- Generic `Cache<K, V>` implementation.
- Storage using `HashMap`.
- Generic type constraints with `Eq`, `Hash`, and `Clone`.
- Automatic removal of the oldest entry when capacity is exceeded.
- Updating access order whenever an item is retrieved.
- Demonstration of inserting, retrieving, and evicting cached values.

## Design Decisions

- Divided the project into independent modules for improved organization and maintainability.
- Used Rust's ownership and lifetime system to ensure memory safety.
- Leveraged generics and trait bounds to maximize code reusability.
- Encapsulated geometry-related functionality within a dedicated module.
- Used a `HashMap` together with an ordering structure to implement a reusable cache.

## Known Limitations

- The cache uses a simple vector to maintain access order, which is not the most efficient approach for large datasets.
- The polygon implementation assumes vertices are provided in the correct order.
- Geometry calculations are limited to two-dimensional polygons.
- The cache stores data only in memory and does not provide persistent storage.

## Cargo Test Output

 Compiling lab_5 v0.1.0 (C:\Users\USER\Documents\rust_lab_24_eg_co_306\lab_5)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.58s
     Running unittests src\main.rs (C:\Users\USER\Documents\rust_lab_24_eg_co_306\target\debug\deps\lab_5-a68196afdfa9efbe.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

## Stretch Goal Status

Completed.