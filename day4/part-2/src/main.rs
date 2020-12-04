use std::io;
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Passport {
    byr : String,
    iyr : String,
    eyr : String,
    hgt : String,
    hcl : String,
    ecl : String,
    pid : String
}

fn main() {
    
    let mut f = File::open("../input.txt").expect("could not open input file");
    let mut buffer = String::new();
    let hcl_re = Regex::new(r"^#[0-9a-f]{6}$").expect("could not compile regex");

    f.read_to_string(&mut buffer).expect("could not read input file to string");

    let lines : Vec<&str> = buffer.split("\n\n").collect();
    let mut valid_passport_count = 0;
    for (i, line) in lines.into_iter().enumerate() {

        let wrapped_line = String::from(format!("{{{}}}", line)).replace(":", "\":\"").replace(" ", "\",\"").replace("\n", "\",\"").replace("}", "\"}").replace("{", "{\"").replace(",\"\"", "");
        
        let passport : Passport; 
        match serde_json::from_str(&wrapped_line) {
            Ok(p) => {
                passport = p
            }
            Err(e) => {
                println!("passport {} : {:?}", i, e);
                continue
            }, 
        };

        match passport.byr.parse::<i16>() {
            Ok(byr) => {
                if byr < 1920 || byr > 2002 {
                    continue
                }
            }
            Err(e) => {
                println!("passport {} : {:?}", i, e);
                continue
            }, 
        };

        match passport.iyr.parse::<i16>() {
            Ok(iyr) => {
                if iyr < 2010 || iyr > 2020 {
                    continue
                }
            }
            Err(e) => {
                println!("passport {} : {:?}", i, e);
                continue
            }, 
        };

        match passport.eyr.parse::<i16>() {
            Ok(eyr) => {
                if eyr < 2020 || eyr > 2030 {
                    continue
                }
            }
            Err(e) => {
                println!("passport {} : {:?}", i, e);
                continue
            }, 
        };

        if passport.hgt.contains("cm"){
            let split_height : Vec<&str> = passport.hgt.split("cm").collect();
            match split_height[0].parse::<i16>() {
                Ok(hgt) => {
                    if hgt < 150 || hgt > 193 {
                        continue
                    }
                }
                Err(e) => {
                    println!("passport {} failed to parse cm height : {:?}", i, e);
                    continue
                }, 
            }
        } else if passport.hgt.contains("in"){
            let split_height : Vec<&str> = passport.hgt.split("in").collect();
            match split_height[0].parse::<i16>() {
                Ok(hgt) => {
                    if hgt < 59 || hgt > 76 {
                        continue
                    }
                }
                Err(e) => {
                    println!("passport {} failed to parse in height : {:?}", i, e);
                    continue
                }, 
            }
        } else {
            continue
        }

        if !hcl_re.is_match(&passport.hcl) {
            println!("regex does not match : {}", passport.hcl);
            continue
        }

        match passport.ecl.as_str() {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => (),
            _ =>  {
                    println!("invalid eye colour : {}", passport.ecl);
                    continue
            }
        }

        println!("pid len : {}", passport.pid.len());
        if passport.pid.len() != 9 {
            println!("invalid pid : {}", passport.pid);
            continue
        }

        match passport.pid.parse::<isize>() {
            Ok(_) => (),
            Err(e) => {
                println!("passport {} failed to parse id : {:?}", i, e);
                continue
            }, 
        }

        valid_passport_count += 1;                
    }

    println!("got {} valid passports", valid_passport_count)

}