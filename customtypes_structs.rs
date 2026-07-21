// An attribute to hide warnings for unused code
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A Unit Struct
struct Unit;

// A Tuple Struct
struct Pair(i32, f32);

// A Struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top-left and bottom-right corners are in space
    top_left: Point,
    bottom_right: Point,
}

