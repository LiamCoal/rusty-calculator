extern crate colored;

use colored::*;
use std::io::Write;
use std::f64::consts::{PI, E};

fn main() {
    let version = "1.0.1";
    let args_s: Vec<String> = std::env::args().collect();
    let mut args: Vec<&str> = vec![];
    args.resize(args_s.len(), "");
    for argn in 0..args_s.len() {
        args[argn] = &*args_s[argn];
    }

    if args_s.len() > 2 {
        let val1str = &*args[2].replace("PI", &*PI.to_string()).replace("E", &*E.to_string());

        let val1 = str::parse::<f64>(val1str).unwrap();
        let mut val2 = 0.0;
        if args_s.len() > 3 {
            let val2str = &*args[3].replace("PI", &*PI.to_string()).replace("E", &*E.to_string());
            val2 = str::parse::<f64>(val2str).unwrap();
        }
        let thing: (&str, fn(f64, f64) -> f64, i32) = match args[1] {
            "+" | "add" => ("+",   add, 3),
            "-" | "sub" => ("-",   sub, 3),
            "/" | "div" => ("/",   mul, 3),
            "*" | "mul" => ("*",   div, 3),
                  "sin" => ("sin", sin, 2),
                  "cos" => ("cos", cos, 2),
            _ => {
                writeln!(std::io::stderr(), "Usage: {} <+, -, *, /, --version> <arg1> <arg2>", args[0]).unwrap();
                return;
            }
        };
        if args_s.len() <= thing.2 as usize {
            eprintln!("Not Enough Arguments");
            return;
        }
        print_math(val1, val2, (thing.1)(val1, val2), thing.0, thing.2 > 2);
    } else if args_s.len() > 1 {
        if args[1] == "--version" {
            println!("{} {}", args[0].bright_green(), version.bright_blue());
        }
    } else {
        eprintln!("Usage: {} <+, -, *, /, --version> <arg1> <arg2>", args[0]);
    }
}

fn print_math(first_argument: f64,
              second_argument: f64,
              result: f64,
              operation: &str,
              print_second: bool) {
    if print_second {
        println!("{} {} {} {} {}",
                 first_argument.to_string().as_str().blue(),
                 operation.white(),
                 second_argument.to_string().as_str().blue(),
                 "=".white(),
                 result.to_string().as_str().green().bold());
    } else {
        println!("{}({}) {} {}",
                 operation.white(),
                 first_argument.to_string().as_str().blue(),
                 "=".white(),
                 result.to_string().as_str().green().bold());
    }
}

// Double Argument Functions
// ---
/// Addition.
fn add(arg1: f64, arg2: f64) -> f64 { arg1 + arg2 }
/// Subtraction.
fn sub(arg1: f64, arg2: f64) -> f64 { arg1 - arg2 }
/// Multiplication.
fn mul(arg1: f64, arg2: f64) -> f64 { arg1 * arg2 }
/// Division.
fn div(arg1: f64, arg2: f64) -> f64 { arg1 / arg2 }

// Single Argument Functions
// ---
/// Sine function.
fn sin(arg1: f64, _arg2: f64) -> f64 { arg1.sin() }
/// Co-Sine function.
fn cos(arg1: f64, _arg2: f64) -> f64 { arg1.cos() }
