use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct PasswordPolicy {
    min: usize,
    max: usize,
    character: char,
    password: String,
}

fn main() {
    
    let file = match File::open("../input.txt") {
        Ok(f) => f,
        Err(err) => panic!("{}", err)
    };
    
    let mut inputs = Vec::new();
    let lines = io::BufReader::new(file).lines();
    for line in lines {
            if let Ok(line) = line {
                
                let split_input_line: Vec<&str> = line.split(' ').collect();
                let character : &str = split_input_line[1].strip_suffix(":").expect("suffix not found");
                let parsed_char : Vec<char> = character.chars().collect();
                let password = String::from(split_input_line[2]);
                let min_max: Vec<&str> = split_input_line[0].split('-').collect();
                let min : usize = min_max[0].parse().expect("could not parse min input");
                let max : usize = min_max[1].parse().expect("could not parse min input");
                let policy = PasswordPolicy{
                    character: parsed_char[0],
                    min,
                    max,
                    password
                };
                inputs.push(policy);
            }
    }

    let mut count = 0;
    for input in inputs {
        let password : Vec<char> = input.password.chars().collect();
        let first_index = input.min - 1;
        let second_index = input.max - 1;
        if password[first_index] == input.character || password[second_index] == input.character {
            if password[first_index] == input.character && password[second_index] == input.character {
                println!("input {:?} is INVALID", input);
            } else {
                println!("input {:?} is VALID", input);
                count += 1
            } 
        } else {
            println!("input {:?} is INVALID", input);
        }
    }
    println!("got {} valid passwords", count);
}
