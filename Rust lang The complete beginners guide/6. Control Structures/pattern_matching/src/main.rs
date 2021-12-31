fn main() {
    for i in 0..15 {
        println!("{}. I have {} oranges", i, get_oranges(i));
    }

    let point = (0, 0);
    match point {
        (0, 0) => println!("Origin"),
        (x, 0) => println!("X axis ({},0)", x),
        (0, y) => println!("Y axis (0,{})", y),
        (x, y) => println!("({},{})", x, y),
    }
}

fn get_oranges(amount: i32) -> &'static str {
    return match amount {
        0 => "no",
        1 | 2 => "one or two",
        3..=7 => "a few",
        _ if (amount % 2 == 0) => "an even amount of",
        _ => "lots of",
    };
}
