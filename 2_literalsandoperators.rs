fn main() {
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2); // Must be signed due to result being negative

    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // Short circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is {:x}", 0x80u32 >> 2);

    // Use underscores to improve readability
    println!("One million is written as {}", 1_000_000u32);
}

/*
    NOTES:
        Literals:
            Integers:
                1
            Floats:
                1.2
            Characters:
                'a'
            Strings:
                "abc"
            Booleans:
                true
            Unit Type:
                ()
            Can be expressed using literals

            Alternatively, integers can be expressed using the following prefixes:
                Hexadecimal:
                    0x
                Octal:
                    0o
                Binary:
                    0b
            
            Underscores can be inserted into numeric literals to improve readability:
                1_000 is the same as 1000
                0.000_001 is the same as 0.000001

            Rust also supports E-notation:
                1e6
                7.6e-4
                    associated type is f64
*/