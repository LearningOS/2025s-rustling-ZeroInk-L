// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.

// I AM DONE HERE
/// `my_macro` 是一个简单的宏，当被调用时，它会在控制台输出 "Check out my macro!"。
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}


fn main() {
    my_macro!();
}


