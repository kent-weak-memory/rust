#![feature(bench_black_box)]
#![deny(usize_as_pointer)]

type WrappedPointer = *const u32;

fn main() {
    let value: usize = 195924;
    let _pointer = value as *const u32; //~ ERROR casting an integer to a pointer will produce an invalid pointer on CHERI
    let _null = 0 as *const u32; // allowed.
    let _singalling = 3 as *const u32; // allowed.
    let _pointer = if std::hint::black_box(false) { value } else { 0 } as WrappedPointer; //~ ERROR casting an integer to a pointer will produce an invalid pointer on CHERI
}
