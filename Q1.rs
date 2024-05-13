// Implement a function that checks whether a given string is a palindrome or not.



use std::io;

fn is_palindrome(s: &str) -> bool {
    let lowercased = s.to_lowercase(); // Convert input string to lowercase
    lowercased.chars().eq(lowercased.chars().rev())
}
fn main() {
    println!("Enter a string:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let input = input.trim(); // Remove trailing newline
    
    if is_palindrome(input) {
        println!("The string \"{}\" is a palindrome.", input);
    } else {
        println!("The string \"{}\" is not a palindrome.", input);
    }
}
