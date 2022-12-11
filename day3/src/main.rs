#![feature(iter_next_chunk)]
use std::collections::HashSet;


fn main() {
    let lines = include_str!("input")
        .lines()
        .map(|line |  {
            let (left, right) = line.split_at(line.len()/2);
            let left_set: HashSet<char> = left.chars().collect();
            let right_set: HashSet<char> = right.chars().collect();

            (left_set, right_set)
        })
        .map(|(a,  b)| {
            a.intersection(&b).collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("{lines:?}");
}
