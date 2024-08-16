mod rusty_commands;
use rusty_commands::RustyCommands;

fn main() {
    let args = RustyCommands::new();
    args.run();
}
