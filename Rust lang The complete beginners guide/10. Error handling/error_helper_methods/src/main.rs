use std::fs::File;

fn main() {
    // let f = File::open("main.jpg").unwrap();
    let f = File::open("main.jpg").expect("Main.jpg is not found");
}
