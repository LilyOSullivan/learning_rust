use std::fs::File;

fn main() {
    // let v = vec![1, 2, 3];
    // v[10];

    let f = File::open("main.jpg");
    match f {
        Ok(f) => {
            println!("File Found {:?}", f);
        }
        Err(e) => {
            println!("File not found \n {:?}", e);
        }
    }

    println!("Continuing Execution");
    divide(Some(1));
    divide(Some(10));
    divide(None);
    divide(Some(0));
}

const ANSWER_TO_LIFE: i32 = 42;

fn divide(x: Option<i32>) {
    match x {
        Some(0) => panic!("Cannot Divide by zero"),
        Some(n) => println!("Result is {}", ANSWER_TO_LIFE / n),
        None => println!("None received, the answer is {}", ANSWER_TO_LIFE),
    }
}
