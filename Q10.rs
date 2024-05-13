// Check if a number is prime in Rust

use std::io;

fn isprime(num: u32) -> bool {
    if num <= 1 {
        return false;
    }
    if num <= 3 {
        return true;
    }
    if num % 2 == 0 || num % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= num {
        if num % i == 0 || num % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn main() {
    println!("Enter Number :");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num: u32 = input.trim().parse().expect("Please enter a valid num");

    if isprime(num) {
        println!("{} is prime.", num);
    } else {
        println!("{} is not prime.", num);
    }
}
