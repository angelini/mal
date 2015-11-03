#![allow(dead_code)]

extern crate linenoise;

pub fn read_line(prompt: &str) -> Option<String> {
    let val = linenoise::input(prompt);
    match val {
        None => { None }
        Some(input) => {
            linenoise::history_add(&input);
            if input == "exit" {
                None
            } else {
                Some(input)
            }
        }
    }
}
