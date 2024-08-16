#[cfg(test)]
mod tests {
    use super::super::super::remove::*;
    use std::fs;
    use tempfile;

    #[test]
    fn test_remove_file() {
        let dir = tempfile::tempdir().unwrap();
        let test_file_path = dir.path().join("test_file.txt");
        fs::write(&test_file_path, "test content").unwrap();

        let args = RemoveArgs::new(
            dir.path()
                .join(&test_file_path)
                .to_str()
                .unwrap()
                .to_string(),
        );

        let result = remove_file(args);

        match result {
            Ok(entries) => {
                assert!(entries.contains(&test_file_path.display().to_string()));
            }
            Err(e) => panic!("Unexpected error: {}", e),
        }
    }

    #[test]
    fn test_remove_file_with_invalid_filename() {
        let args = RemoveArgs::new("invalid_directory".to_string());

        let result = remove_file(args);

        match result {
            Ok(_) => panic!("Expected an error"),
            Err(e) => assert_eq!(e, "File doesn't exists"),
        }
    }
}
