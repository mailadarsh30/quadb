// Implement a function that returns the kth smallest element in a given array.


use std::io;


fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k > arr.len() {
        return None;
    }

    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();

    Some(sorted_arr[k - 1])
}

fn main() {
    println!("Enter the Array :");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let arr: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("err"))
        .collect();
    println!("Enter the value of k ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read inpuut");
    let k: usize = input.trim().parse().expect("err");

    match kth_smallest(&arr, k) {
        Some(val) => println!("The {}th smallest ele is: {}", k, val),
        None => println!("Array index out of bounds!"),
    }
}
