macro_rules! my_macro {
    () => {
        println!("First Macro")
    };
}

// macro_rules! name {
//     ($name: expr) => {
//         println!("Hey {}", $name)
//     };
// }

macro_rules! name {
    ($($name:expr),*) => ( $(println!("Hey {}",$name);)*)
}

macro_rules! xy {
    (x => $e:expr) => {
        println!("X is {}", $e)
    };
    (y => $e:expr) => {
        println!("Y is {}", $e)
    };
}

macro_rules! build_fn {
    ($fn_name:ident) => {
        fn $fn_name() {
            println!("{:?} was called", stringify!($fn_name));
        }
    };
}

fn main() {
    my_macro!();
    name!("Sarah");
    name!("Sarah", "Alex", "Steph");

    xy!(x => 5);
    xy!(y => 15*2);

    build_fn!(hey);
    hey();
}
