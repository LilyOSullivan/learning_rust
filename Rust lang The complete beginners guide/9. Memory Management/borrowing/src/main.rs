fn main() {
    let mut a = 6;
    let b = &mut a;
    println!("{}", *b);
    *b += 2;
    println!("{}", a);

    let mut v = vec![1, 2, 3, 4, 5];
    for i in &v {
        println!("{}", i);
    }
}
