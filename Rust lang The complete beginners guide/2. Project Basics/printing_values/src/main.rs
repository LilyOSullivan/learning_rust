fn main() {
    println!("Hello, world!");
    println!("My name is {} and I'm {} years old", "Lily", 15);
    println!("a + b = {}", 3 + 9);
    println!("{0} has a {2} and {0} has a {1}", "Lily", "cat", "dog");
    println!("{name} {surname}", surname = "O'Sullivan", name = "Lily");
    println!("binary: {:b}, Hex: {:x}, Octal: {:o}", 50, 50, 50);
    println!("Array: {:?}", [1, 2, 3]);
}
