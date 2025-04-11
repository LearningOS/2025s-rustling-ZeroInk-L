// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.

// I AM DONE HERE

macro_rules! print_macro_demo {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    print_macro_demo!();
}
