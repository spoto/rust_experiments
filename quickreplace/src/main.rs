// for instance:
// cargo run "world" "Rust" test.txt test-modified.txt; cat test-modified.txt
// cargo run "[[a-z]" "Rust" test.txt test-modified.txt
use std::{env, fs};
use regex::Regex;
use text_colorizer::Colorize;

fn main() {
    let args = Arguments::new(&env::args().skip(1).collect());
    let data = read(&args.filename);
    let replaced_data = replace(&args.target, &args.replacement, &data);
    write(&args.output, &replaced_data)
}

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String
}

impl Arguments {
    fn new(args: &Vec<String>) -> Arguments {
        if args.len() != 4 {
            print_usage();
            eprintln!("{} wrong number of arguments: expected 4, got {}.",
                      "Error".red().bold(), args.len());
            std::process::exit(1);
        };

        Arguments {
            target: args[0].clone(),
            replacement: args[1].clone(),
            filename: args[2].clone(),
            output: args[3].clone()
        }
    }
}

fn write(output: &String, replaced_data: &String) {
    match fs::write(output, &replaced_data) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{} failed to write to file '{}': {:?}", "Error:".red().bold(), output, e);
            std::process::exit(1);
        }
    }
}

fn read(filename: &String) -> String {
    match fs::read_to_string(filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to read from file '{}': {:?}", "Error:".red().bold(), filename, e);
            std::process::exit(1);
        }
    }
}

fn replace(target: &str, replacement: &str, text: &str) -> String {
    match Regex::new(target) {
        Ok(regex) => regex.replace_all(text, replacement).to_string(),
        Err(e) => {
            eprintln!("{} failed to replace text: {:?}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    eprintln!("{} - change occurrences of one string into another", "quickreplace".green());
    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}