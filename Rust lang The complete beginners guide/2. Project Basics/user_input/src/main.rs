use std::io;

/// Crate comment.
/// What is this module trying to achieve.

fn main() {
    //! # Main heading
    //! ```
    //! fn main()
    //! ```
    //!
    //! Reads user input and prints it to the console

    let mut input = String::new();

    // Print a message to the user
    println!("Say something");

    /*
        Check response and
         act accordingly
    */
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("You said {}", input);
        }
        Err(e) => {
            println!("Something went wrong {}", e);
        }
    }
}
