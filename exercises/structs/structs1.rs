// structs1.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a
// hint.

// I AM DONE HERE

struct ColorClassicStruct {
    // DONE: Something goes here
    red: u8,
    green: u8,
    blue: u8,
}

impl ColorClassicStruct {
    // DONE: Implement a method for this struct!
    fn new(red: u8, green: u8, blue: u8) -> ColorClassicStruct {
        ColorClassicStruct { red, green, blue }
    }
}

struct ColorTupleStruct(
    u8,
    u8,
    u8,
);

#[derive(Debug)]
struct UnitLikeStruct;

impl UnitLikeStruct {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // DONE: Instantiate a classic c struct!
        let green = ColorClassicStruct { 
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // DONE: Instantiate a tuple struct!
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // DONE: Instantiate a unit-like struct!
        let unit_like_struct = UnitLikeStruct{};
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
