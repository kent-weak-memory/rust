// run-pass

// Check that memcpy() preserves capabilities.

use std::hint::black_box;

fn small() {
    let values: [u32; 3] = [0, 1, 2];
    let references: [*const u32; 3] = black_box([&values[0], &values[1], &values[2]]);
    let mut copies: [*const u32; 3] = [std::ptr::null(); 3];
    copies.copy_from_slice(&references);
    assert_eq!(copies, references);
    unsafe {
        assert_eq!(*copies[0], values[0]);
        assert_eq!(*copies[1], values[1]);
        assert_eq!(*copies[2], values[2]);
    }
}

fn large() {
    let values: [u32; 3] = [0, 1, 2];
    let references: Vec<&u32> = black_box(values.iter().cycle().take(4096).collect());
    let mut copies: Vec<&u32> = values[2..].iter().cycle().take(4096).collect();
    copies.copy_from_slice(&references);
    for (check, expect) in copies.iter().copied().zip(values.iter().copied().cycle()) {
        let check: &u32 = check;
        let expect: u32 = expect;
        assert_eq!(*check, expect);
    }
    unsafe {
        std::ptr::copy_nonoverlapping(references.as_ptr(), copies.as_mut_ptr(), copies.len());
    }
    for (check, expect) in copies.iter().copied().zip(values.iter().copied().cycle()) {
        let check: &u32 = check;
        let expect: u32 = expect;
        assert_eq!(*check, expect);
    }
}

#[derive(Clone, Copy, Debug)]
struct A<'a> {
    value: u32,
    reference: &'a u32,
}
fn structs() {
    let values: [u32; 3] = [0, 1, 2];
    let references: [A; 3] = black_box([
        A{value: 0, reference: &values[0]},
        A{value: 1, reference: &values[1]},
        A{value: 2, reference: &values[2]},
    ]);
    let mut copies: [A; 3] = [A{value: 123, reference: &values[2]}; 3];
    copies.copy_from_slice(&references);
    for (index, data) in copies.iter().enumerate() {
        assert_eq!(data.value, index as u32);
        assert_eq!(*data.reference, values[index]);
    }
}

pub fn main() {
    small();
    large();
    structs();
}
