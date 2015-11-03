#![allow(dead_code)]

use std::io;
use std::io::prelude::*;

pub fn read_line(prompt: &str) -> Option<String> {
    print!("{}", prompt);
    io::stdout().flush().ok().expect("Could not flush stdout");

    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .ok()
        .expect("Failed to read line");

    let len = line.len();
    line.remove(len - 1);

    if line == "exit" {
        None
    } else {
        Some(line)
    }
}
