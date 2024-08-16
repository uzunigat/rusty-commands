use clap::Parser;
use std::path::Path;

#[derive(Parser, Debug, Clone)]
pub struct CopyArgs {
    #[arg(value_name = "FILE_NAME")]
    file_path: String,
    #[arg(value_name = "TO_DIRECTORY")]
    to: String,
}

#[allow(dead_code)]
impl CopyArgs {
    pub fn new(file_path: String, to: String) -> Self {
        CopyArgs { file_path, to }
    }
}

pub fn copy_files(args: CopyArgs) -> Result<String, String> {
    let to_directory = Path::new(&args.to);
    let file_path = Path::new(&args.file_path);

    if !file_path.exists() || !file_path.is_file() {
        return Err("File doesn't exists".to_string());
    }

    if !to_directory.exists() || !to_directory.is_dir() {
        return Err("Destination directory doesn't exists".to_string());
    }

    let file_name = file_path.file_name().unwrap();
    let to_path = to_directory.join(file_name);

    match std::fs::copy(file_path, to_path) {
        Ok(_) => Ok(file_name.to_str().unwrap().to_string()),
        Err(e) => Err(format!("Error while copying the file {}", e)),
    }
}
