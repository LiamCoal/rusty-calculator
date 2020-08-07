mod calculator;
mod tokenizer;

extern crate colored;

use crate::calculator::calculate;
use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use colored::Colorize;

fn main() {
    let variables: &mut HashMap<String, f64> = &mut HashMap::new();
    if std::env::args().len() == 1 {
        loop {
            print!("{}", "<= : ".white());
            stdout().flush().expect("Unable to flush output.");
            let mut exp = String::new();
            stdin().read_line(&mut exp).expect("Error.");
            print!("{}", "=>   ".white());
            println!("{}", (&*calculate(&*exp, variables).to_string()).bright_green().bold());
        }
    } else {
        for arg in std::env::args().skip(1) {
            println!("=>   {}", calculate(&*arg, variables));
        }
    }

}
