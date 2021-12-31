fn main() {
    let a = |a: i32| a + 1;
    println!("{}", a(6));

    let b = |b: i32| -> i32 {
        let c = b + 1;
        c
    };
    println!("{}", b(4));

    let gen = |x| println!("Received {}", x);
    gen(3);
    // gen(true);
}
