// Reworking of DNA Most Common Substring for substrings of length k from minK to maxK in Rust
// Previous version in Python3
// Devon McKee, 2020
use std::io;
use std::io::Read;
use std::io::Write;
use std::fs::File;
use std::collections::HashMap;

fn file_to_string(path : String) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn most_common_substring(dna : &str, min_k: usize, max_k : usize) -> (String, usize) {
    let mut substrings : HashMap<String, usize> = HashMap::new();
    let dna_chars : Vec<char> = dna.chars().collect();
    for k in min_k..(max_k + 1) {
        for window in dna_chars.windows(k) {
            let window = window.iter().collect();
            let window_val = if substrings.contains_key(&window) { substrings[&window] } else { 0 };
            substrings.insert(window, 1 + window_val);
        }
    } 
    let mut max_substring = (String::from(""), 0);
    for (key, val) in substrings.iter() {
        if &max_substring.1 < val {
            max_substring = (key.to_string(), val.clone());
        }
    }
    max_substring
}

fn main() {
    println!("-- DNA Most Common Substring -- ");
    println!("  - Implementation in Rust - ");

    let mut path = String::new();
    print!("File Path: ");
    io::stdout().flush().expect("Flush failed!");
    io::stdin().read_line(&mut path).expect("Failed to read file path");
    let dna = match file_to_string(path.trim().to_string()) {
        Ok(v) => v,
        Err(e) => panic!("Error reading file contents: {}", e),
    };

    let mut min_k = String::new();
    print!("Min K Value: ");
    io::stdout().flush().expect("Flush failed!");
    io::stdin().read_line(&mut min_k).expect("Failed to read min K value!");
    let min_k : usize = match min_k.trim().parse() {
        Ok(n) => n,
        Err(_) => panic!("Failed to parse min K value!"),
    };

    let mut max_k = String::new();
    print!("Max K Value: ");
    io::stdout().flush().expect("Flush failed!");
    io::stdin().read_line(&mut max_k).expect("Failed to read max K value!");
    let max_k : usize = match max_k.trim().parse() {
        Ok(n) => n,
        Err(_) => panic!("Failed to parse max K value!"),
    };

    let (substring, count) = most_common_substring(&dna, min_k, max_k);
    println!("Most Common Substring from {} to {}: {} with count {}", &min_k , &max_k, &substring, &count);
}
