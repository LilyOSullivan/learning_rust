struct RustDev {
    awesome: bool,
}

struct JavaDev {
    awesome: bool,
}

trait Developer {
    fn new(awesome: bool) -> Self;
    fn language(&self) -> &str;
    fn say_hello(&self) {
        println!("Hello World!")
    }
}

impl Developer for RustDev {
    fn new(awesome: bool) -> Self {
        RustDev { awesome: awesome }
    }

    fn language(&self) -> &str {
        "Rust"
    }

    fn say_hello(&self) {
        println!("println!(\"Hello World\");");
    }
}

impl Developer for JavaDev {
    fn new(awesome: bool) -> Self {
        JavaDev { awesome: awesome }
    }

    fn language(&self) -> &str {
        "Java 1.8"
    }

    fn say_hello(&self) {
        println!("System.out.println(\"Hello World!\");");
    }
}

fn main() {
    // let r = RustDev { awesome: true };
    let r = RustDev::new(true);
    let j = JavaDev::new(false);

    println!("{}", r.language());
    r.say_hello();

    println!("{}", j.language());
    j.say_hello();
}
