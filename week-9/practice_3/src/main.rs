use std::fs::remove_file;

fn main() {
    remove_file("../practice_1/data.txt").expect("couldn't remove file");
    println!("file is rwmoved");
}
