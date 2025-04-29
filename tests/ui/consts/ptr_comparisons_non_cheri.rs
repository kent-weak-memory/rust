// compile-flags: --crate-type=lib
// normalize-stderr-32bit: "8 bytes" -> "$$TWO_WORDS bytes"
// normalize-stderr-64bit: "16 bytes" -> "$$TWO_WORDS bytes"
// normalize-stderr-32bit: "size 4" -> "size $$WORD"
// normalize-stderr-64bit: "size 8" -> "size $$WORD"
// ignore-purecap

const FOO: &usize = &42;

const _: usize = unsafe { std::mem::transmute::<*const usize, usize>(FOO) + 4 };
//~^ ERROR evaluation of constant value failed
//~| unable to turn pointer into raw bytes

const _: usize = unsafe { *std::mem::transmute::<&&usize, &usize>(&FOO) + 4 };
//~^ ERROR evaluation of constant value failed
//~| unable to turn pointer into raw bytes
