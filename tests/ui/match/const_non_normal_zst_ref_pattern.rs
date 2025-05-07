// check-pass
// ignore-purecap: this type of `transmute()` does not work on CHERI.

const FOO: isize = 10;
const ZST: &() = unsafe { std::mem::transmute(FOO) };
fn main() {
    match &() {
        ZST => 9,
    };
}
