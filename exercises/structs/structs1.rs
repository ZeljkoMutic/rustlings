// structs1.rs
// Address all the TODOs to make the tests pass!



struct ColorClassicStruct {
    // TODO: Something goes here
    name: String,
    hex: i32,
}

struct ColorTupleStruct(String,i32);

#[derive(Debug)]
struct UnitStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        
        let green = ColorClassicStruct{
            name: "green".to_string(),
            hex: 54,
        };
        
        assert_eq!(green.name, "green");
        assert_eq!(green.hex, 54);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        let green = ColorTupleStruct("green".to_string(),54);
        

        assert_eq!(green.0, "green");
        assert_eq!(green.1, 54);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct!
        let _unit_struct = UnitStruct;
        let message = format!("{:?}s are fun!", _unit_struct);

        assert_eq!(message, "UnitStructs are fun!");
    }
}
