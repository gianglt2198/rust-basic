use rusty_kv::Database;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let test_path = "integration_test.json";

        let _ = fs::remove_file(test_path);

        {
            let mut db = Database::new(test_path);
            db.insert("name".to_string(), "gemini".to_string());
            db.insert("age".to_string(), "18".to_string());
            db.insert("address".to_string(), "google".to_string());
            db.save().expect("Failed to save");
        }

        let db = Database::load(test_path).expect("Failed to load");
        assert_eq!(db.get("name"), Some(&"gemini".to_string()));
        assert_eq!(db.get("non_existent"), None);

        // Clean up
        fs::remove_file(test_path).unwrap();
    }
}
