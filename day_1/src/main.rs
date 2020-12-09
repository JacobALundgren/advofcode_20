extern crate clap;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::Path;

use clap::App;

fn get_occurrences(v : &[i32]) -> HashMap<i32, i32> {
    let mut hash = HashMap::<i32, i32>::new();
    for val in v {
        let count = hash.entry(*val).or_insert(0);
        *count += 1;
    }
    hash
}

fn two_sum(hash : &HashMap<i32, i32>, target : i32, blocklist : &[i32]) -> Option<(i32, i32)> {
    let predicate = | &(val, _) : &(&i32, &i32) | {
        let count_blocked = blocklist.iter().filter(| &x | *x == *val).count() as i32;
        if 2 * val == target {
            return hash.get(val).unwrap() > &(1 + count_blocked);
        }
        match hash.get(&(target - val)) {
            Some(other_count) => other_count > &(count_blocked),
            None => false
        }
    };
    let first_num = hash.iter().find(predicate);
    match first_num {
        Some(num) => Some((*num.0, target - *num.0)),
        None => None
    }
}

fn three_sum(hash : &HashMap<i32, i32>, target : i32) -> Option<(i32, i32, i32)> {
    for (val, _) in hash.iter() {
        let blocklist = [*val];
        match two_sum(&hash, target - val, &blocklist) {
            Some((second, third)) => return Some((*val, second, third)),
            None => ()
        }
    }
    None
}

fn main() {
    let matches = App::new("ExpenseReport")
        .version("0.1")
        .author("Jacob Lundgren")
        .about("Finds numbers that add up to 2020 and calculates their product")
        .args_from_usage(
            "<FILENAME>     'The name of the file holding the expenses'")
        .get_matches();

    let path = Path::new(matches.value_of("FILENAME").unwrap());
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("Unable to open {}: {}", display, why),
        Ok(file) => BufReader::new(file),
    };

    let nums : Result<Vec<i32>, Error> = file.lines()
        .map(| line | line.and_then(| val | val.parse().map_err(| e | Error::new(ErrorKind::InvalidData, e))))
        .collect();

    let nums = nums.unwrap();

    let hash = get_occurrences(&nums);
    {
        let (first, second) = two_sum(&hash, 2020, &[]).unwrap();
        println!("Two sum numbers are {}, {}. Product is {}", first, second, first * second);
    }
    {
        let res = three_sum(&hash, 2020).unwrap();
        println!("Three sum numbers are {}, {}, {}. Product is {}", res.0, res.1, res.2, res.0 * res.1 * res.2);
    }
}
