/*
By: <aum markandey>
Date: 2026-10-02
Program Details: magic eightball
*/


use std::io;
use rand::Rng;
fn main() {
     println!("Hello, please input your question for the magic eightball, to stop simply type 'exit' and press enter");
    loop{
     println!("Please input your question: ");  
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut rng = rand::rng();
    let dice: i32 = rng.random_range(1..=8);
    let ans1 = "Yes";
    let ans2 = "No";
    let ans3 = "Maybe";
    let ans4 = "Ask again later";
    let ans5 = "Definitely";
    let ans6 = "I don't think so";
    let ans7 = "signs point to yes";
    let ans8 = "My sources say no";
    if input.trim().to_lowercase() == "exit" {
        break;
    }
    match dice {
        1 => println!("{}", ans1),
        2 => println!("{}", ans2),
        3 => println!("{}", ans3),
        4 => println!("{}", ans4),
        5 => println!("{}", ans5),
        6 => println!("{}", ans6),
        7 => println!("{}", ans7),
        8 => println!("{}", ans8),
        _ => println!("Error"),
    }
    }
    
}
