// compile using: $ rustc [filename].rs
// run with ./[filename.rs]
// sources:
// 1: https://doc.rust-lang.org/rust-by-example/std/str.html
// 2: https://rust-classes.com/basics_input.html
// 3: https://doc.rust-lang.org/book/ch03-02-data-types.html
// 4: https://stackoverflow.com/questions/54262976/how-do-i-print-stdout-and-get-stdin-on-the-same-line-in-rust

use std::io::{self, Write};


fn main() {
    println!("Hello, world!");
    print!("> ");

    let mut input = str_input();
    println!("your input was {}", input);
    
    

    let int_input = u32_input();
    println!("your input was {}", int_input);

    
}



fn str_input() -> String{
    let mut input = String::new();
    let _ = io::stdout().flush(); // allows input to be on the same line as a previous output [4]
    std::io::stdin() // accesses the keyboard
        .read_line(&mut input) // stores the input
        .expect("can not read user input"); //error handling

    input
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

    int
}

