use crate::Colours::Red;

fn main() {
    let p1: Point<i32> = Point { x: 2, y: 3 };
    let p2: Point<f64> = Point { x: 2.2, y: 3.3 };

    println!("{:?}", p1);
    println!("{:?}", p2);

    let c1 = Red("#f00");
    let c2 = Red(255);

    println!("{:?}", c1);
    println!("{:?}", c2);

    let p3: Point2<i32, f64> = Point2 { x: 4, y: 6.6 };
    println!("{:?}", p3);
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
enum Colours<T> {
    Red(T),
    Blue(T),
    Green(T),
}

#[derive(Debug)]
struct Point2<T, V> {
    x: T,
    y: V,
}
