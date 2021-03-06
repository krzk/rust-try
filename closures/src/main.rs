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

struct CacherV3<T, K, V>
    where T: Fn(&K) -> V,
          K: Eq + std::hash::Hash,
          V: Copy,
{
    calculation: T,
    values: HashMap<K, V>,
}

impl<T, K, V> CacherV3<T, K, V>
    where T: Fn(&K) -> V,
          K: Eq + std::hash::Hash,
          V: Copy,
{
    fn new(calculation: T) -> CacherV3<T, K, V> {
        CacherV3 {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: K) -> V {
        let value = self.values.get(&arg);
        match value {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(&arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

// Cacher for values without Copy trait
// I am not sure if it is possible - the CacherV4.value() would
// have interface like HashMap.get() and HashMap.insert(). But these are
// conflicting interfaces and cannot be done in one method...
//
// struct CacherV4<T, K, V>
//     where T: Fn(&K) -> V,
//           K: Eq + std::hash::Hash,
// {
//     calculation: T,
//     values: HashMap<K, V>,
// }

// impl<T, K, V> CacherV4<T, K, V>
//     where T: Fn(&K) -> V,
//           K: Eq + std::hash::Hash,
// {
//     fn new(calculation: T) -> CacherV4<T, K, V> {
//         CacherV4 {
//             calculation,
//             values: HashMap::new(),
//         }
//     }

//     fn value<'a>(&'a mut self, arg: K) -> &'a V {
//         let value = self.values.get(&arg);
//         match value {
//             Some(v) => v,
//             None => {
//                 let v = (self.calculation)(&arg);
//                 //let v_ret = &v;
//                 let k2 = &arg;
//                 self.values.insert(arg, v);
//                 // v_ret
//                 let v2 = self.values.get(k2).unwrap();
//                 v2
//             }
//         }
//     }
// }

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

    #[test]
    fn CacherV3_value_u32_u32() {
        let calculation = |num: &u32| {
            println!("Test calculation, CacherV3 for {}", num);
            num * 3
        };
        let mut c = CacherV3::new(calculation);
        assert_eq!(c.value(4), 12);
        assert_eq!(c.value(4), 12);
        assert_eq!(c.value(5), 15);
        assert_eq!(c.value(5), 15);
        assert_eq!(c.value(0), 0);
        assert_eq!(c.value(0), 0);
    }

    #[test]
    fn CacherV3_value_string_usize() {
        let calculation = |num: &String| {
            println!("Test calculation, CacherV3 for {}", num);
            num.len() * 3
        };
        // FIXME: No clue how to set first type to Cacher<>
        // Something like: (Fn(String) -> u32)?
        let mut c: CacherV3<_, String, usize> = CacherV3::new(calculation);
        assert_eq!(c.value(String::from("1234")), 12);
        assert_eq!(c.value(String::from("1234")), 12);
        assert_eq!(c.value(String::from("12345")), 15);
        assert_eq!(c.value(String::from("12345")), 15);
        assert_eq!(c.value(String::from("")), 0);
    }
}
