fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let slice = &numbers[1..4];
    println!("{:?}", slice);

    let mut colours = ["red", "green", "blue", "pink"];
    update_colours(&mut colours[2..4]);
    println!("{:?}", colours);
}

fn update_colours(colours_slice: &mut [&str]) {
    colours_slice[0] = "yellow";
    colours_slice[1] = "orange";
}
