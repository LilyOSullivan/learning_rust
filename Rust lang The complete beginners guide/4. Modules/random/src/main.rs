use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn main() {
    let mut rng = rand::thread_rng();
    let i: i32 = rng.gen();
    println!("{}", i);

    println!("Bounded int {}", rng.gen_range(0, 2));
    println!("Bounded float {}", rng.gen_range(0.0, 100.0));

    let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();

    println!("Gen string {}", rand_string);
}
