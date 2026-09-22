fn main() {
    // Lives in the main function
    let long_lived_binding: i32 = 1;

    // Block has smaller scope than main function
    {
        // Only exists in this blocks scope
        let short_lived_binding: i32 = 2;

        println!("inner short: {}", short_lived_binding);
    }

    // short_lived_binding does not exist in this scope
    // println!("outer short: {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding);

    let shadowed_binding: i32 = 1;

    {
        println!("before being shadowed: {}", shadowed_binding);

        // Shadows the outer one
        let shadowed_binding: &str = "abc";

        println!("shadowed inner block: {}", shadowed_binding);
    }
    println!("outside inner block: {}", shadowed_binding);

    // Shadows the previous binding
    let shadowed_binding: i32 = 2;
    println!("shadowed in outer block: {}", shadowed_binding);
}