extern crate levenshtein;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use levenshtein::*;

fn read_consensus() -> String {
    let path = Path::new("consensus");
    let display = path.display();

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open {}: {}", display,
                           why.description()),
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => println!("successfully read file!"),
        Err(why) => panic!("couldn't read {}: {}", display,
                           why.description()),
    }
    s
}

fn is_router_line(line: &str) -> bool {
    let name = line.split(" ").nth(0);
    match name {
        Some(n) => n == "r",
        None => false,
    }
}

fn is_match(first: &str, second: &str) -> bool {
    let distance = levenshtein(first, second);
    distance <= 3
}

fn get_router_name(line: &str) -> Result<&str, &str> {
    let name = line.split(" ").nth(1);
    match name {
        Some(n) => return Ok(n),
        None => return Err("malformed router line"),
    }
}

fn contains_match(name: &str, all: &Vec<&str>) -> bool {
    all.iter()
       .find(|x| is_match(&name, x))
       .is_some()
}

fn get_matches(router_lines: Vec<&str>) -> Vec<Vec<&str>> {
    let mut all_router_matches:Vec<Vec<&str>> = Vec::new();

    for line in router_lines {
        let name = match get_router_name(line) {
            Ok(n) => n,
            Err(_) => continue,
        };

        let num_matched;

        {
            num_matched = all_router_matches.iter_mut()
                .filter_map(|x|  {
                    if contains_match(&name, &x) {
                        x.push(name);
                        return Some(x)
                    }
                    None
                }).count();
        }

        if num_matched == 0 {
            let new_match = vec![name];
            all_router_matches.push(new_match);
        }
    }

    all_router_matches
}

fn main() {
    println!("Getting similar router lines....");

    let consensus = read_consensus();
    let router_lines = consensus.split("\n")
                                .filter(|x| is_router_line(x))
                                .collect::<Vec<&str>>();

    // Only interested in groups of size greater than one
    let _matches = get_matches(router_lines).iter()
                                .filter(|x| x.len() > 1)
                                .map(|x| format!("count: {}, matches: {} \n", x.len().to_string(), &x.join(" ,")))
                                .collect::<Vec<String>>()
                                .join(" \n\n");

    print!("{}", _matches);
}
