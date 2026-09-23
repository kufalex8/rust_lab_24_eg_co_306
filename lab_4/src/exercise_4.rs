// Exercise D — Custom Iterator

struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let next = self.a + self.b;
        self.a = self.b;
        self.b = next;
        Some(self.a) // infinite iterator - never returns None
    }
}

/* TODO 6: Implement a `Primes` struct that is an infinite iterator
yielding prime numbers using a sieve approach. */
struct Primes {
    current: u64,
    primes_found: Vec<u64>,
}

impl Primes {
    fn new() -> Self {
        Primes {
            current: 2,
            primes_found: Vec::new(),
        }
    }
}

impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        loop {
            let candidate = self.current;
            self.current += 1;

            // Sieve-style trial division against previously found primes
            let is_prime = self
                .primes_found
                .iter()
                .take_while(|&&p| p * p <= candidate)
                .all(|&p| !candidate.is_multiple_of(p));

            if is_prime {
                self.primes_found.push(candidate);
                return Some(candidate);
            }
        }
    }
}

pub fn run() {
    let fibs: Vec<u64> = Fibonacci::new().take(15).collect();
    println!("First 15 Fibonacci: {:?}", fibs);

    // TODO 5: Find the first Fibonacci number greater than 1,000,000.
    let first_gt_1m = Fibonacci::new()
        .find(|&x| x > 1_000_000)
        .expect("Fibonacci sequence is infinite");
    println!("First Fibonacci > 1,000,000: {}", first_gt_1m);

    // To check if the Primes iterator works, let's print the first 20 primes.
    let first_20_primes: Vec<u64> = Primes::new().take(20).collect();
    println!("First 20 Primes: {:?}", first_20_primes);
}
