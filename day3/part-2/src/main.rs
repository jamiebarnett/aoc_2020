use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug,Clone)]
struct Slope {
    right: usize,
    down: usize,
    trees_hit: usize
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
            
                let line_input : Vec<char> = line.chars().collect();
                
                inputs.push(line_input);
            }
    }

    let mut slopes : Vec<Slope> = [
        Slope{right: 1, down: 1, trees_hit: 0},
        Slope{right: 3, down: 1, trees_hit: 0},
        Slope{right: 5, down: 1, trees_hit: 0},
        Slope{right: 7, down: 1, trees_hit: 0},
        Slope{right: 1, down: 2, trees_hit: 0},
    ].to_vec();
    for (j ,slope) in slopes.clone().into_iter().enumerate() {
        println!("slope {}", j);
        let mut tree_count = 0;
        let mut index = slope.right;
        for (i, input) in inputs.clone().into_iter().enumerate().step_by(slope.down) {
            if i == 0 {
                continue
            }
            println!("{} at index {} on row {}", input[index], index, i);
            if input[index] == '#' {
                println!("hit tree at {}", index);
                tree_count += 1;
            }
            index += slope.right;
            if index >= input.len(){
                index -= input.len();
            }
        };
        slopes[j].trees_hit = tree_count;
    }

    let mut output = 1;
    for (i, slope) in slopes.into_iter().enumerate() {
        println!("slope {} hit {} trees", i, slope.trees_hit);
        output *= slope.trees_hit;
    };
    println!("output : {}", output);
}
