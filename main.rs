// sources:
// 1: https://docs.rs/rand/latest/rand/
// 2: https://rustjobs.dev/blog/string-concatenation-in-rust/#:~:text=In%20Rust%2C%20you%20can%20concatenate,proper%20types%20String%20and%20%26str%20.&text=In%20this%20example%2C%20we%20first,to%20it%20using%20the%20%2B%20operator%20.
// 3: https://stackoverflow.com/questions/33405672/how-can-i-convert-a-one-element-string-into-a-char
// 4: https://stackoverflow.com/questions/30154541/how-do-i-concatenate-strings

use std::io::{self, Write}; // needed to make flush work
use std::fmt::Display;
// use rand::prelude::*;


fn main() {

    let words : [&str;7] = ["Acting", "Appear", "Charge", "Copper", "Behind", "Either", "Double"];


    let mut running = true;
    while running {
        println!{"\n Hangman Menu: \n 1: to play the game \n 2: Exit program"}

        print!("> ");
        let mut input = u32_input();

        // menu to pick your option
        match input{

        1 => {
            // println!("game start");
            game_loop(words);
        
        }

        // ends program
        2 => {running = false}

        // 3 => {
        //     println!("input 1 - 6");
        //     let input = u32_input();
        //     output_hangman(input);
        // }

        _ => {println!("Error: unknown option")}

        }

    }

}

fn game_loop(words: [&str;7]){
    let mut word = pick_word(words); 
    
    println!();

    let mut correct_guesses = [false, false, false, false, false, false];
    let mut incorrect_guesses = String::new();
    
    let mut game_over = false;

    while !game_over {
        let mut guess = str_input();

        compare_guess(guess, word.clone(), &mut correct_guesses);
        output_word(&word, &correct_guesses)
    }


}

fn compare_guess( guess: String, word: String, mut correct_guesses: &mut [bool;6]) {
    let mut count = 0;
    let char_guess = guess.chars().next().expect("string is empty");                                        // source 3

    for i in word.chars(){
        // compares the guessed letter to all the letters in the 
        if char_guess == i {
            correct_guesses[count] = true
        }
        count += 1;

    }

    // return correct_guesses;
    
}

fn output_word(word:&String, correct_guesses:&[bool;6]){
    let mut ind_cnt = 0;
    for i in word.chars(){
        if correct_guesses[ind_cnt]{
            print!("{}",i)
        }
        else{
            print!("_")
        }
        ind_cnt+=1


    }

}

fn pick_word(words: [&str;7]) -> String {
    let rand_int = 2;
    // let mut rng = rand::thread_rng();
    // let rand_int: u32 = rng.gen_range(0..=words.len());

    return words[rand_int].to_string();
}

fn output_hangman(fail:u32) {
    
    match fail {
        1 => {
            println!("    +---+");
            println!("    |   |");
            println!("        |");
            println!("        |");
            println!("        |");
            println!("        |");
            println!("  =======");
        }
        2 => {
            println!("    +---+");
            println!("    |   |");
            println!("    O   |");
            println!("        |");
            println!("        |");
            println!("        |");
            println!("  =======");
        }
        3 => {
            println!("    +---+");
            println!("    |   |");
            println!("    O   |");
            println!("    |   |");
            println!("        |");
            println!("        |");
            println!("  =======");
        }
        4 => {
            println!("    +---+");
            println!("    |   |");
            println!("    O   |");
            println!("   /|   |");
            println!("        |");
            println!("        |");
            println!("  =======");
        }
        5 => {
            println!("    +---+");
            println!("    |   |");
            println!("    O   |");
            println!("   /|\\  |");
            println!("        |");
            println!("        |");
            println!("  =======");
        }
        6 => {
            println!("    +---+");
            println!("    |   |");
            println!("    O   |");
            println!("   /|\\  |");
            println!("   / \\  |");
            println!("        |");
            println!("  =======");
        }
        _ => println!("Error: out of bounds")

    }

}

fn str_input() -> String{
    let mut input = String::new();
    let _ = io::stdout().flush(); // allows input to be on the same line as a previous output [4]

    std::io::stdin() // accesses the keyboard
        .read_line(&mut input) // stores the input
        .expect("can not read user input"); //error handling

    return input
}

fn u32_input() -> u32{
    let mut int = 0;
    let mut input = String::new();
    let _ = io::stdout().flush(); // allows input to be on the same line as a previous output [4]

    std::io::stdin() // accesses the keyboard
        .read_line(&mut input) // stores the input
        .expect("can not read user input"); //error handling

        let trimmed = input.trim(); // trimms the extra spaces
        match trimmed.parse::<u32>() {// parses the str into an int
            Ok(i) => int = i, 

            Err(..) => println!("this was not an integer: {}", trimmed), //error handling
    }

    return int
}
