#![allow(unused_variables)]

use std::mem;

pub fn safe(x: &mut i32, y: &mut i32) {} //~ ERROR barrier

fn main() {
    let mut x = 0;
    let xraw: *mut i32 = unsafe { mem::transmute(&mut x) };
    // We need to apply some tricky to be able to call `safe` with two mutable references
    // with the same tag: We transmute both the fn ptr (to take raw ptrs) and the argument
    // (to be raw, but still have the unique tag).
    let safe_raw: fn(x: *mut i32, y: *mut i32) = unsafe {
        mem::transmute::<fn(&mut i32, &mut i32), _>(safe)
    };
    safe_raw(xraw, xraw);
}