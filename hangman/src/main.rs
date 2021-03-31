use std::io;
use std::io::Read;
use std::io::Write;
use std::fs::File;
use std::ops::Not;
use rand::seq::SliceRandom;

struct HangmanLetter {
    letter: char,
    found: bool,
}

fn file_to_strings(path : String) -> Result<Vec<String>, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let words_string = contents.split("\n");
    let words = words_string.map(|s| s.trim().to_string()).collect::<Vec<String>>();
    Ok(words)
}

fn char_to_hangman_letter(c : char) -> HangmanLetter {
    HangmanLetter {
        letter: c,
        found: false,
    }
}

fn word_found(hls : &Vec<HangmanLetter>) -> bool {
    let mut found = true;
    for hl in hls {
        if hl.found.not() {
            found = false;
        }
    }
    found
}

fn format_hangman_word(hls : &Vec<HangmanLetter>) -> String {
    let mut word_chars = Vec::new();
    for hl in hls {
        if hl.found {
            word_chars.push(hl.letter);
        } else {
            word_chars.push('_');
        }
    }
    word_chars.into_iter().collect()
}

fn main() {
    println!("    Hangman in Rust   ");
    println!(" by Devon McKee, 2020 ");
    println!("----------------------");
    let mut path = String::new();
    println!("Path to words file: ");
    io::stdout().flush().expect("Flush failed!");
    io::stdin().read_line(&mut path).expect("Failed to read file path!");
    let words = match file_to_strings(path.trim().to_string()) {
        Ok(v) => v,
        Err(e) => panic!("Error reading file contents: {}", e),
    };
    let rand_word = words.choose(&mut rand::thread_rng()).unwrap();
    let mut lives = rand_word.len();
    let mut word_chars = rand_word.chars().map(|c| char_to_hangman_letter(c)).collect::<Vec<HangmanLetter>>();
    let mut guessed_letters = Vec::new();
    while word_found(&word_chars).not() && lives > 0 {
        println!("Word: {}", format_hangman_word(&word_chars));
        println!("Lives: {}", &lives);
        println!("Guessed: {}", &guessed_letters.iter().collect::<String>());
        let mut guess = String::new();
        io::stdout().flush().expect("Flush failed!");
        io::stdin().read_line(&mut guess).expect("Failed to read guess!");
        let trimmed_guess = guess.trim().to_string();
        let mut found_letter = false;
        if trimmed_guess.len() == 1 {
            let guess_char = trimmed_guess.chars().next().unwrap();
            for hl in &mut word_chars {
                if guess_char == hl.letter {
                    hl.found = true;
                    found_letter = true;                    
                }
            }
            if guessed_letters.contains(&guess_char).not() {
                
                if found_letter.not() {
                    guessed_letters.push(guess_char);
                    guessed_letters.sort();
                    println!("Letter not in word!");
                    lives = lives - 1;
                }
            }
            
        } else {
            println!("Guess a single letter for each word!");
        }
    }
    if word_found(&word_chars) {
        println!("Word found! The word was {}", rand_word);
    } else {
        println!("Ran out of lives! The word was {}", rand_word);
    }
}
