use std::fmt::Display;

// Generic function with multiple trait bounds
fn print_largest<T: PartialOrd + Display>(list: &[T]) {
    if list.is_empty() {
        return;
    }

    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    println!("The largest is {}", largest);
}

// Generic struct
#[derive(Debug)]
struct Pair<T> {
    first: T,
    second: T,
}

impl<T: Display + PartialOrd> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Pair { first, second }
    }

    fn cmp_display(&self) {
        if self.first >= self.second {
            println!("First is larger: {}", self.first);
        } else {
            println!("Second is larger: {}", self.second);
        }
    }
}

// TODO 2: write a generic function.
fn zip_with<A, B, C, F>(a: &[A], b: &[B], f: F) -> Vec<C>
where
    F: Fn(&A, &B) -> C,
{
    let mut result = Vec::new();

    for (x, y) in a.iter().zip(b.iter()) {
        result.push(f(x, y));
    }

    result
}

pub fn main() {
    print_largest(&[34, 50, 25, 100, 65]);
    print_largest(&["mango", "apple", "banana"]);

    let p = Pair::new(5, 10);
    p.cmp_display();

    // TODO 2 Test
    let a = [1, 2, 3];
    let b = [10, 20, 30];

    let sums = zip_with(&a, &b, |x, y| x + y);
    println!("Sums: {:?}", sums);

    let products = zip_with(&a, &b, |x, y| x * y);
    println!("Products: {:?}", products);
}
