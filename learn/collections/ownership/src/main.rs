#![allow(dead_code)]

/// Removes all the zeros in-place from a vector of integers.
fn remove_zeros(v: &mut Vec<i32>) {
    for i in (0 .. v.len()).rev() {
        if v[i] == 0 {
            v.remove(i);
            v.shrink_to_fit();
        }
    }
}

/// Reverses the elements of a vector in-place
/* In a situation where the borrow checker rejects an operation that is actually
safe and has no workaround, then unsafe code is sometimes acceptable if it's critical
to avoid allocations. In this specific case, you should actually use Vec::swap,
which is internally implemented with heavily-tested unsafe code similar to the
code above. But in general, if the standard library doesn't happen to support your
use case, then unsafe can be acceptable if used correctly. */
fn reverse(v: &mut Vec<String>) {
    let n = v.len();
    for i in 0 .. n/2 {
        let p1 = &mut v[i] as *mut String;
        let p2 = &mut v[n - i - 1] as *mut String;
        unsafe { std::ptr::swap_nonoverlapping(p1, p2, 1); }
    }
}

fn main() {
    println!("Hello, world!");
}
