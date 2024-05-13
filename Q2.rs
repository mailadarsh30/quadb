// Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.

use std::io;

fn search(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;

        if arr[mid] == target {
            if mid == 0 || arr[mid - 1] != target {
                return Some(mid);
            } else {
                // check on left side...
                right = mid - 1;
            }
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    None
}
fn main() {
    println!("Enter Array");
    let mut input = String::new();
    io::stdin().read_line(&mut input);

    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|num| num.parse().expect("Invalid number"))
        .collect();

    println!("Enter the target number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read input.");
    let target: i32 = input.trim().parse().expect("Invalid number");

    match search(&arr, target) {
        Some(index) => println!("First occurrence {} is at index {}", target, index),
        None => println!("{} not found in the array", target),
    }
}

