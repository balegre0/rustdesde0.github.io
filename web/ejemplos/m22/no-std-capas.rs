// Una biblioteca que funciona sin sistema operativo: solo `core` y `alloc`.
#![no_std]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::Write;

// 1. core: tipos, Option/Result, iteradores, fmt... sin reservar memoria
pub fn maximo(v: &[i32]) -> Option<i32> {
    v.iter().copied().max()
}

// 2. alloc: Vec, String, Box... necesitan un asignador global (que pone quien nos use)
pub fn pares(hasta: u32) -> Vec<u32> {
    (0..hasta).filter(|x| x % 2 == 0).collect()
}

pub fn informe(v: &[i32]) -> String {
    let mut s = String::new();
    // write! funciona sobre cualquier fmt::Write, también sin std
    let _ = write!(s, "{} elementos, máximo {:?}", v.len(), maximo(v));
    s
}

// 3. Lo que NO hay sin std: archivos, hilos, red, println!, HashMap con RandomState...
