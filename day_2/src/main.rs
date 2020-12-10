extern crate clap;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use clap::App;

#[derive(Debug)]
struct Requirement {
    c : char,
    l_num : u32,
    r_num : u32,
}

fn parse_entry(line : &str) -> (Requirement, &str) {
    let tokens : Vec<&str> = line.split(&['-', ' ', ':'][..]).filter(| &s | s.len() > 0).collect();
    (Requirement {
        c : tokens[2].chars().nth(0).unwrap(),
        l_num : tokens[0].parse().unwrap(),
        r_num : tokens[1].parse().unwrap(),
    }, tokens[3])
}

fn entry_valid(line : &str) -> bool {
    let (req, password) = parse_entry(&line);
    match password.chars().filter(| &c | c == req.c).count() as u32 {
        x if x >= req.l_num && x <= req.r_num => true,
        _ => false,
    }
}

fn entry_valid2(line : &str) -> bool {
    let (req, password) = parse_entry(&line);
    (password.chars().nth((req.l_num - 1) as usize).unwrap() == req.c) ^ (password.chars().nth((req.r_num - 1) as usize).unwrap() == req.c)
}

fn main() {
    let matches = App::new("PasswordChecker")
        .version("0.1")
        .author("Jacob Lundgren")
        .about("Finds passwords that do not satisfy the associated requirement")
        .args_from_usage(
            "<FILENAME>     'The name of the file holding the passwords and requirements'")
        .get_matches();

    let path = Path::new(matches.value_of("FILENAME").unwrap());
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("Unable to open {}: {}", display, why),
        Ok(file) => BufReader::new(file),
    };

    let lines : Vec<_> = file.lines().collect();

    let count_valid = lines.iter().fold(0, | acc, line | acc + (entry_valid(line.as_ref().unwrap().as_str()) as u32));
    println!("The number of valid passwords according to the old requirement system is {}", count_valid);
    let count_valid_new = lines.iter().fold(0, | acc, line | acc + (entry_valid2(line.as_ref().unwrap().as_str()) as u32));
    println!("The number of valid passwords according to the new requirement system is {}", count_valid_new);
}

