#[derive(Debug)]
struct Person {
    name: String,
}

#[derive(Debug)]
struct Dog<'p> {
    name: String,
    owner: &'p Person,
}

impl Person {
    fn get_name(&self) -> &String {
        &self.name
    }
}

fn main() {
    let p1 = Person {
        name: String::from("Alex"),
    };
    let d1 = Dog {
        name: String::from("Doggie"),
        owner: &p1,
    };
    println!("{:?}", d1);

    let mut a: &String;
    {
        let p2 = Person {
            name: String::from("Mary"),
        };
        // a = p2.get_name();
        a = p1.get_name();
    }
    println!("{}", a);
}

fn get_str() -> &'static str {
    "Hello"
}
