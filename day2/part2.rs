use std::fs::File;
use std::io::prelude::*;

fn divisible_ratio(array: &Vec<u32>) -> u32 {
    for i in 0..array.len()-1 {
        let x = array[i];
        for j in i+1..array.len() {
            let y = array[j];
            if x > y && x % y == 0 {
                return x / y;
            } else if y % x == 0 {
                return y / x;
            }
        }
    }
    0
}

fn main() {
    let mut f = File::open("input.txt").expect("file not found");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("failed to read file");

    let mut sum = 0;
    for line in input.lines() {
        let mut nums: Vec<u32> = Vec::new();
        for num_str in line.split(" ") {
            let num: u32 = num_str.parse().unwrap();
            nums.push(num);
        }
        sum += divisible_ratio(&nums);
    }
    println!("{}", sum);
}
