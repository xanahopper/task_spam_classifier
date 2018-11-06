use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub fn say(what: &str) -> String {
    format!("you say: {:?}", what)
}

pub fn get_all_file(dir_path: &str) -> HashMap<PathBuf, Vec<PathBuf>> {
    let mut map = HashMap::new();
    for d in fs::read_dir(dir_path).unwrap() {
        let dir = d.unwrap();
        let dir_name = dir.path();
        let files = fs::read_dir(dir.path()).unwrap()
            .map(|e| e.unwrap().path())
            .collect::<Vec<PathBuf>>();
        map.insert(dir_name, files);
    }
    map
}