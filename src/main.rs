extern crate spam_classifier;

fn main() {
    let files = spam_classifier::get_all_file("/Users/xana/Dev/data/20_newsgroups/");
    println!("{:?}", files);
}
