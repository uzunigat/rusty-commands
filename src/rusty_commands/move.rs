use clap::Parser;
use std::path::Path;

#[derive(Debug, Parser, Clone)]
pub struct MoveArgs {
    #[arg(value_name = "FILE_PATH")]
    file_path: String,
    #[arg(value_name = "TO_DIRECTORY")]
    to: String,
}

#[allow(dead_code)]
impl MoveArgs {
    pub fn new(file_path: String, to: String) -> Self {
        MoveArgs { file_path, to }
    }
}

pub fn move_file(args: MoveArgs) -> Result<String, String> {
    let file_path = Path::new(&args.file_path);
    let to_directory = Path::new(&args.to);

    if !file_path.exists() || !file_path.is_file() {
        return Err("File doesn't exists".to_string());
    }

    if !to_directory.exists() || !to_directory.is_dir() {
        return Err("Destination directory doesn't exists".to_string());
    }

    match std::fs::rename(file_path, to_directory.join(file_path.file_name().unwrap())) {
        Ok(_) => Ok(file_path.display().to_string()),
        Err(e) => Err(format!("Error while moving the file {}", e)),
    }
}
