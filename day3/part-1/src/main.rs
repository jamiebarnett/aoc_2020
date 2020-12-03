use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    
    let file = match File::open("../input.txt") {
        Ok(f) => f,
        Err(err) => panic!("{}", err)
    };
    
    let mut inputs = Vec::new();
    let lines = io::BufReader::new(file).lines();
    for line in lines {
            if let Ok(line) = line {
            
                let line_input : Vec<char> = line.chars().collect();
                
                inputs.push(line_input);
            }
    }

    let mut tree_count = 0;
    let mut index = 3;
    for (i, input) in inputs.into_iter().enumerate() {
        if i == 0 {
            continue
        }
        println!("{} at index {} on row {}", input[index], index, i);
        if input[index] == '#' {
            println!("hit tree at {}", index);
            tree_count += 1;
        }
        index += 3;
        if index >= input.len(){
            index -= input.len();
        }
    };

    
    println!("hit {} trees", tree_count);
}
