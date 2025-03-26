// run-pass

#![allow(non_camel_case_types)]
// Binop corner cases

fn test_nil() {
    assert_eq!((), ());
    assert!((!(() != ())));
    assert!((!(() < ())));
    assert!((() <= ()));
    assert!((!(() > ())));
    assert!((() >= ()));
}

fn test_bool() {
    assert!((!(true < false)));
    assert!((!(true <= false)));
    assert!((true > false));
    assert!((true >= false));

    assert!((false < true));
    assert!((false <= true));
    assert!((!(false > true)));
    assert!((!(false >= true)));

    // Bools support bitwise binops
    assert_eq!(false & false, false);
    assert_eq!(true & false, false);
    assert_eq!(true & true, true);
    assert_eq!(false | false, false);
    assert_eq!(true | false, true);
    assert_eq!(true | true, true);
    assert_eq!(false ^ false, false);
    assert_eq!(true ^ false, true);
    assert_eq!(true ^ true, false);
}

fn test_ptr() {
    let p1: *const u8 = 0_usize as *const u8;
    let p2: *const u8 = 0_usize as *const u8;
    let p3: *const u8 = 1_usize as *const u8;

    assert_eq!(p1, p2);
    assert!(p1 != p3);
    assert!(p1 < p3);
    assert!(p1 <= p3);
    assert!(p3 > p1);
    assert!(p3 >= p3);
    assert!(p1 <= p2);
    assert!(p1 >= p2);
}

#[derive(PartialEq, Debug)]
struct p {
  x: isize,
  y: isize,
}

fn p(x: isize, y: isize) -> p {
    p {
        x: x,
        y: y
    }
}

fn test_class() {
  let q = p(1, 2);
  let mut r = p(1, 2);

  println!("q = {:x}, r = {:x}", (&q) as *const p as usize, (&r) as *const p as usize);
  assert_eq!(q, r);
  r.y = 17;
  assert!((r.y != q.y));
  assert_eq!(r.y, 17);
  assert!((q != r));
}

pub fn main() {
    test_nil();
    test_bool();
    test_ptr();
    test_class();
}
