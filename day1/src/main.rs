use std::{collections::BinaryHeap, cmp::Reverse};

fn main() {
    println!("Hello, world!");
    let mut count :u32 = 0;
    let mut heap: BinaryHeap<Reverse<u32>> = BinaryHeap::new();

    for line in include_str!("input").lines() {
        if line.is_empty() {
            // eval sum and go to next
            heap.push(Reverse(count));
            count = 0;
            if heap.len() > 3 {
                heap.pop();
            }
            continue;
        }
        count += line.parse::<u32>().unwrap();
    }
    let max_first = heap.iter().rev().next().unwrap().0;
    let max_second: u32 = heap.iter().map(|num | num.0).sum();
    println!("{} {}", max_first, max_second);
}
