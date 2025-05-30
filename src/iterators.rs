// struct Shoe {
//     size: u32,
//     style: String,
// }

// fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
//     shoes
//         .into_iter()
//         .filter(|shoe| shoe.size >= shoe_size)
//         .collect()
// }

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        println!("I've been called");
        if self.count < 5 {
            self.count += 1;
            return Some(self.count);
        }
        None
    }
}

/// #Tests stuff
/// ```
/// let arg = 5;
/// calling_next_directly();
/// ```
pub fn calling_next_directly() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

fn using_other() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
}
