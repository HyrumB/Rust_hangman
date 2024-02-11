

// // sources:
// // 1: https://doc.rust-lang.org/rust-by-example/flow_control/match.html
// // 2: https://www.reddit.com/r/rust/comments/i8qzug/how_to_call_functions_from_other_files/
// // 3:

// pub mod get_input; //[2]
// use get_input::u32_input; //[2]

// fn main() {

//     let mut running = true;
//     while running {
//         println!{"Menu: \n 1: to print one \n 2: end"}

//         print!("> ");
//         let mut input = u32_input();
//         match input{

//         1 => println!("one"),
//         // count +=1;
//         // println!("{}", count);\
//         2 => running = false,

//         _ => println!("Error: unknown option"),

//         }
//     }

// }