/* use std::error::Error;
use std::fs;
use std::time::Instant; */
use std::{fs, time::Instant};

fn main() {
    let before = Instant::now();
    let bad_letters: [char; 8] = ['g', 'k', 'm', 'q', 'v', 'w', 'x', 'z'];
    let path = "files\\words.txt";
    let file: String = fs::read_to_string(path).unwrap();
    let word_list: Vec<String> = file.split("\n").map(|x| x.to_string().to_owned()).collect();
    let word_list = word_list
        .iter()
        .filter(|&x| !x.contains(bad_letters))
        .cloned()
        .collect::<Vec<String>>();
    let mut longest_words: Vec<String> = vec![];
    let mut max_len = 0;
    for word in &word_list {
        match word.len().cmp(&max_len) {
            std::cmp::Ordering::Greater => {
                max_len = word.len();
                longest_words.clear();
                longest_words.push(word.to_owned());
            }
            std::cmp::Ordering::Equal => {
                longest_words.push(word.to_owned());
            }
            _ => {}
        }
    }
    println!("{:?}", longest_words);
    println!("Elapsed: {:?}", before.elapsed());
}
