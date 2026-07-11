use std::fmt;

// A struct holding 2 numbers
// 'Debug' will be derived so the results can be contrasted with 'Display'
#[derive(Debug)]
struct MinMax(i64, i64);

// Implement Display for the MinMax struct so we can print out its values
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Define a struct where the fields are nameable for comparison
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Implement Display for Point2D
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:
    \nDisplay: {}
    \nDebug: {:?}
    ", minmax, minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range
    );

    let point = Point2D {x: 3.3, y: 7.2};

    println!("Compare points:
    \nDisplay: {}
    \nDebug: {:?}
    ", point, point);

}