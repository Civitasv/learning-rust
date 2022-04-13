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

// struct TreeNode<'a> {
//     val: u32,
//     children: Vec<Option<Box<&'a TreeNode<'a>>>>,
// }

// impl<'a> TreeNode<'a> {
//     fn new(val: u32) -> TreeNode<'a> {
//         TreeNode {
//             val: val,
//             children: Vec::new(),
//         }
//     }
// }

// fn tree_map(f: fn(&mut u32), tree: &mut Box<&TreeNode>) {
//     f(&mut tree.val);
//     forest_map(f, &mut tree.children);
// }

// fn forest_map(f: fn(&mut u32), forest: &mut Vec<Option<Box<&TreeNode>>>) {
//     for item in forest {
//         let treenode = match item {
//             Some(s) => s,
//             None => continue,
//         };
//         tree_map(f, treenode);
//     }
// }
// #[cfg(test)]
// mod tests {
//     use crate::{tree_map, TreeNode};

//     #[test]
//     fn tree_map_test() {
//         let mut root = TreeNode::new(1);
//         let mut item1 = TreeNode::new(2);
//         let mut item2 = TreeNode::new(3);
//         let mut item3 = TreeNode::new(4);
//         let mut item4 = TreeNode::new(5);

//         root.children.push(Some(Box::new(&item1)));
//         root.children.push(Some(Box::new(&item2)));
//         item1.children.push(Some(Box::new(&item3)));
//         item1.children.push(Some(Box::new(&item4)));

//         tree_map(
//             |value| {
//                 *value = *value + 1;
//             },
//             &mut Box::new(root),
//         )
//     }
// }
