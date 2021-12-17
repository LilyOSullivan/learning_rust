mod archive;
// use crate::archive::arch::arch_file;
use crate::archive::arch::arch_file as arc;

use rand::Rng;

fn main() {
    arc("some_text.txt");

    let mut rng = rand::thread_rng();
    let a: i32 = rng.gen();

    println!("{}", a);
}
