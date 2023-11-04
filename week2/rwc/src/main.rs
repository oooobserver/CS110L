use std::env;
use std::fs::File; 
use std::fs;
use std::io::{self, Read};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];

    let file = File::open(filename).unwrap();

    // output the number of words, lines, and characters in the file.

    let mut lines_number = 0;
    let mut characters_number = 0;

    for byte in io::BufReader::new(file).bytes() {
        match byte {
            Ok(byte) => {
                if byte as char == '\n' {
                    lines_number += 1;
                }
                characters_number += 1;
            }
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                break;
            }
        }
    }

    let contents = fs::read_to_string(filename).unwrap();

    let mut in_whitespace = false;
    let mut word_count = 1;

    for c in contents.chars() {
        if c.is_whitespace() {
            if !in_whitespace {
                in_whitespace = true;
                word_count += 1; // Increment word count for each whitespace sequence
            }
        } else {
            in_whitespace = false;
        }
    }

    // Adjust word count if the file ends with whitespace
    if in_whitespace {
        word_count -= 1;
    }

    println!("character: {}, word: {}, line: {}",characters_number,word_count,lines_number);
    




}
