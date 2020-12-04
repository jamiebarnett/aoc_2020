use std::io;
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Passport {
    byr : i16,
    iyr : i16,
    eyr : i16,
    hgt : String,
    hcl : String,
    ecl : String,
    pid : String
}

fn main() {
    
    let mut f = File::open("../input.txt").expect("could not open input file");
    let mut buffer = String::new();

    f.read_to_string(&mut buffer).expect("could not read input file to string");

    let birth_year_regex = Regex::new(r"byr:([0-9]{4})").unwrap();

    let lines : Vec<&str> = buffer.split("\n\n").collect();
    let mut valid_passport_count = 0;
    for line in lines {
        
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
        } else {
            match birth_year_regex.captures(&line) {
                Some(birth_year_match) => {
                    match birth_year_match {
                        Some(birth_year_match[1]) => {
                            // validate...
                        },
                        None => continue
                    }
                },
                None => continue
            }
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