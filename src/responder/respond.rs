use crate::logger::{Message, Log};
use crate::analyzer::tone::analyze;

pub fn respond(msg: String){    
    let mode = analyze(msg);
    Message::log("Your message is recorded");
}