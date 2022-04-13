use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;
use std::vec::Vec;

struct Cacher<T, K, U>
where
    T: Fn(K) -> U,
    K: Hash + Eq + Copy,
    U: Copy,
{
    calculation: T,
    value: HashMap<K, U>,
}

impl<T, K, U> Cacher<T, K, U>
where
    T: Fn(K) -> U,
    K: Hash + Eq + Copy,
    U: Copy,
{
    fn new(calculation: T) -> Cacher<T, K, U> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: K) -> U {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    }
}

struct TreeNode<'a T>{
    val: T,
    children: &'a Vec<TreeNode>
}

impl<T> TreeNode<T>{
    fn new(val: T)->TreeNode<T>{
       TreeNode{
           val, 
           None
        }
    }
}

fn tree_map<T>(f:fn(T)->T, tree:TreeNode<T>) {
    
}

fn forest_map(fn, forest) {

}
