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
    let ys: [i32; 500] = [0; 500];

    println!("First element of the XS array: {}", xs[0]);
    println!("Second element of the XS array: {}", xs[1]);

    // 'len' returns the count of elements in an array
    println!("Number of elements in array: {}", xs.len());

    // Arrays are stack allocated (as opposed to the heap)
    println!("XS Array occupies {} bytes", mem::size_of_val(&xs));
    // Note: The stack is very fast and is where memory is allocated by default in Rust. The allocation is local to a function call however and is limited in size
    // The heap on the other hand is slower and is explicitally allocated by your program but is effectively unlimited in size and is globally accessible

    //Arrays can be borrowed as slices
    println!("Borrow the whole array as a slice.");
    analyse_slice(&xs);

    // Slices can point to a section of an array
    // They are of the form [starting_index..ending_index]
    // 'starting_index' is the first position in the slice
    // 'ending_index' is one more than the last position in the slice
    println!("Borrow a section of the array as a slice");
    analyse_slice(&ys[1 .. 4]);

    // Example of empty slice '&[]':
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // Same but more verbose

    // Arrays can be safely accessed using '.get', which returns an 'Option'.
    // This can be matched as shown below, or used with '.expect()' if you would like the program to exit with a nice message instead of happily continue
    for i in 0..xs.len() + 1 { // One element too
        match xs.get(i) {
            Some(xval) => println!("i={}: xs={}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

    // Out of bound indexing on array with constant value causes compile time error
    // println!("{}", xs[5]);
    
    // Out of bound indexing on slice causes runtime error
    // println!("{}", xs[..][5]);
}