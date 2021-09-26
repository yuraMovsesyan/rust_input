use std::io;
use std::io::prelude::*;

fn main() {
    let mut matrix: Vec<Vec<f64>>= Vec::new();
    let stdin = io::stdin();
    let mut i = 0;
    for line in stdin.lock().lines() {
        let line = line.unwrap();

        let words: Vec<&str> = line.split_whitespace().collect();

        let mut numbers: Vec<f64> = Vec::new();

        for i in  words.iter() {
            let num: f64 = i.parse().unwrap();
            numbers.push(num);
        }

        println!("{:?}", numbers);

        matrix.push(numbers);

        i += 1;

        if i >= 2{
            break;
        }

        println!("{}", i);
    }

    println!("\n\n{:?}", matrix);
}
