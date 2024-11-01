// revisions: O Os
//[Os] compile-flags: -Copt-level=s
//[O] compile-flags: -O
// ignore-purecap TODO(seharris): optimisations don't happen here that should, but it doesn't look like the generated code is actually broken, just inefficient.
#![crate_type = "lib"]

// CHECK: @func{{2|1}} = {{.*}}alias{{.*}}@func{{1|2}}

#[no_mangle]
pub fn func1(c: char) -> bool {
    c == 's' || c == 'm' || c == 'h' || c == 'd' || c == 'w'
}

#[no_mangle]
pub fn func2(c: char) -> bool {
    matches!(c, 's' | 'm' | 'h' | 'd' | 'w')
}
