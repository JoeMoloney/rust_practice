fn main() {
    // Variable type annotation
    let logical: bool = true;

    /* 
        Numbers can be additionally annotated via a suffix or by default
        Integers default to i32 and floats to f64
        Rust can infer types based on context also
    */ 
    
    // Annotated
    let a_float: f64 = 1.0; // Regular Annotation
    let an_integer = 5i32; // Suffix Annotation

    let another_float = 62.5f64;
    let another_integer: i32 = 101;

    // Defaulted
    let default_float = 3.0;
    let default_integer = 7;

    // Type (inferred from context)
    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 4294967296i64;

    // Mutable variables values can be changed
    let mut mutable = 12; // Mutable i32
    mutable = 21;

    // The type of variable CANNOT be changed
    // mutable = true;

    // Variables can be overwritten with shadowing
    let mutable = true;

    /*
        Compound Types:
            Array
            Tuple
    */

    // Array signature consists of Type T and length as [T; length]
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];

    // Tuple is a collection of values of different types
    // and is constructed using parantheses ()
    let my_tuple = (5u32, 1u8, true, -5.04f32);
}

/*
    NOTES:
        variable shadowing:
            https://en.wikipedia.org/wiki/Variable_shadowing
            Variable Shadowing occurs when a variable declared within' a certain scope
                (decision block / method / inner class)
            has the same name as a variable declared in an outer scope.
        
        types (Scalar Types):
            Signed Integers:
                i8, i16, i32, i64, i128 & isize (pointer size)
            
            Unsigned Integers:
                u8, u16, u32, u64, u128 & usize (pointer size)
            
            Signed/Unsigned refer to whether its possible for a number to be negative
            Signed = number needs to have a sign with it
            Unsigned = number does not need to have a sign with it
                And will only ever be positive

            Floating Point:
                f32, f64
            
            Unicode Scalar Values: ('a', 'α' and '∞' (4 bytes each))
                char
            
            Boolean:
                true, false
            
            Unit Type: (only possible value is an empty tuple)
                ()
                Despite the value of a unit type being a tuple,
                it is not considered a compound type because it does not contain multiple values
            
            Compound Type:
                Tuples, Arrays
*/