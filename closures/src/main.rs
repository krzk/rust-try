use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn main() {
    let user_intensity = 10;
    let random_int = 5; // chosen by fair dice roll; guaranteed to be random

    generate_workout(user_intensity, random_int);
}

fn calculate_workout_task(intensity: u32) -> u32 {
    println!("Calculating the workout factor...");
    thread::sleep(Duration::from_secs(1));
    intensity * 2
}

struct CacherV1<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> CacherV1<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> CacherV1<T> {
        CacherV1 {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

struct CacherV2<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    values: HashMap<u32, u32>,
}

impl<T> CacherV2<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> CacherV2<T> {
        CacherV2 {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        // *self.values.entry(arg).or_insert((self.calculation)(arg)) // not good, closure got called each time
        let value = self.values.get(&arg);
        match value {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, factor: u32) {
    // let task = calculate_workout_task(intensity); // Approach standard
    // let task = |num| { // Approach basic closure
    //     println!("Calculating the workout factor...");
    //     thread::sleep(Duration::from_secs(1));
    //     num * 2
    // };
    let mut task = CacherV1::new(|num| {
        println!("Calculating the workout factor...");
        thread::sleep(Duration::from_secs(1));
        num * 2
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            task.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            task.value(intensity)
        );
    } else {
        if factor == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                task.value(intensity)
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_workout_task_positive() {
        assert_eq!(calculate_workout_task(3), 6);
    }

    #[test]
    fn CacherV1_value() {
        let calculation = |num| {
            println!("Test calculation, CacherV1 for {}", num);
            num * 3
        };
        let mut c = CacherV1::new(calculation);
        assert_eq!(c.value, None);
        assert_eq!(c.value(4), 12);
        assert_eq!(c.value, Some(12));
        assert_eq!(c.value(4), 12);
        assert_eq!(c.value, Some(12));
        assert_eq!(c.value(5), 12);
        assert_eq!(c.value, Some(12));
        assert_eq!(c.value(0), 12);
        assert_eq!(c.value, Some(12));
    }

    #[test]
    fn CacherV2_value() {
        let calculation = |num| {
            println!("Test calculation, CacherV2 for {}", num);
            num * 3
        };
        let mut c = CacherV2::new(calculation);
        assert_eq!(c.value(4), 12);
        assert_eq!(c.value(4), 12);
        assert_eq!(c.value(5), 15);
        assert_eq!(c.value(5), 15);
        assert_eq!(c.value(0), 0);
        assert_eq!(c.value(0), 0);
    }
}
