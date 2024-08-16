use clap::Parser;
use std::path::Path;

#[derive(Parser, Debug, Clone)]
pub struct RemoveArgs {
    #[arg(value_name = "FILE_PATH")]
    file_path: String,
}

#[allow(dead_code)]
impl RemoveArgs {
    pub fn new(file_path: String) -> Self {
        RemoveArgs { file_path }
    }
}

pub fn remove_file(args: RemoveArgs) -> Result<String, String> {
    let file_to_remove = Path::new(&args.file_path);

    if !file_to_remove.exists() || !file_to_remove.is_file() {
        return Err("File doesn't exists".to_string());
    }

    match std::fs::remove_file(file_to_remove) {
        Ok(_) => Ok(file_to_remove.display().to_string()),
        Err(e) => Err(format!("Error while removing the file {}", e)),
    }
}
