// Reverse a string in Rust

use std::io;

fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}

fn main() {
    println!("Enter a string:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let trimmed_input = input.trim();

    let reversed = reverse_string(trimmed_input);
    println!("Original: {}", trimmed_input);
    println!("Reversed: {}", reversed);
}
