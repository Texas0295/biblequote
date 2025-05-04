use std::fs;
use std::path::PathBuf;
use std::io;

use dirs::data_dir;

const DEFAULT_VERSES:&str = r"
For God so loved the world, that he gave his only Son, that whoever believes in him should not perish but have eternal life. (John 3:16)
The Lord is my shepherd; I shall not want. (Psalm 23:1)
I can do all things through Christ who strengthens me. (Philippians 4:13)
Trust in the Lord with all your heart, and do not lean on your own understanding. (Proverbs 3:5)
For where two or three are gathered in my name, there am I among them. (Matthew 18:20)
Be still, and know that I am God. (Psalm 46:10)
The steadfast love of the Lord never ceases; his mercies never come to an end; they are new every morning; great is your faithfulness. (Lamentations 3:22-23)
For all have sinned and fall short of the glory of God. (Romans 3:23)
But the fruit of the Spirit is love, joy, peace, patience, kindness, goodness, faithfulness, gentleness, self-control; against such things there is no law. (Galatians 5:22-23)
Do not be anxious about anything, but in everything by prayer and supplication with thanksgiving let your requests be made known to God. (Philippians 4:6)
";

pub fn get_data_dir_opt() -> Option<String> {
    let mut path = data_dir()?;
    path.push("biblequote");

    if let Err(e) = fs::create_dir_all(&path) {
        eprintln!("[ERROR] Failed to create data dir {:?}: {}", path, e);
        return None;
    }

    path.to_str().map(|s| s.to_owned() + "/")
}

pub fn read_verses(path_str: String) -> Result<Vec<String>, io::Error> {
    let path = PathBuf::from(path_str);

    if path.exists() {
        return fs::read_to_string(path)
            .map(|content| content.lines().map(|s| s.to_string()).collect());
    }

    match fs::write(&path, DEFAULT_VERSES) {
        Ok(_) => {
            println!("[INFO] Verses file not found. Created default file at: {}", path.display());
            fs::read_to_string(path)
                .map(|content| content.lines().map(|s| s.to_string()).collect())
        }
        Err(e) => {
            eprintln!("[ERROR] Failed to create default verses file at {:?}: {}", path, e);
            Err(io::Error::new(io::ErrorKind::Other, "Failed to create default verses file"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn read_file() {
        let result = read_verses("test_resources/verses.txt".to_string());
        assert!(result.is_ok());
        let verses = result.unwrap();
        assert!(!verses.is_empty());
    }

    #[test]
    fn read_empty_file() {
        let result = read_verses("test_resources/verses_empty.txt".to_string());
        assert!(result.is_ok());
        let verses = result.unwrap();
        assert!(verses.is_empty());
    }

    #[test]
    fn read_file_not_exsist() {
        let file_path = "non_existent.txt".to_string();
        let result = read_verses(file_path.clone());
        assert!(result.is_ok());
        let verses = result.unwrap();
        assert!(!verses.is_empty());
        assert_eq!(verses.len(), DEFAULT_VERSES.lines().count());

        fs::remove_file(file_path).unwrap_or_else(|e| eprintln!("Failed to remove test file: {}", e));
    }
}
