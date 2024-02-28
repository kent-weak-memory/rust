// Code generation of atomic operations.
//
// compile-flags: -O
#![crate_type = "lib"]

use std::sync::atomic::{AtomicI32, Ordering::*};

// CHECK-LABEL: @compare_exchange
#[no_mangle]
pub fn compare_exchange(a: &AtomicI32) {
    // NONCHERI: cmpxchg i32* %{{.*}}, i32 0, i32 10 monotonic monotonic
    // CHERI: cmpxchg i32 addrspace(200)* %{{.*}}, i32 0, i32 10 monotonic monotonic
    let _ = a.compare_exchange(0, 10, Relaxed, Relaxed);

    // NONCHERI: cmpxchg i32* %{{.*}}, i32 0, i32 20 release monotonic
    // CHERI: cmpxchg i32 addrspace(200)* %{{.*}}, i32 0, i32 20 release monotonic
    let _ = a.compare_exchange(0, 20, Release, Relaxed);

    // NONCHERI: cmpxchg i32* %{{.*}}, i32 0, i32 30 acquire monotonic
    // CHERI: cmpxchg i32 addrspace(200)* %{{.*}}, i32 0, i32 30 acquire monotonic
    // NONCHERI: cmpxchg i32* %{{.*}}, i32 0, i32 31 acquire acquire
    // CHERI: cmpxchg i32 addrspace(200)* %{{.*}}, i32 0, i32 31 acquire acquire
    let _ = a.compare_exchange(0, 30, Acquire, Relaxed);
    let _ = a.compare_exchange(0, 31, Acquire, Acquire);

    // NONCHERI: cmpxchg i32* %{{.*}}, i32 0, i32 40 acq_rel monotonic
    // CHERI: cmpxchg i32 addrspace(200)* %{{.*}}, i32 0, i32 40 acq_rel monotonic
    // NONCHERI: cmpxchg i32* %{{.*}}, i32 0, i32 41 acq_rel acquire
    // CHERI: cmpxchg i32 addrspace(200)* %{{.*}}, i32 0, i32 41 acq_rel acquire
    let _ = a.compare_exchange(0, 40, AcqRel, Relaxed);
    let _ = a.compare_exchange(0, 41, AcqRel, Acquire);

    // NONCHERI: cmpxchg i32* %{{.*}}, i32 0, i32 50 seq_cst monotonic
    // CHERI: cmpxchg i32 addrspace(200)* %{{.*}}, i32 0, i32 50 seq_cst monotonic
    // NONCHERI: cmpxchg i32* %{{.*}}, i32 0, i32 51 seq_cst acquire
    // CHERI: cmpxchg i32 addrspace(200)* %{{.*}}, i32 0, i32 51 seq_cst acquire
    // NONCHERI: cmpxchg i32* %{{.*}}, i32 0, i32 52 seq_cst seq_cst
    // CHERI: cmpxchg i32 addrspace(200)* %{{.*}}, i32 0, i32 52 seq_cst seq_cst
    let _ = a.compare_exchange(0, 50, SeqCst, Relaxed);
    let _ = a.compare_exchange(0, 51, SeqCst, Acquire);
    let _ = a.compare_exchange(0, 52, SeqCst, SeqCst);
}

// CHECK-LABEL: @compare_exchange_weak
#[no_mangle]
pub fn compare_exchange_weak(w: &AtomicI32) {
    // NONCHERI: cmpxchg weak i32* %{{.*}}, i32 1, i32 10 monotonic monotonic
    // CHERI: cmpxchg weak i32 addrspace(200)* %{{.*}}, i32 1, i32 10 monotonic monotonic
    let _ = w.compare_exchange_weak(1, 10, Relaxed, Relaxed);

    // NONCHERI: cmpxchg weak i32* %{{.*}}, i32 1, i32 20 release monotonic
    // CHERI: cmpxchg weak i32 addrspace(200)* %{{.*}}, i32 1, i32 20 release monotonic
    let _ = w.compare_exchange_weak(1, 20, Release, Relaxed);

    // NONCHERI: cmpxchg weak i32* %{{.*}}, i32 1, i32 30 acquire monotonic
    // CHERI: cmpxchg weak i32 addrspace(200)* %{{.*}}, i32 1, i32 30 acquire monotonic
    // NONCHERI: cmpxchg weak i32* %{{.*}}, i32 1, i32 31 acquire acquire
    // CHERI: cmpxchg weak i32 addrspace(200)* %{{.*}}, i32 1, i32 31 acquire acquire
    let _ = w.compare_exchange_weak(1, 30, Acquire, Relaxed);
    let _ = w.compare_exchange_weak(1, 31, Acquire, Acquire);

    // NONCHERI: cmpxchg weak i32* %{{.*}}, i32 1, i32 40 acq_rel monotonic
    // CHERI: cmpxchg weak i32 addrspace(200)* %{{.*}}, i32 1, i32 40 acq_rel monotonic
    // NONCHERI: cmpxchg weak i32* %{{.*}}, i32 1, i32 41 acq_rel acquire
    // CHERI: cmpxchg weak i32 addrspace(200)* %{{.*}}, i32 1, i32 41 acq_rel acquire
    let _ = w.compare_exchange_weak(1, 40, AcqRel, Relaxed);
    let _ = w.compare_exchange_weak(1, 41, AcqRel, Acquire);

    // NONCHERI: cmpxchg weak i32* %{{.*}}, i32 1, i32 50 seq_cst monotonic
    // CHERI: cmpxchg weak i32 addrspace(200)* %{{.*}}, i32 1, i32 50 seq_cst monotonic
    // NONCHERI: cmpxchg weak i32* %{{.*}}, i32 1, i32 51 seq_cst acquire
    // CHERI: cmpxchg weak i32 addrspace(200)* %{{.*}}, i32 1, i32 51 seq_cst acquire
    // NONCHERI: cmpxchg weak i32* %{{.*}}, i32 1, i32 52 seq_cst seq_cst
    // CHERI: cmpxchg weak i32 addrspace(200)* %{{.*}}, i32 1, i32 52 seq_cst seq_cst
    let _ = w.compare_exchange_weak(1, 50, SeqCst, Relaxed);
    let _ = w.compare_exchange_weak(1, 51, SeqCst, Acquire);
    let _ = w.compare_exchange_weak(1, 52, SeqCst, SeqCst);
}
