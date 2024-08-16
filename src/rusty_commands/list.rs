use clap::Parser;
use std::fs;
use std::path::Path;

#[derive(Parser, Debug, Clone)]
pub struct ListArgs {
    #[arg(value_name = "DIRECTORY")]
    directory: String,
}

#[allow(dead_code)]
impl ListArgs {
    pub fn new(directory: String) -> Self {
        ListArgs { directory }
    }
}

pub fn list_files(args: ListArgs) -> Result<Vec<String>, String> {
    let path = Path::new(&args.directory);

    if !path.exists() || !path.is_dir() {
        return Err("Path does not exist or is not a directory".to_string());
    }

    let mut results = Vec::new();
    let entries = fs::read_dir(path).map_err(|e| e.to_string())?;
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        results.push(path.display().to_string());
    }

    Ok(results)
}
