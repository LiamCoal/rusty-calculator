mod calculator;
mod tokenizer;

extern crate colored;

use crate::calculator::calculate;
use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use colored::Colorize;
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    let variables: &mut HashMap<String, f64> = &mut HashMap::new();
    if std::env::args().len() == 1 {
        let mut rl = Editor::<()>::new();
        if rl.load_history("history.txt").is_err() {
            println!("No expression history found.");
        }
        loop {
            let mut exp = String::new();
            let readline = rl.readline(&*("<= : "));
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str());
                    exp = line;
                },
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break
                },
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break
                },
                Err(err) => {
                    println!("Error: {:?}", err);
                    break
                }
            }
            print!("{}", "=>   ");
            println!("{}", (&*calculate(&*exp, variables).to_string()).green().bold());
        }
        rl.save_history("history.txt").unwrap();
    } else {
        for arg in std::env::args().skip(1) {
            println!("=>   {}", calculate(&*arg, variables));
        }
    }

}
