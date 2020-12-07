use ::day_05;
use std::{collections::BinaryHeap};

fn main () {
    let res = include_str!("input.txt")
    .lines()
    .map(|l| day_05::get_seat_id(l))
    .max()
    .unwrap();
    println!("{:?}", res);

    //Part 2
    let lines = include_str!("input.txt")
    .lines();

    let v: BinaryHeap<i32> = lines
    .map(|l| day_05::get_seat_id(l))
    .collect();

    let result = day_05::find_missing(&v.into_sorted_vec());
    println!("{:?}", result);
}

