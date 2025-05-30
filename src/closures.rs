use std::{thread, time::Duration};

pub fn start() {
    let simluated_intensity = 10;
    let random = 7;
    generate_workout(simluated_intensity, random);
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Self {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(value) => value,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
// fn simulated_calculation(intensity: u32) -> u32 {
//     println!("calculating slowly");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

fn generate_workout(intensity: u32, random_number: u32) {
    let mut cached_result = Cacher::new(|num: u32| {
        println!("calculating intensity");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("do {} situps!", cached_result.value(intensity));
        println!("Next, do {} situp!", cached_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take break today");
        }
    }
}
