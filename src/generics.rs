use std::vec;

// fn largest(vector: Vec<i32>) -> i32 {
//     let mut largest = vector[0];
//     for number in &vector {
//         if number > &largest {
//             largest = *number;
//         }
//     }
//     largest
// }

fn largest<T: PartialOrd + Copy>(vector: Vec<T>) -> T {
    let mut largest = vector[0];
    for number in &vector {
        if number > &largest {
            largest = *number;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    fn print(&self) {
        println!("printing stuff");
    }
}

impl Point<f64> {
    pub fn somethingforjustfloats(&self) -> Self {
        Point { x: 69.0, y: 420.0 }
    }
}

pub fn generics() {
    println!("{}", largest(vec![1, 2, 3, 4, 5]));
    println!("{}", largest(vec!['a', 'b', 'c']));
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1.2, y: 69.2 };
    p2.somethingforjustfloats();
    p2.print();
    p1.print();
}
