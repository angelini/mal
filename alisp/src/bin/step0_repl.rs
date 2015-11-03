extern crate alisp;

use alisp::core;

fn read(str: String) -> String {
    str
}

fn eval(ast: String) -> String {
    ast
}

fn print(exp: String) -> String {
    exp
}

fn main() {
    loop {
        let line = core::read_line("user> ");
        match line { None => break, _ => () }
        println!("{}", print(eval(read(line.unwrap()))));
    }
}
