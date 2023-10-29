// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn print(Vec: &Vec<char>, P:&Vec<char>){
    for i in 0..Vec.len(){
        if Vec[i]== '-'{
            print!("{}",P[i]);
        }else{
            print!("{}","-");
        }
    }
    print!("\n");
}

fn main() {
    let word = pick_a_random_word();
    let mut chars: Vec<char> = word.chars().collect();
    let mut positions = Vec::new();
    for i in 0..chars.len(){
        positions.push('-');
    }

    let mut chances = NUM_INCORRECT_GUESSES;


    while chances != 0{
        let mut flag = 1;
        for i in &chars{
            if *i != '-'{
                flag = 0;
            }
        }

        if flag == 1{
            println!("game over, you win!");
            break;
        }
      


        print!("Please guess a letter: ");

        io::stdout()
            .flush()
            .expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");

        if let Some(char) = guess.chars().next() {
            if chars.contains(&char){
                println!("you are goddamn right!");
                if let Some(index) = chars.iter().position(|&x| x == char){
                    positions[index] = char;
                    chars[index] = '-';
                }else{
                    println!("wrong");
                }
            }else{
                println!("you are powerless!");
                chances -= 1;
                println!("remain chances are {}",chances);
            }
        } else {
            println!("String is empty");
        }

        print(&chars,&positions);

    }


}
