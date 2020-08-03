extern crate colored;

use colored::*;
use std::io::Write;

fn main() {
    let version = "1.0.1";
    let args_s: Vec<String> = std::env::args().collect();
    let mut args: Vec<&str> = vec![];
    args.resize(args_s.len(), "");
    for argn in 0..args_s.len() {
        args[argn] = &*args_s[argn];
    }

    if args_s.len() > 3 {
        let val1 = str::parse::<f64>(args[2]).unwrap();
        let val2 = str::parse::<f64>(args[3]).unwrap();
        match args[1] {
            "+" | "add" => {
                print_math(val1, val2, val1 + val2, "+");
            }
            "-" | "sub" => {
                print_math(val1, val2, val1 - val2, "-");
            }
            "*" | "mul" => {
                print_math(val1, val2, val1 * val2, "*");
            }
            "/" | "div" => {
                print_math(val1, val2, val1 / val2, "/");
            }
            _ => {
                writeln!(std::io::stderr(), "Usage: {} <+, -, *, /, --version> <arg1> <arg2>", args[0]).unwrap();
            }
        }
    } else if args_s.len() > 1 {
        if args[1] == "--version" {
            println!("{} {}", args[0].bright_green(), version.bright_blue());
        }
    } else {
        writeln!(std::io::stderr(), "Usage: {} <+, -, *, /, --version> <arg1> <arg2>", args[0]).unwrap();
    }
}

fn print_math(first_argument: f64,
              second_argument: f64,
              result: f64,
              operation: &str) {
    println!("{} {} {} {} {}",
             first_argument.to_string().as_str().blue(),
             operation.white(),
             second_argument.to_string().as_str().blue(),
             "=".white(),
             result.to_string().as_str().green().bold());
}
