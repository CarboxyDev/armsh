use crate::command::*;
use colored::Colorize;
use std::io::{stdout, Write};

pub fn shell_prompt() {
    let cwd = std::env::current_dir()
        .expect("Error: Unable to access CWD")
        .display()
        .to_string();
    let cwd_tokens: Vec<&str> = cwd.split("/").collect();
    let cwd_tail = cwd_tokens[cwd_tokens.len() - 1];
    let shell_badge = "~>";
    let command_start = "$";

    print!(
        "{} {} {} ",
        shell_badge.red(),
        cwd_tail.blue(),
        command_start.blue(),
    );
}

pub fn shell_input() {
    let mut input = String::new();
    let _ = stdout().flush();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error: Unable to take input from user");

    let input = input
        .strip_suffix("\n")
        .expect("Error: Failed to parse the input");

    match input {
        "clear" => basic::clear(),
        _ => {
            // TODO: Incorporate into some sort of Error handling system. Kind of like error::unknown_command()
            println!("armsh: command not found");
        }
    }
}
