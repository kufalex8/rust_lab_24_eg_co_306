// 4.4Exercise C — Closures & Advanced Iterators

fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(f(x))
}

fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    (2..=((n as f64).sqrt() as u32)).all(|i| !n.is_multiple_of(i))
}

pub fn run() {
    println!("--- Exercise C: Closures & Advanced Iterators ---");
    let double = |x| x * 2;
    println!("apply_twice(double, 3) = {}", apply_twice(double, 3));

    let add10 = make_adder(10);
    println!("add10(5) = {}", add10(5));

    // Chained iterators
    let result: Vec<String> = (1..=20)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .take(5)
        .map(|x| format!("{:>4}", x))
        .collect();
    println!("First 5 even squares: {}", result.join(""));

    // fold (reduce)
    let product: u64 = (1..=10).product();
    println!("10! = {}", product);

    // TODO 4a: Using only iterator methods (no loops)
    // Compute the sum of squares of odd numbers from 1 to 99
    let sum_odd_squares: u64 = (1..100)
        .filter(|x| x % 2 != 0)
        .map(|x| (x * x) as u64)
        .sum();
    println!(
        "Sum of squares of odd numbers from 1 to 99: {}",
        sum_odd_squares
    );

    // TODO 4b: Using only iterator methods (no loops)
    // Collect all prime numbers up to 50 into a Vec<u32>
    let primes: Vec<u32> = (1..=50).filter(|&x| is_prime(x)).collect();
    println!("Primes up to 50: {:?}", primes);
}
