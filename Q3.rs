

use std::io;

fn short_word(input: &str) -> &str {
    let mut short = input.trim();
    
    for word in input.split_whitespace() {
        if word.len() < short.len() {
            short = word;
        }
    }
    short
}
fn main() {
    println!("Enter a string of words:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input line bro");
    let shortest = short_word(&input.trim());
    
    println!("The shortest word is: {}", shortest);
}
