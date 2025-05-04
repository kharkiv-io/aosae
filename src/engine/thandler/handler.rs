use crate::engine::core::core;
use std::io;
use std::string;
use symphonia::core::conv::IntoSample;

pub struct SplitBuilder {
    command: String,
    argument: Vec<String>,
}

pub fn parse_command_args(command: String) -> SplitBuilder {
    let splits: Vec<&str> = command.split_whitespace().collect();
    let mut command = String::new();
    let mut argument = Vec::new();
    let vec_len = splits.len();
    command = splits[0].to_string();
    for i in 1..vec_len {
        argument.push(splits[i].to_string());
    }
    SplitBuilder { command, argument }
}

pub fn handler() {
    loop {
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Can't read command to buf?");
        let command = command.trim();
        let parsed = parse_command_args(command.to_owned());
        match parsed {
            _ if parsed.command == "play" => {
                if parsed.argument.len() == 2 {
                    if parsed.argument[0] == "--from-audio" {}
                }
            }
            _ => {
                println!(".");
            }
        }
    }
}
