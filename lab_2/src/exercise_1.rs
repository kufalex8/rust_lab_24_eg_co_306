pub fn main() {
    // if as an expression
    let number: i32 = 7;

    let description = if number % 2 == 0 { "even" } else { "odd" };

    println!("{} is {}", number, description);
    let number: i32 = 306;

    let description = if number % 2 == 0 { "even" } else { "odd" };

    println!("{} is {}", number, description);

    // loop with break value ...
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // loop returns a value
        }
    };

    println!("Loop result: {}", result);

    // while
    let mut n = 2;

    while n < 100 {
        n *= 2;
    }

    println!("First power of 2 >= 100: {}", n);

    // for over a range
    let sum: i32 = (1..=100).sum();

    println!("Sum 1..=100 = {}", sum);

    for i in 1..=12 {
        println!("7 x {} = {}", i, 7 * i);
    }
}
