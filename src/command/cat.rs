pub fn cat(command_options: Vec<&str>) {
    // TODO: Implement cat for multiple files in the future
    let file_name = *(command_options.get(0).unwrap_or(&""));
    let file_content = std::fs::read_to_string(file_name);
    match file_content {
        Ok(content) => println!("{}", content),
        Err(_) => println!("cat: unable to find file {}", file_name),
    }
}
