use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;

fn main() {
    let version = get_compiler_version();
    let binary_version = convert_text_to_binary(&version);
    // TODO: take date from command line and get number of days.
    let i = 1;
    let code_line = get_line_of_source(i);
    println!("{}\n{}", binary_version, code_line);
}

fn get_compiler_version() -> String {
    let output = Command::new("rustc")
        .arg("-V")
        .output()
        .expect("Failed command")
        .stdout;
    let matcher = Regex::new(r"\((.+)\)").unwrap();
    let version = String::from_utf8(output).expect("broken");
    println!("{}", version);
    let caps = matcher.captures(&version).unwrap();
    caps[1].to_string()
}

fn convert_text_to_binary(message: &String) -> String {
    let nums = message.clone().into_bytes();
    let mut ret: String = "".to_owned();
    for num in nums {
        ret.push_str(&format!("{:0>8b}", num));
    }
    ret = ret.replace("1", "🦀").replace("0", "🦝");
    ret
}

fn get_line_of_source(i: usize) -> String {
    let filename = "src/main.rs";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut num_lines = 0;
    for (index, _) in reader.lines().enumerate() {
        num_lines = index
    }

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        if i.rem_euclid(num_lines) == index {
            return line.unwrap();
        }
    }
    "".to_string()
}
