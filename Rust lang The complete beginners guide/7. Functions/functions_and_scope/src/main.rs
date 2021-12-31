static mut R: i32 = 0;

fn main() {
    {
        let a = 3;
        println!("{}", a);
    }
    // println!("{}", a);

    unsafe {
        R = 4;
        println!("R={}", R);
    }
}
