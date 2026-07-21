
// An attribute to hide warnings for unused code
#![allow(dead_code)]
use std::fmt;

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
#[derive(Clone)]
struct Point {
    x: f32,
    y: f32,
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point: ({}, {})", self.x, self.y)
    }
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top-left and bottom-right corners are in space
    top_left: Point,
    bottom_right: Point,
}
impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rectangle: ({}, {})", self.top_left, self.bottom_right)
    }
}

// Activity 1/2
fn rect_area(Rectangle {
    top_left: Point { x: tlx, y: tly },
    bottom_right: Point { x: brx, y: bry },
}: &Rectangle,) -> f32 {
    (tlx - brx) * (tly - bry)
}

// Activity 2/2
fn square(point: Point, width_height: f32) -> Rectangle {
    Rectangle { 
        top_left: point,
        bottom_right: Point {
            x: width_height,
            y: width_height,
        },
    }
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a 'Point'
    let point: Point = Point { x: 5.2, y: 0.4 };
    let another_point: Point = Point { x: 10.3, y: 0.2 };

    // Access fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our other one
    let bottom_right = Point { x: 10.3, ..another_point };

    // 'bottom_right.y' will be the same as 'another_point.y' because we used that field from 'another_point'
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a 'let' binding
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // Struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    // Activity 1/2
    let activity_point1 = Point { x: 0.0, y: 0.0 };

    let activity_point2 = Point { x: 5.0, y: 15.0 };
    
    let activity_rectangle: Rectangle = Rectangle { top_left: activity_point1.clone(), bottom_right: activity_point2 };
    
    println!("rect_area = {}", rect_area(&activity_rectangle));

    // Activity 2/2
    println!("Activity 2: Return Rectangle: {}", square(activity_point1, 20.0));
}