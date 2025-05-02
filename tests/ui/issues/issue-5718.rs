// run-pass
// pretty-expanded FIXME #23616
// ignore-purecap: this transmute does not work on CHERI targets, and I
// (seharris) don't want to risk breaking the test trying to avoid it.

struct Element;

macro_rules! foo {
    ($tag: expr, $string: expr) => {
        if $tag == $string {
            let element: Box<_> = Box::new(Element);
            unsafe {
                return std::mem::transmute::<_, usize>(element);
            }
        }
    }
}

fn bar() -> usize {
    foo!("a", "b");
    0
}

fn main() {
    bar();
}
