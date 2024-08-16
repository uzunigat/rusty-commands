#[cfg(test)]
mod tests {

    use super::super::super::r#move::*;
    use std::fs;
    use tempfile;

    #[test]
    fn test_move_file() {
        let from_path = tempfile::tempdir().unwrap();
        let to_path = tempfile::tempdir().unwrap();

        let test_file_path = from_path.path().join("test_file.txt");
        fs::write(&test_file_path, "test content").unwrap();

        let args = MoveArgs::new(
            test_file_path.to_str().unwrap().to_string(),
            to_path.path().to_str().unwrap().to_string(),
        );

        let result = move_file(args);

        match result {
            Ok(entries) => {
                assert!(entries.contains(&test_file_path.display().to_string()));
            }
            Err(e) => panic!("Unexpected error: {}", e),
        }
    }

    #[test]
    fn test_move_file_with_invalid_destination() {
        let args = MoveArgs::new(
            "invalid_filename".to_string(),
            "invalid_destination".to_string(),
        );

        let result = move_file(args);

        match result {
            Ok(_) => panic!("Expected an error"),
            Err(e) => assert_eq!(e, "File doesn't exists"),
        }
    }
}
