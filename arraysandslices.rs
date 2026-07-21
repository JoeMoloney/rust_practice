use std::mem;

// Notes:
// Slices are similar to arrays except their length is not known at compile time
// A slice is a two-word object, the first word is a pointer to the data & the second word is the length of the slice
// The word size is the same as a usize, determined by the processor architecture (e.g. 64bits on x86-x64)
// Slices can be used to borrow a section of an array and have the type signature &[T]

// The following function borrows a slice
fn analyse_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main() {
    // Fixed-size array (type signature is superflous (more than enough / extra / unnecessary))
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialised to the same value
    let ys: [i32, 500] = [0; 500];

    println!("First element of the XS array: {}", xs[0]);
    println!("Second element of the XS array: {}", xs[1]);

    // 'len' returns the count of elements in an array
    println!("Number of elements in array: {}", xs.len());

    // Arrays are stack allocated (as opposed to the heap)
    // Note: The stack is very fast and is where memory is allocated by default in Rust. The allocation is local to a function call however and is limited in size
    // The heap on the other hand is slower and is explicitally allocated by your program but is effectively unlimited in size and is globally accessible
    println!("Array occupies {} bytes", mem::size_of_val(&xs));
}