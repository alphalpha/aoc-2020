mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn get_lines(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|iter| iter.unwrap()).collect()
}

fn main() {
    day_1::solve("data/aoc-1-expenses.txt");
    day_2::solve("data/aoc-2-passwords.txt");
    day_3::solve("data/aoc-3-trees.txt");
    day_4::solve("data/aoc-4-passports.txt");
    day_5::solve("data/aoc-5-seats.txt");
}
