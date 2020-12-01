use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    
    let file = match File::open("../input.txt") {
        Ok(f) => f,
        Err(err) => panic!("{}", err)
    };
    
    let mut input = Vec::new();
    let lines = io::BufReader::new(file).lines();
    for line in lines {
            if let Ok(line) = line {
                let i : isize = line.parse().expect("could not parse input");
                input.push(i);
            }
    }

    for i in input.clone() {
        for j in &input {
            for k in &input {
                if i + j + k == 2020 {
                    println!("{} + {} + {} = 2020", i, j, k);
                    println!("{} * {} * {} = {}", i, j, k, i * j * k)
                }
            }
        }
    }

}
