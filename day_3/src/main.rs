extern crate clap;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use clap::App;

type Geography = Vec<Vec<bool>>;

fn count_tree_intersections(geo : &Geography, x_incr : usize, y_incr : usize) -> u32 {
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    while y < geo.len() {
        x = x % geo[y].len();
        count += geo[y][x] as u32;
        x += x_incr;
        y += y_incr;
    }
    count
}

fn main() {
    let matches = App::new("TobogganRoutePlanner")
        .version("0.1")
        .author("Jacob Lundgren")
        .about("Determines the number of trees with which a toboggan path will intersect")
        .args_from_usage(
            "<FILENAME>     'The name of the file holding the geography'")
        .get_matches();

    let path = Path::new(matches.value_of("FILENAME").unwrap());
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("Unable to open {}: {}", display, why),
        Ok(file) => BufReader::new(file),
    };

    let char_conv = | c | {
        match c {
            '.' => false,
            '#' => true,
            _ => panic!("Invalid character found: {}", c)
        }
    };
    let geo : Geography = file.lines().map(
        | line | -> Vec<bool> {
            line.unwrap().as_str().chars().map(char_conv).collect()
        }
    ).collect();

    println!(
        "Number of trees intersected with (1, 1): {}\n\
        Number of trees intersected with (3, 1): {}\n\
        Number of trees intersected with (5, 1): {}\n\
        Number of trees intersected with (7, 1): {}\n\
        Number of trees intersected with (1, 2): {}\n",
        count_tree_intersections(&geo, 1, 1),
        count_tree_intersections(&geo, 3, 1),
        count_tree_intersections(&geo, 5, 1),
        count_tree_intersections(&geo, 7, 1),
        count_tree_intersections(&geo, 1, 2),
    );

}
