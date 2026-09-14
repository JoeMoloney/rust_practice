// Create enum to classify a web event.
// Enum variants can be either:
enum WebEvent {
    // unit-like
    PageLoad,
    PageUnload,
    // tuple structs
    KeyPress(char),
    Paste(String),
    // or c-like structures
    Click { x: i64, y: i64 },
}

// Function that takes a WebEvent enum
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),

        // Destructure 'c' from inside the enum event:
        WebEvent::KeyPress(c) => println!("pressed '{}'", c),
        WebEvent::Paste(s) => println!("pasted \"{}\"", s),

        // Destructure 'Click into 'x' and 'y'
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}", x, y);
        },
    }
}

fn type_inspect(alias: VeryVerboseEnumOfThingsToDoWithNumbers) {
    match alias {
        VeryVerboseEnumOfThingsToDoWithNumbers::Add => println!("Add Enum"),
        VeryVerboseEnumOfThingsToDoWithNumbers::Subtract => println!("Subtract Enum"),
    }
}

// Type Aliases
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add, Subtract,
}
impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x -y,
        }
    }
}

// Create a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
    let pressed = WebEvent::KeyPress('x');

    // to_owned() creates an owned String from a string slice
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x:20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let x_add = Operations::Add;
    let x_subtract = Operations::Subtract;
    type_inspect(x_add);
    type_inspect(x_subtract);

    println!("{}", VeryVerboseEnumOfThingsToDoWithNumbers::Add.run(10, 20));
    println!("{}", VeryVerboseEnumOfThingsToDoWithNumbers::Subtract.run(100, 50));
}