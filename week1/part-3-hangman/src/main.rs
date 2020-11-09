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
use std::collections::HashSet;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn get_input(prompt: &str) ->  char {
    print!("{}", prompt);
    //flush stdout
    io::stdout()
        .flush()
        .expect("Error flushing stdout.");
    
    //get input
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading stdin.");
    
    //return the input
    let guess_vec: Vec<char> = input.chars().collect();
    return guess_vec[0];

    
}

fn word_so_far(secret_word_chars: &Vec<char>, guesses: &HashSet<char>) {
    for i in 0..secret_word_chars.len() {
        if guesses.contains(&secret_word_chars[i]) {
            print!("{}", secret_word_chars[i]);
        } else {
            print!("-");
        }
    }

    print!("\n");
    //flush word to stdout
    io::stdout()
        .flush()
        .expect("Error flushing stdout: word_so_far");

    print!("You have guessed the following letters: ");
    for elem in guesses.iter() {
        print!("{}", elem);
    }
    print!("\n");
    //flush word to stdout
    io::stdout()
        .flush()
        .expect("Error flushing stdout: word_so_far");
}

fn result(guess: &char, secret_word: &Vec<char>) -> bool {
    //print!("{}, {}", secret_word[0], *guess);
    for index in 0..secret_word.len() {
        if secret_word[index] == *guess {
            return true
        }
    }
    return false
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);

    // Your code here! :)
    println!("Welcome to CS110L Hangman!");
    let mut guesses: HashSet<char> = HashSet::new();
    let mut num_inccorect = 0;
    while  num_inccorect <= NUM_INCORRECT_GUESSES {
        //print word so far
        word_so_far(&secret_word_chars, &guesses);
        println!("You have {} guesses left", NUM_INCORRECT_GUESSES - num_inccorect);
        let guess = get_input("Please guess a letter: ");

        if !result(&guess, &secret_word_chars) {
            //made incorrect guess
            num_inccorect += 1;
            println!("Sorry thats not in the word.");
            
        } else {
            println!("Correct!");
        }
        guesses.insert(guess);
    }
}
