fn main() {
    let cat: &'static str = "Fluffy";
    println!("{}", cat);

    let mut dog = String::from("Max");
    println!("{}", dog);

    let owner = format!("Hi I'm {} the owner of {}", "Lily", dog);
    println!("{}", owner);

    println!("{}", dog.len());

    dog.push(' ');
    dog.push_str("the dog");
    println!("{}", dog);

    let new_dog = dog.replace("the", "is my");
    println!("{}", new_dog);
}
