#[cfg(test)]
mod tests {

    use super::super::super::copy::*;
    use std::fs;
    use tempfile;

    #[test]
    fn test_copy_file() {
        let source_directory = tempfile::tempdir().unwrap();
        let destination_directory = tempfile::tempdir().unwrap();

        let test_file_path = source_directory.path().join("test_file.txt");
        fs::write(&test_file_path, "test content").unwrap();

        let args = CopyArgs::new(
            test_file_path.to_str().unwrap().to_string(),
            destination_directory.path().to_str().unwrap().to_string(),
        );

        let result = copy_files(args);

        match result {
            Ok(r) => {
                assert_eq!(r, "test_file.txt");
            }
            Err(e) => panic!("Unexpected error: {}", e),
        }
    }
}
