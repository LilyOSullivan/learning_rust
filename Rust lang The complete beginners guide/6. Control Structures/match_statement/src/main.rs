use crate::Suit::{Club, Diamond, Heart, Spade};

fn main() {
    print_choice(Suit::Heart);
    print_choice(Suit::Spade);
    print_choice(Suit::Club);
    print_choice(Suit::Diamond);

    country(44);
    country(34);
    country(345);
    country(-20);
}

fn country(code: i32) {
    let country = match code {
        44 => "UK",
        34 => "Spain",
        1..=999 => "unknown",
        _ => "Invalid",
    };

    println!("Country is {}", country);
}

enum Suit {
    Heart,
    Spade,
    Club,
    Diamond,
}

fn print_choice(choice: Suit) {
    match choice {
        Heart => {
            println!("\u{2665}")
        }
        Spade => {
            println!("\u{2660}")
        }
        Club => {
            println!("\u{2663}")
        }
        Diamond => {
            println!("\u{2666}")
        }
    }
}
