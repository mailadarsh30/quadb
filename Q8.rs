// Given a binary tree, implement a function that returns the maximum depth of the tree.

use std::cmp;
use std::io;


// Definition for a binary tree node.
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    // #[allow(dead_code)]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn build_tree() -> Option<Box<TreeNode>> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let val: i32 = input.trim().parse().expect("Invalid input");

    if val == -1 {
        return None;
    }

    let left = build_tree();
    let right = build_tree();

    Some(Box::new(TreeNode { val, left, right }))
}

fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(node.left);
            let right_depth = max_depth(node.right);
            cmp::max(left_depth, right_depth) + 1
        }
        None => 0,
    }
}

fn main() {
    println!("Enter the values :");
    let root = build_tree();

    let depth = max_depth(root);
    println!("Depth of the tree: {}", depth);
}
