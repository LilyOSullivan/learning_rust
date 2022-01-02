trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut sum: i32 = 0;
        for i in self {
            sum += *i;
        }
        sum
    }
}

fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];
    println!("Sum = {}", v.sum());

    // let v2 = vec![1.0, 2.0, 3.0];
    // println!("Sum = {}", v2.sum());
}
