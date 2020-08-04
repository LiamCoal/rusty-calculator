# rusty-calculator
A calculator written in [Rust](https://www.rust-lang.org).

![Rust](https://github.com/LiamCoal/rusty-calculator/workflows/Rust/badge.svg)

## wat
This is a calculator that runs in your terminal, written in Rust.
- - -
These are the things that are supported:

* Addition: `+` or `add`
* Subtraction: `-` or `sub`
* Multiplication: `*` or `mul` (NOTE: You need to prefix `*` with backslash to get it to work on bash, like this: `\*`)
* Division: `/` or `div`

These are the things that are planned on:

* Constants like PI or E
* `sin`, `cos`, `tan`
* Input
* Variables

## Building and Running
### Running
You can use:
```bash
git clone https://github.com/LiamCoal/rusty-calculator.git &&
cd rusty-calculator &&
cargo run
```
You will need rust installed on your computer, so go to https://rustup.rs, copy the command, and run it. You might need to prefix it with `sudo`.

### Building
Pretty similar:
```bash
cargo build --release
```

###### meow