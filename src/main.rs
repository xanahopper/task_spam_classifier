extern crate spam_classifier;

use std::env;

fn main() {
    let mut args = env::args();
    args.next();
    let dir_path = match args.next() {
        Some(path) => path,
        None => env::current_dir().unwrap().join("data/20_newsgroups/").to_str().unwrap().to_owned()
    };
    let files = spam_classifier::get_all_file(&dir_path);
    for (key, _list) in files {
        println!("{:?}", key);
    }
}
