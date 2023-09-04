use logger::{Message, Log};
mod logger;
mod analyzer;
use std::io::{self};
mod responder;
use crate::responder::respond::respond;

fn user_input_func() -> String {
    let mut user_input = String::new();
    let stdin = io::stdin(); 
    let stats = stdin.read_line(&mut user_input);
    if stats.is_err() {
        Message::log("error occured")
    };
    return user_input;
}

fn main() {
    Message::log("Welcome to Chat. Please enter your message");    
    loop {
        let input_string= user_input_func();
        if input_string.eq("exit") {
            Message::log("Thank you for using chat");
            break;
        };
        respond(input_string);
    }
}
