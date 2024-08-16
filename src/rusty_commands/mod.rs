mod copy;
mod list;
mod r#move;
mod remove;

#[cfg(test)]
mod tests;

use clap::{Parser, Subcommand};
use copy::{copy_files, CopyArgs};
use list::{list_files, ListArgs};
use r#move::{move_file, MoveArgs};
use remove::{remove_file, RemoveArgs};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[derive(Debug)]
pub struct RustyCommands {
    #[command(subcommand)]
    cmd: Commands,
}

impl RustyCommands {
    pub fn new() -> Self {
        RustyCommands::parse()
    }

    pub fn run(&self) {
        match &self.cmd {
            Commands::Ls(args) => {
                let files = list_files(args.to_owned()).unwrap();
                for file in files {
                    println!("{}", file);
                }
            }
            Commands::Cp(args) => {
                let copied_file_name = copy_files(args.to_owned());
                match copied_file_name {
                    Ok(file_name) => println!("File copied successfully: {}", file_name),
                    Err(e) => eprintln!("{}", e),
                }
            }
            Commands::Rm(args) => {
                let removed_file_name = remove_file(args.to_owned());
                match removed_file_name {
                    Ok(file_name) => println!("File removed successfully: {}", file_name),
                    Err(e) => eprintln!("{}", e),
                }
            }
            Commands::Mv(args) => {
                let moved_dile_name = move_file(args.to_owned());
                match moved_dile_name {
                    Ok(file_name) => println!("File moved successfully: {}", file_name),
                    Err(e) => eprintln!("{}", e),
                }
            }
        }
    }
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    #[command(about = "List files in a directory")]
    Ls(ListArgs),
    #[command(about = "Copy files from one directory to another")]
    Cp(CopyArgs),
    #[command(about = "Move files from one directory to another")]
    Mv(MoveArgs),
    #[command(about = "Delete files in a directory")]
    Rm(RemoveArgs),
}
