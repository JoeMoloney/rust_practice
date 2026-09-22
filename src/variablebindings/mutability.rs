fn main() {
    let _immune_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // You cannot assign a new value to an immutable variable
    // _immutable_binding += 1;
}