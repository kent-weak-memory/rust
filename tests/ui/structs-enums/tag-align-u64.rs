// run-pass
#![allow(dead_code)]
#![allow(deprecated)]

enum Tag {
    TagInner(u64)
}

struct Rec {
    c8: u8,
    t: Tag
}

fn mk_rec() -> Rec {
    return Rec { c8:0, t:Tag::TagInner(0) };
}

fn is_u64_aligned(u: &Tag) -> bool {
    let p: usize = u as *const Tag as usize;
    let u64_align = std::mem::min_align_of::<u64>();
    return (p & (u64_align - 1)) == 0;
}

pub fn main() {
    let x = mk_rec();
    assert!(is_u64_aligned(&x.t));
}
