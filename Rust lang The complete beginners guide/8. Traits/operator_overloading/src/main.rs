use std::ops::Add;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 1.3, y: 4.6 };
    let p2 = Point { x: 3.7, y: 1.4 };
    println!("{:?}", p1 + p2);
}
