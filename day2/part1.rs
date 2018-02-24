use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("input.txt").expect("file not found");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("failed to read file");

    let mut sum = 0;
    for line in input.lines() {
        let mut min = u32::max_value();
        let mut max = 0;
        for num_str in line.split(" ") {
            let num: u32 = num_str.parse().unwrap();
            if num < min {
                min = num;
            }
            if num > max {
                max = num;
            }
        }
        sum += max - min;
    }
    println!("{}", sum);
}
