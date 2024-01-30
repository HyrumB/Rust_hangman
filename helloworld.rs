// compile using: $ rustc [filename].rs
// run with ./[filename.rs]


fn main() {
    println!("Hello, world!");
    print!("> ");
}

fn input() -> String{
    let mut input = String::new();
    std::io::stdin() // accesses the keyboard
        .read_line(&mut input) // stores the input
        .expect("can not read user input"); //error handling

    println!("your input was {}", input);
}