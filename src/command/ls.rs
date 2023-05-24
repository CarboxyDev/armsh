pub fn ls() {
    let read_dir = std::fs::read_dir("./").expect("Unable to read the current working directory");

    let mut i: u32 = 0;
    for file in read_dir {
        i += 1;
        print!(
            "{}    ",
            file.unwrap()
                .path()
                .display()
                .to_string()
                .strip_prefix("./")
                .expect("Error: Failed to output file")
        );

        if i % 4 == 0 {
            println!("");
        }
    }

    if i % 4 != 0 {
        println!("");
    }
}
