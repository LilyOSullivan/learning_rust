fn main() {
    let primes: Vec<i32> = Vec::new();
    let mut primes = vec![2, 3, 5, 7, 9, 11];
    println!("{:?}", primes);
    primes.push(13);
    println!("{:?}", primes);
    primes.remove(2);
    println!("{:?}", primes);

    const DEFAULT: i32 = 6;
    let mut numbers = [DEFAULT; 4];
    println!("{:?}", numbers);
    numbers[2] = 8;
    println!("{:?}", numbers);

    for number in numbers.iter() {
        println!("{}", number);
    }
}
