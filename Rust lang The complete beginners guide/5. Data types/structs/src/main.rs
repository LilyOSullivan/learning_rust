fn main() {
    let emp = Employee {
        name: String::from("John"),
        company: String::from("Pear"),
        age: 51,
    };

    println!("{:?}", emp);
    println!("{}", emp.name);
    println!("{}", emp.is_adult());
    println!("{}", emp.details());
    println!("{}", Employee::static_details());
}

#[derive(Debug)]
struct Employee {
    name: String,
    company: String,
    age: u32,
}

impl Employee {
    fn details(&self) -> String {
        format!(
            "name: {}, employed: {}, age: {}",
            &self.name, &self.company, &self.age
        )
    }

    fn is_adult(&self) -> bool {
        self.age >= 18
    }

    fn static_details() -> String {
        String::from("Details of a person")
    }
}
