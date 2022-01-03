use std::fs::remove_file;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

fn main() {
    File::create("src/name.txt").expect("create failed");
    // file.write_all("Hello World!\n".as_bytes()).expect("Write failed");
    // let mut file = OpenOptions::new()
    //     .append(true)
    //     .open("src/name.txt")
    //     .expect("Cannot open file");
    // file.write_all("Adding content to the file.\n".as_bytes()).expect("Write failed");

    let mut file = File::open("src/name.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    println!("{}", contents);
    remove_file("src/name.txt").expect("Delete Failed");
}
