use std::mem;

// Make sure we error with the right kind of error on a too large slice.
const TEST: () = { unsafe {
    let slice: *const [u8] = mem::transmute((1usize as *const u8, usize::MAX as *const u8));
    let _val = &*slice; //~ ERROR: evaluation of constant value failed
    //~| slice is bigger than largest supported object
} };

fn main() {}
