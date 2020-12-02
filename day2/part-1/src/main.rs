use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct PasswordPolicy {
    min: isize,
    max: isize,
    character: String,
    password: String,
}

fn main() {
    
    let file = match File::open("../input.txt") {
        Ok(f) => f,
        Err(err) => panic!("{}", err)
    };
    
    let mut input = Vec::new();
    let lines = io::BufReader::new(file).lines();
    for line in lines {
            if let Ok(line) = line {
                
                let split_input_line: Vec<&str> = line.split(' ').collect();
                let character = String::from(split_input_line[1].strip_suffix(":").expect("suffix not found"));
                let password = String::from(split_input_line[2]);
                let min_max: Vec<&str> = split_input_line[0].split('-').collect();
                let min : isize = min_max[0].parse().expect("could not parse min input");
                let max : isize = min_max[1].parse().expect("could not parse min input");
                let policy = PasswordPolicy{
                    character,
                    min,
                    max,
                    password
                };
                println!("{:?}", policy);
                input.push(policy);
            }
    }
}
