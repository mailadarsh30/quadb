// Implement a function that finds the longest common prefix of a given set of strings.


use std::io;

fn main() {
    println!("Enter the strings :");

// user input
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    // split the string into individuals.............
    let strs: Vec<String> = input.split_whitespace().map(String::from).collect();

    // Function calling...
    let res = Prefix::common_Prefix(strs);
    println!("Longest Common Prefix: {}", res);
}

struct Prefix;

impl Prefix {
    pub fn common_Prefix(strs: Vec<String>) -> String {
        let mut ans = String::new();
        let mut strs = strs;
        strs.sort();
        let a = strs.first().unwrap_or(&String::new()).to_string();
        let b = strs.last().unwrap_or(&String::new()).to_string();
        for (ca, cb) in a.chars().zip(b.chars()) {
            if ca == cb {
                ans.push(ca);
            } else {
                break;
            }
        }
        
        ans
    }
}
