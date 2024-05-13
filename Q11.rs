// Merge two sorted arrays in Rust


use std::io;

fn merge(nums1: &mut Vec<i32>, m: usize, nums2: &[i32], n: usize) {
    let mut i = m as i32 - 1;
    let mut j = n as i32 - 1;
    let mut k = m as i32 + n as i32 - 1;
    
    while i >= 0 && j >= 0 {
        if nums1[i as usize] > nums2[j as usize] {
            nums1[k as usize] = nums1[i as usize];
            k -= 1;
            i -= 1;
        } else {
            nums1[k as usize] = nums2[j as usize];
            k -= 1;
            j -= 1;
        }
    }
    
    while j >= 0 {
        nums1[k as usize] = nums2[j as usize];
        k -= 1;
        j -= 1;
    }
}

fn main() {

    // Read input for the first array
    println!("Enter Array1 elements :");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("err");
    let arr1: Vec<i32> = input1
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("err"))
        .collect();

    // Read input for the second array
    println!("Enter Array2 elements :");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("err");
    let arr2: Vec<i32> = input2.trim().split_whitespace()
        .map(|s| s.parse().expect("err"))
        .collect();

    let mut merged_array = vec![0; arr1.len() + arr2.len()];
    merged_array[..arr1.len()].copy_from_slice(&arr1);
    
    // Merge the arrays
    merge(&mut merged_array, arr1.len(), &arr2, arr2.len());
    println!("Merged array: {:?}", merged_array);
}



