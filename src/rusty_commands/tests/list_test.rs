#[cfg(test)]
mod tests {
    use super::super::super::list::*;
    use std::fs;
    use tempfile;

    #[test]
    fn test_list_files() {
        let dir = tempfile::tempdir().unwrap();
        let test_file_path = dir.path().join("test_file.txt");
        fs::write(&test_file_path, "test content").unwrap();

        let args = ListArgs::new(dir.path().to_str().unwrap().to_string());

        // Call the function
        let result = list_files(args);

        // Assert the result
        match result {
            Ok(entries) => {
                assert!(entries.contains(&test_file_path.display().to_string()));
            }
            Err(e) => panic!("Unexpected error: {}", e),
        }
    }

    #[test]
    fn test_list_files_with_invalid_directory() {
        let args = ListArgs::new("invalid_directory".to_string());

        let result = list_files(args);

        match result {
            Ok(_) => panic!("Expected an error"),
            Err(e) => assert_eq!(e, "Path does not exist or is not a directory"),
        }
    }
}
