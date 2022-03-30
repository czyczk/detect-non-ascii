use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        for ch in line.unwrap().chars() {
            if !ch.is_ascii() {
                println!("'{}' detected as non-ASCII char.", ch);
                return;
            }
        }
    }

    println!("All in ASCII range.")
}
