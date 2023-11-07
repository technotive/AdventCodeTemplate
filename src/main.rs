use std::{env, fs};

mod days; mod solutions;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => run_all(),
        2 => run_one(&args[1]),
        _ => eprintln!("Too many arguments")
    }
}

fn run_one(arg: &String) {
    let Ok(i) = arg.parse() else { eprintln!("Not a positive number: {arg}"); return; };
    if i > 25 { eprintln!("Case number was {i}, but must be at most 25."); return; }
    run(i);
}

fn run_all() {
    for i in 0..=25 { run(i); }
}

fn run(i: usize) {
    println!("\x1b[90m=============================================== DAY {:0>2}\x1b[0m", i);
    let path = format!("input/{:0>2}.txt", i);
    if let Ok(input) = fs::read_to_string(&path) {
        days::solve(i, &input);
    } else {
        println!("\x1b[33mNo input file '{path}'\x1b[0m")
    }
}
