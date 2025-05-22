// run-pass
// ignore-emscripten no threads
// ignore-aarch64-unknown-freebsd-purecap TODO(seharris): ideally fix this, but it seems unlikely this will be a problem in real programs.
// This is broken on Morello, likely due to an LLVM optimisation pass not
// behaving the same as other targets.
// It seems to work on for `vec![0 as u8; LEN]` and `Box::new([0 as u8; LEN])`,
// but using `std::mem::zeroed()` causes a stack allocation, which then causes
// a stack overflow.
// Given that this seems an unusual thing to do, we're choosing just to note
// the issues and ignore it for the moment.
// compile-flags: -O

// Tests that the `vec!` macro does not overflow the stack when it is
// given data larger than the stack.

// FIXME(eddyb) Improve unoptimized codegen to avoid the temporary,
// and thus run successfully even when compiled at -C opt-level=0.

// Setting this to lower values causes `SIGILL` on Morello, we aren't sure how.
// ...just don't do that I guess?
const LEN: usize = 1 << 16;

use std::thread::Builder;

fn main() {
    assert!(Builder::new().stack_size(LEN / 2).spawn(|| {
        // FIXME(eddyb) this can be vec![[0: LEN]] pending
        // https://llvm.org/bugs/show_bug.cgi?id=28987
        let vec = vec![unsafe { std::mem::zeroed::<[u8; LEN]>() }];
        assert_eq!(vec.len(), 1);
    }).unwrap().join().is_ok());
}
