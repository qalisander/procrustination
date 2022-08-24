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

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{hint, thread};
use std::fmt::Write;
use std::simd::SimdElement;
use std::slice::ArrayChunks;
use rand::distributions::Slice;

fn main() {
    let x = 1_f32;
    let mut speech = "\"Ouch!\" said the well.\n";
//    let s: &[u8] = speech.as_ref();
    let s: &[] = speech.as_ref();

}

fn binary_search<T: PartialOrd>(arr: &[T], elem: &T) -> Option<usize>{
    unimplemented!()
}


//fn binary_search<T: PartialOrd>(arr: &[T], elem: &T) -> Option<usize>{
//    unimplemented!()
//}

fn f() {
    let s = String::from("abcd");

    for _ in 0..10 {
        std::thread::spawn(|| {
            // Do something with the String `s` here
            println!("{}", &s);
        });
    }
}