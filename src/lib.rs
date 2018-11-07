use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub fn say(what: &str) -> String {
    format!("you say: {:?}", what)
}

pub fn get_all_file(dir_path: &str) -> HashMap<PathBuf, Vec<PathBuf>> {
    let mut map = HashMap::new();
    if let Ok(mut dirs) = fs::read_dir(dir_path) {
        while let Some(Ok(dir)) = dirs.next() {
            let dir_name = dir.path();
            if let Ok(files) = fs::read_dir(&dir_name) {
                let file_list = files
                    .filter(|e| e.is_ok())
                    .map(|e| e.unwrap().path())
                    .collect();
                map.insert(dir_name, file_list);
            }
        }
    }
    map
}
