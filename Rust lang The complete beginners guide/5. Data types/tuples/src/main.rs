fn main() {
    let mut person: (&str, i32, bool) = ("Alex", 27, true);
    println!("{:?}", person);

    println!("{}", person.0);

    person.0 = "Maria";
    println!("{}", person.0);

    let (name,age,employed) = person;
    println!("name={}, age={}, employed={}",name,age,employed);
}
