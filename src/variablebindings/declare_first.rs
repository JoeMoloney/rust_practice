fn main() {

    let a_binding;

    {
        let x:i8 = 2;

        a_binding = x * x;
    }

    println!("a_binding: {}", a_binding);

    // Declare first, initialize later
    // must be initialized before use... standard stuff...

    let another_binding:i8;

    // println!("another_binding: {}", another_binding);

    another_binding = 1;

    println!("another_binding: {}", another_binding);
}