fn main() {
    // When data is bound by the same name immutably, 
    // it also freezes. Frozen data can’t be modified 
    // until the immutable binding goes out of scope:

    let mut _mutable_integer = 7i32;

    {
        //Shadowing immutable
        let _mutable_integer = _mutable_integer;

        // _mutable_integer = 50; // frozen in this scope
    }

    _mutable_integer = 3; // not frozen in this scope
}