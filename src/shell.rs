use std::io::{stdout, Write};

use colored::Colorize;

pub fn shell_prompt() {
    let user = std::env::var("USER").expect("Error: Unable to find USER env var");
    let cwd = std::env::current_dir()
        .expect("Error: Unable to access CWD")
        .display()
        .to_string();
    let cwd_tokens: Vec<&str> = cwd.split("/").collect();
    let cwd_tail = cwd_tokens[cwd_tokens.len() - 1];

    print!("{} {} $ ", user.magenta(), cwd_tail.blue());
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
}
