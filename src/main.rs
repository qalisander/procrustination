use bit_set::BitSet;
use bit_vec::BitVec;
use core::iter;
use counter::Counter;
use itertools::{Itertools, PeekingNext};
use num::Complex;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::ops::Index;

//use rust_sandbox::plants_and_zombies::example_tests;
//
//fn main() {
//    example_tests::tests()
//}

fn main() {
    let mut num = 5;
    let mut num2 = 100;

    let r1 = &mut num as *mut i32;
    let mut r2 = r1 as *const i32;

    // Safe because r1 and r2 were obtained from references and so are
    // guaranteed to be non-null and properly aligned, the objects underlying
    // the references from which they were obtained are live throughout the
    // whole unsafe block, and they are not accessed either through the
    // references or concurrently through any other pointers.
    unsafe {
        println!("r1 is: {}", *r1);
        *r1 = 10;
        println!("r2 is: {}", *r2);
        r2 = num2 as *const i32;

    }
}