use colored::Colorize;

pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn help() {
    let help_heading = "ARMSH help";
    println!("{}", help_heading.on_bright_green().black());
    println!("Available commands: clear, ...");
}

pub fn exit() {
    std::process::exit(0x0100);
}

pub fn pwd() {
    let current_dir = std::env::current_dir();
    match current_dir {
        Ok(path) => println!("{}", path.display()),
        Err(_) => println!("Unable to fetch present working directory."),
    }
}
