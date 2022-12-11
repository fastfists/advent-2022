#![feature(iter_next_chunk)]
use std::collections::HashSet;


fn main() {
    let lines = include_str!("input")
        .lines()
        .map(|line |  {
            let mut chars = line.chars().collect::<Vec<_>>();
            chars.chunks(2).collect::<Vec<_>>();
        });

    lines.for_each (|  line | println!("{line:?}"));
    // println!("{lines:?}");
}
