use chrono::{NaiveDate, NaiveTime, Utc};
use chrono_tz::US::Central;
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

fn get_line_of_source(i: i64) -> String {
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
        if i.rem_euclid(num_lines as i64) == index as i64 {
            return line.unwrap();
        }
    }
    "".to_string()
}

fn get_time_since_date(date_str: &str) -> i64 {
    let dt = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .expect("Could not parse date.")
        .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
        .and_local_timezone(Central);
    let dt = dt.unwrap();
    let now = Utc::now();
    let diff = now - &dt.with_timezone(&Utc);
    diff.num_days()
}
