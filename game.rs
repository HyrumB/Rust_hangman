pub mod get_input; //[2]
use get_input::u32_input;
use get_input::str_input;

fn main() {

    let mut running = true;
    while running {
        println!{"Hangman Menu: \n 1: to play the game \n 2: Exit program"}

        print!("> ");
        let mut input = u32_input();
        match input{

        1 => println!("game start"),
            
        2 => running = false,

        3 => {
            println!("input 1 - 6");
            let input = u32_input();
            output_hangman(input);
        }

        _ => println!("Error: unknown option"),

        }
    }

}


// fn output_underscores() {

// }

// fn pick_word(words: [u32]) {

// }

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