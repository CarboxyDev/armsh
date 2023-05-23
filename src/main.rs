mod command;
mod shell;

fn main() {
    loop {
        shell::shell_prompt();
        shell::shell_input();
    }
}
