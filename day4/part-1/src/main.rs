use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    
    let mut f = File::open("../input.txt").expect("could not open input file");
    let mut buffer = String::new();

    f.read_to_string(&mut buffer).expect("could not read input file to string");

    let lines : Vec<&str> = buffer.split("\n\n").collect();
    let mut valid_passport_count = 0;
    for line in lines {
        
        println!("{:?}", line);
        
        if !line.contains("ecl"){
            continue
        }
        if !line.contains("pid"){
            continue
        }
        if !line.contains("eyr"){
            continue
        }
        if !line.contains("hcl"){
            continue
        }
        if !line.contains("byr"){
            continue
        }
        if !line.contains("iyr"){
            continue
        }
        if !line.contains("hgt"){
            continue
        }
        valid_passport_count += 1;
    }

    println!("got {} valid passports", valid_passport_count)

}