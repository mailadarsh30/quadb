// Find the maximum subarray sum in Rust

use std::io;

fn max_subarray_sum(nums: &[i32]) -> i32 {
    let mut sum = 0;
    let mut maxi = std::i32::MIN;

    for &num in nums {
        sum += num;
        maxi = std::cmp::max(sum, maxi);
        if sum < 0 {
            sum = 0;
        }
    }

    maxi
}

fn main() {
    println!("Enter elements:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not a number"))
        .collect();

    println!("Maximum subarray sum: {}", max_subarray_sum(&nums));
}
