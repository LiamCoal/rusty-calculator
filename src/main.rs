mod calculator;
mod tokenizer;

extern crate colored;

use crate::calculator::calculate;
use std::collections::HashMap;
use std::io::{stdin, stdout, Write, Read};
use colored::Colorize;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::fs::File;
use std::env::args;
use std::borrow::Borrow;

fn main() {
    let variables: &mut HashMap<String, f64> = &mut HashMap::new();
    let mut args = args().collect::<Vec<String>>();
    let argsv = &*args.clone();
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
    } else if File::open(argsv.get(1).unwrap()).is_ok() {
        let mut script = File::open(argsv.get(1).unwrap()).unwrap();
        let mut file = String::new();
        script.read_to_string(&mut file);
        for line in file.split("\n") {
            println!("{} => {}", line.italic(), (&*calculate(line, variables).to_string()).green().bold())
        }
    } else {
        for arg in std::env::args().skip(1) {
            println!("=>   {}", calculate(&*arg, variables));
        }
    }

}
