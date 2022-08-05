use std::{collections::HashMap, thread, time::Duration};
fn main() {
    let simulated_user_specified_value = 30;
    let simulated_random_number = 3;
    generate_workout(simulated_user_specified_value, simulated_random_number);
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
    let u = vec![1, 2, 3];
    let equal_to_x_vec = move |z| z == u;
    //println!("can't ust u here {:?}", u); //Code will not compile consider getting ride of pritln!
    let v = vec![2, 3, 4];
    assert!(equal_to_x_vec(v));
    let v1 = vec![2, 3, 5, 6, 72, 2];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("{}", val);
    }
}
#[test]
fn iteratore_demonstration() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let results = (self.calculation)(arg);
                self.value.insert(arg, results);
                results
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num: u32| {
        println!("calculating slowly..");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today do {} pushups", expensive_closure.value(intensity));
        println!("Next, do {}  situps", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated");
        } else {
            println!(
                "Today, run for {} minutes",
                expensive_closure.value(intensity)
            );
        }
    }
}
#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| {
        println!("Function called {}", a);
        a
    });
    let v1 = c.value(1);
    let v2 = c.value(2);
    assert_eq!(v2, 2);
    assert_eq!(v1, 1);
}
