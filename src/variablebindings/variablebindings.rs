fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // Copy 'an_integer' into 'copied_integer'
    let copied_integer = an_integer;

    println!("An Integer: {}", copied_integer);
    println!("A boolean: {}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // The compiler wanrs about unused variable bindings
    // These warnings can be silenced by prefixing the variable name with an underscore
    let _unused_variable = 1u32;

    let noisy_unused_variable = 2u32;
    // Prefix ^ with an underscore to suppress the warning
}