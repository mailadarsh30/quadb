// Given a sorted array of integers, implement a function that returns the median of the array.

use std::io;

fn find_median(arr: &[i32]) -> f64 {
    let n = arr.len();
    if n % 2 == 0 {
        // if array len is even return avg of middle two elements.
        let mid_right = n / 2;
        let mid_left = mid_right - 1;
        (arr[mid_left] + arr[mid_right]) as f64 / 2.0
    } else {
        // If the length of the array is odd, return the middle element........
        arr[n / 2] as f64
    }
}
fn main() {
    println!("Enter sorted array:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid input"))
        .collect();
    println!("Median is : {}", find_median(&arr));
}
