fn main() {
    let square = |a: i32| a * a;
    apply(square, 6);

    // Calculate sum of all squares less than 500
    // Only for even numbers
    let limit = 500;
    let mut sum = 0;
    for i in 0.. {
        let isq = i * i;
        if isq > limit {
            break;
        } else {
            if is_even(isq) {
                sum += isq;
            }
        }
    }
    println!("Sum={}", sum);

    // With HOF
    let sum2 = (0..)
        .map(|x: i32| x * x)
        .take_while(|&x| x <= limit)
        .filter(|x| is_even(*x))
        .fold(0, |sum, x| sum + x);
    println!("With HOF Sum2={}", sum2);
}

fn is_even(x: i32) -> bool {
    x % 2 == 0
}

fn apply(f: fn(i32) -> i32, a: i32) {
    println!("Result {}", f(a));
}
