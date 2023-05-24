pub fn cd(command_options: Vec<&str>) {
    // temporary code without flags
    let path = *(command_options.get(0).unwrap_or(&""));
    let chdir = std::env::set_current_dir(std::path::Path::new(path));
    match chdir {
        Ok(_) => {}
        Err(_) => println!("cd: no such directory named {}", path),
    };
}
