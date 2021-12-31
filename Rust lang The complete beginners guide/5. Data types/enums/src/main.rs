use crate::Colour::Blue;
use crate::Person::Name;

fn main() {
    let my_colour = Colour::Red;
    println!("{:?}", my_colour);
    let my_color = Blue;

    let person = Name(String::from("Alex"));
    println!("{:?}", person);
}

#[derive(Debug)]
enum Colour {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
enum Person {
    Name(String),
    Surname(String),
    Age(u32),
}
